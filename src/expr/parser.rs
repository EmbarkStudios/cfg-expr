use crate::{
    error::{ParseError, Reason},
    expr::{
        lexer::{Lexer, Token},
        ExprNode, Expression, InnerPredicate, Operator, TargetPredicate,
    },
};
use smallvec::SmallVec;

impl Expression {
    pub fn parse(original: &str) -> Result<Self, ParseError<'_>> {
        let lexer = Lexer::new(original);

        // The lexer automatically trims any cfg( ), so reacquire
        // the string before we start walking tokens
        let original = lexer.inner;

        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
        enum Func {
            All,
            Any,
            Not,
        }

        struct FuncAndSpan {
            func: Func,
            parens_index: usize,
            span: std::ops::Range<usize>,
            predicates: SmallVec<[InnerPredicate; 5]>,
            nest_level: u8,
        }

        let mut func_stack = SmallVec::<[FuncAndSpan; 5]>::new();
        let mut expr_queue = SmallVec::<[ExprNode; 5]>::new();

        // Keep track of the last token to simplify validation of the token stream
        let mut last_token: Option<Token<'_>> = None;

        let parse_predicate = |key: (&str, std::ops::Range<usize>),
                               val: Option<(&str, std::ops::Range<usize>)>|
         -> Result<InnerPredicate, ParseError<'_>> {
            // Warning: It is possible for arbitrarily-set configuration
            // options to have the same value as compiler-set configuration
            // options. For example, it is possible to do rustc --cfg "unix" program.rs
            // while compiling to a Windows target, and have both unix and windows
            // configuration options set at the same time. It is unwise to actually
            // do this.
            //
            // rustc is very permissive in this regard, but I'd rather be really
            // strict, as it's much easier to loosen restrictions over time than add
            // new ones
            macro_rules! err_if_val {
                () => {
                    if let Some((_, vspan)) = val {
                        return Err(ParseError {
                            original,
                            span: vspan,
                            reason: Reason::Unexpected(&[]),
                        });
                    }
                };
            }

            let span = key.1;
            let key = key.0;

