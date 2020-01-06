pub mod lexer;
mod parser;

use smallvec::SmallVec;

/// The predicate operator
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Operator {
    Not,
    All,
    Any,
}

use crate::targets as targ;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TargetPredicate {
    Arch(targ::Arch),
    Os(Option<targ::Os>),
    Family(Option<targ::Family>),
    Env(Option<targ::Env>),
    Endian(targ::Endianness),
    Vendor(Option<targ::Vendor>),
    PointerWidth(u8),
}

impl TargetPredicate {
    pub fn matches(self, target: &targ::TargetInfo) -> bool {
        use TargetPredicate::*;

        match self {
            Arch(a) => a == target.arch,
            Os(os) => os == target.os,
            Family(fam) => fam == target.family,
            Env(env) => env == target.env,
            Endian(end) => end == target.endian,
            Vendor(ven) => ven == target.vendor,
            PointerWidth(w) => w == target.pointer_width,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Predicate<'a> {
    Target(TargetPredicate),
    Test,
    DebugAssertions,
    ProcMacro,
    Feature(&'a str),
    TargetFeature(&'a str),
    Flag(&'a str),
    KeyValue { key: &'a str, val: &'a str },
}

#[derive(Clone, PartialEq, Debug)]
pub enum InnerPredicate {
    Target(TargetPredicate),
    Test,
    DebugAssertions,
    ProcMacro,
    Feature(std::ops::Range<usize>),
    TargetFeature(std::ops::Range<usize>),
    Other {
        identifier: std::ops::Range<usize>,
        value: Option<std::ops::Range<usize>>,
    },
}

impl InnerPredicate {
    fn to_pred<'a>(&self, s: &'a str) -> Predicate<'a> {
        use InnerPredicate as IP;
        use Predicate::*;

        match self {
            IP::Target(tp) => Target(*tp),
            IP::Test => Test,
            IP::DebugAssertions => DebugAssertions,
            IP::ProcMacro => ProcMacro,
            IP::Feature(rng) => Feature(&s[rng.clone()]),
            IP::TargetFeature(rng) => TargetFeature(&s[rng.clone()]),
            IP::Other { identifier, value } => match value {
                Some(vs) => KeyValue {
                    key: &s[identifier.clone()],
                    val: &s[vs.clone()],
                },
                None => Flag(&s[identifier.clone()]),
            },
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) enum ExprNode {
    Op(Operator),
    Predicate(InnerPredicate),
}

#[derive(Debug)]
pub struct Expression {
    pub(crate) expr: SmallVec<[ExprNode; 5]>,
    // We keep the original string around for display purposes only
    pub(crate) original: String,
}

impl Expression {
    pub fn predicates(&self) -> impl Iterator<Item = Predicate<'_>> {
        self.expr.iter().filter_map(move |item| match item {
            ExprNode::Predicate(pred) => {
                let pred = pred.clone().to_pred(&self.original);
                Some(pred)
            }
            _ => None,
        })
    }

    pub fn eval<EP: FnMut(&Predicate<'_>) -> bool>(&self, mut eval_predicate: EP) -> bool {
        let mut result_stack = SmallVec::<[bool; 8]>::new();

        // We store the expression as postfix, so just evaluate each license
        // requirement in the order it comes, and then combining the previous
        // results according to each operator as it comes
        for node in self.expr.iter() {
            match node {
                ExprNode::Predicate(pred) => {
                    let pred = pred.to_pred(&self.original);
                    result_stack.push(eval_predicate(&pred));
                }
                ExprNode::Op(Operator::All) => {
                    // all() with a comma separated list of configuration predicates.
                    // It is false if at least one predicate is false.
                    // If there are no predicates, it is true.
                    let mut result = true;

                    while let Some(rs) = result_stack.pop() {
                        result = result && rs;
                    }

                    result_stack.push(result);
                }
                ExprNode::Op(Operator::Any) => {
                    // any() with a comma separated list of configuration predicates.
                    // It is true if at least one predicate is true.
                    // If there are no predicates, it is false.
                    let mut result = false;

                    while let Some(rs) = result_stack.pop() {
                        result = result || rs;
                    }

                    result_stack.push(result);
                }
                ExprNode::Op(Operator::Not) => {
                    // not() with a configuration predicate.
                    // It is true if its predicate is false
                    // and false if its predicate is true.
                    let r = result_stack.pop().unwrap();
                    result_stack.push(!r);
                }
            }
        }

        result_stack.pop().unwrap()
    }
}

// impl fmt::Debug for Expression {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Expression")
//             .field("original", &self.original)
//             .field(
//         for (i, node) in self.expr.iter().enumerate() {
//             if i > 0 {
//                 f.write_str(" ")?;
//             }

//             match node {
//                 ExprNode::Req(req) => write!(f, "{}", req.req)?,
//                 ExprNode::Op(Operator::And) => f.write_str("AND")?,
//                 ExprNode::Op(Operator::Or) => f.write_str("OR")?,
//             }
//         }

//         Ok(())
//     }
// }