            Ok(match key {
                // These are special cases in the cfg language that are
                // semantically the same as `target_family = "<family>"`,
                // so we just make them not special
                "unix" | "windows" => {
                    err_if_val!();

                    let fam = key.parse().map_err(|reason| ParseError {
                        original,
                        span,
                        reason,
                    })?;

                    InnerPredicate::Target(TargetPredicate::Family(Some(fam)))
                }
                "test" => {
                    err_if_val!();
                    InnerPredicate::Test
                }
                "debug_assertions" => {
                    err_if_val!();
                    InnerPredicate::DebugAssertions
                }
                "proc_macro" => {
                    err_if_val!();
                    InnerPredicate::ProcMacro
                }
                "feature" => {
                    // rustc allows bare feature without a value, but the only way
                    // such a predicate would ever evaluate to true would be if they
                    // explicitly set --cfg feature, which would be terrible, so we
                    // just error instead
                    match val {
                        Some((_, span)) => InnerPredicate::Feature(span),
                        None => {
                            return Err(ParseError {
                                original,
                                span,
                                reason: Reason::Unexpected(&["= \"<feature_name>\""]),
                            });
                        }
                    }
                }
                target_key if key.starts_with("target_") => {
                    let (val, vspan) = match val {
                        None => {
                            return Err(ParseError {
                                original,
                                span,
                                reason: Reason::Unexpected(&["= \"<target_cfg_value>\""]),
                            });
                        }
                        Some((val, vspan)) => (val, vspan),
                    };

                    macro_rules! tp {
                        ($which:ident) => {
                            TargetPredicate::$which(val.parse().map_err(|r| ParseError {
                                original,
                                span: vspan,
                                reason: r,
                            })?)
                        };

                        (opt $which:ident) => {
                            if !val.is_empty() {
                                TargetPredicate::$which(Some(val.parse().map_err(|r| {
                                    ParseError {
                                        original,
                                        span: vspan,
                                        reason: r,
                                    }
                                })?))
                            } else {
                                TargetPredicate::$which(None)
                            }
                        };
                    }

                    let tp = match &target_key[7..] {
                        "arch" => tp!(Arch),
                        "feature" => {
                            if val.is_empty() {
                                return Err(ParseError {
                                    original,
                                    span: vspan,
                                    reason: Reason::Unexpected(&["<feature>"]),
                                });
                            }

                            return Ok(InnerPredicate::TargetFeature(vspan));
                        }
                        "os" => tp!(opt Os),
                        "family" => tp!(opt Family),
                        "env" => tp!(opt Env),
                        "endian" => tp!(Endian),
                        "pointer_width" => {
                            TargetPredicate::PointerWidth(val.parse().map_err(|_| ParseError {
                                original,
                                span: vspan,
                                reason: Reason::InvalidInteger,
                            })?)
                        }
                        "vendor" => tp!(opt Vendor),
                        _ => {
                            return Err(ParseError {
                                original,
                                span,
                                reason: Reason::Unexpected(&[
                                    "target_arch",
                                    "target_feature",
                                    "target_os",
                                    "target_family",
                                    "target_env",
                                    "target_endian",
                                    "target_pointer_width",
                                    "target_vendor",
                                ]),
                            })
                        }
                    };

                    InnerPredicate::Target(tp)
                }
                _other => InnerPredicate::Other {
                    identifier: span,
                    value: val.map(|(_, span)| span),
                },
            })
        };

        macro_rules! token_err {
            ($span:expr) => {{
                let expected: &[&str] = match last_token {
                    None => &["<key>", "all", "any", "not"],
                    Some(Token::All) | Some(Token::Any) | Some(Token::Not) => &["("],
                    Some(Token::CloseParen) => &[")", ","],
                    Some(Token::Comma) => &[")", "<key>"],
                    Some(Token::Equals) => &["\""],
                    Some(Token::Key(_)) => &["=", ",", ")"],
                    Some(Token::Value(_)) => &[",", ")"],
                    Some(Token::OpenParen) => &["<key>", ")", "all", "any", "not"],
                };

                return Err(ParseError {
                    original,
                    span: $span,
                    reason: Reason::Unexpected(&expected),
                });
            }};
        }

        let mut pred_key: Option<(&str, _)> = None;
        let mut pred_val: Option<(&str, _)> = None;

        let mut root_predicate_count = 0;

        // Basic implementation of the https://en.wikipedia.org/wiki/Shunting-yard_algorithm
        'outer: for lt in lexer {
            let lt = lt?;
            match &lt.token {
                Token::Key(k) => match last_token {
                    None | Some(Token::OpenParen) | Some(Token::Comma) => {
                        pred_key = Some((k, lt.span.clone()));
                    }
                    _ => token_err!(lt.span),
                },
                Token::Value(v) => match last_token {
                    Some(Token::Equals) => {
                        // We only record the span for keys and values
                        // so that the expression doesn't need a lifetime
                        // but in the value case we need to strip off
                        // the quotes so that the proper raw string is
                        // provided to callers when evaluating the expression
                        pred_val = Some((v, lt.span.start + 1..lt.span.end - 1));
                    }
                    _ => token_err!(lt.span),
                },
                Token::Equals => match last_token {
                    Some(Token::Key(_)) => {}
                    _ => token_err!(lt.span),
                },
                Token::All | Token::Any | Token::Not => match last_token {
                    None | Some(Token::OpenParen) | Some(Token::Comma) => {
                        let new_op = match lt.token {
                            Token::All => Func::All,
                            Token::Any => Func::Any,
                            Token::Not => Func::Not,
                            _ => unreachable!(),
                        };

                        if let Some(fs) = func_stack.last_mut() {
                            fs.nest_level += 1
                        }

                        func_stack.push(FuncAndSpan {
                            func: new_op,
                            span: lt.span,
                            parens_index: 0,
                            predicates: SmallVec::new(),
                            nest_level: 0,
                        });
                    }
                    _ => token_err!(lt.span),
                },
                Token::OpenParen => match last_token {
                    Some(Token::All) | Some(Token::Any) | Some(Token::Not) => {
                        if let Some(ref mut fs) = func_stack.last_mut() {
                            fs.parens_index = lt.span.start;
                        }
                    }
                    _ => token_err!(lt.span),
                },
                Token::CloseParen => match last_token {
                    None | Some(Token::All) | Some(Token::Any) | Some(Token::Not)
                    | Some(Token::Equals) => token_err!(lt.span),
                    _ => {
                        if let Some(top) = func_stack.pop() {
                            let key = pred_key.take();
                            let val = pred_val.take();

                            let func = match top.func {
                                Func::All => Operator::All,
                                Func::Any => Operator::Any,
                                Func::Not => {
                                    // not() doesn't take a predicate list, but only a single predicate,
                                    // so ensure we have exactly 1
                                    let num_predicates = top.predicates.len() as u8
                                        + if key.is_some() { 1 } else { 0 }
                                        + top.nest_level;

                                    if num_predicates != 1 {
                                        return Err(ParseError {
                                            original,
                                            span: top.span.start..lt.span.end,
                                            reason: Reason::InvalidNot(num_predicates),
                                        });
                                    }

                                    Operator::Not
                                }
                            };

                            for pred in top.predicates {
                                expr_queue.push(ExprNode::Predicate(pred));
                            }

                            if let Some(key) = key {
                                let inner_pred = parse_predicate(key, val)?;
                                expr_queue.push(ExprNode::Predicate(inner_pred));
                            }

                            expr_queue.push(ExprNode::Op(func));

                            // This is the only place we go back to the top of the outer loop,
                            // so make sure we correctly record this token
                            last_token = Some(Token::CloseParen);
                            continue 'outer;
                        }

                        // We didn't have an opening parentheses if we get here
                        return Err(ParseError {
                            original,
                            span: lt.span,
                            reason: Reason::UnopenedParens,
                        });
                    }
                },
                Token::Comma => match last_token {
                    None
                    | Some(Token::OpenParen)
                    | Some(Token::All)
                    | Some(Token::Any)
                    | Some(Token::Not)
                    | Some(Token::Equals) => token_err!(lt.span),
                    _ => {
                        let key = pred_key.take();
                        let val = pred_val.take();

                        let inner_pred = if let Some(key) = key {
                            Some(parse_predicate(key, val)?)
                        } else {
                            None
                        };

                        match (inner_pred, func_stack.last_mut()) {
                            (Some(pred), Some(func)) => {
                                func.predicates.push(pred);
                            }
                            (Some(pred), None) => {
                                root_predicate_count += 1;

                                expr_queue.push(ExprNode::Predicate(pred));
                            }
                            _ => {}
                        }
                    }
                },
            }

            last_token = Some(lt.token);
        }

        if let Some(Token::Equals) = last_token {
            return Err(ParseError {
                original,
                span: original.len()..original.len(),
                reason: Reason::Unexpected(&["\"<value>\""]),
            });
        }

        // If we still have functions on the stack, it means we have an unclosed parens
        match func_stack.pop() {
            Some(top) => {
                if top.parens_index != 0 {
                    Err(ParseError {
                        original,
                        span: top.parens_index..original.len(),
                        reason: Reason::UnclosedParens,
                    })
                } else {
                    Err(ParseError {
                        original,
                        span: top.span,
                        reason: Reason::Unexpected(&["("]),
                    })
                }
            }
            None => {
                let key = pred_key.take();
                let val = pred_val.take();

                if let Some(key) = key {
                    root_predicate_count += 1;
                    expr_queue.push(ExprNode::Predicate(parse_predicate(key, val)?));
                }

                if expr_queue.is_empty() {
                    Err(ParseError {
                        original,
                        span: 0..original.len(),
                        reason: Reason::Empty,
                    })
                } else if root_predicate_count > 1 {
                    Err(ParseError {
                        original,
                        span: 0..original.len(),
                        reason: Reason::MultipleRootPredicates,
                    })
                } else {
                    Ok(Expression {
                        original: original.to_owned(),
                        expr: expr_queue,
                    })
                }
            }
        }
    }
}
