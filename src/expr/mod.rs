pub mod lexer;
mod parser;

use smallvec::SmallVec;

/// A predicate function, used to combine 1 or more predicates
/// into a single value
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Func {
    /// `not()` with a configuration predicate. It is true if its predicate
    /// is false and false if its predicate is true.
    Not,
    /// `all()` with a comma separated list of configuration predicates. It
    /// is false if at least one predicate is false. If there are no predicates,
    /// it is true.
    ///
    /// The associated `usize` is the number of predicates inside the `all()`.
    All(usize),
    /// `any()` with a comma separated list of configuration predicates. It
    /// is true if at least one predicate is true. If there are no predicates,
    /// it is false.
    ///
    /// The associated `usize` is the number of predicates inside the `any()`.
    Any(usize),
}

use crate::targets as targ;

/// All predicates that pertains to a target, except for `target_feature`
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TargetPredicate {
    /// [target_arch](https://doc.rust-lang.org/reference/conditional-compilation.html#target_arch)
    Arch(targ::Arch),
    /// [target_endian](https://doc.rust-lang.org/reference/conditional-compilation.html#target_endian)
    Endian(targ::Endian),
    /// [target_env](https://doc.rust-lang.org/reference/conditional-compilation.html#target_env)
    Env(Option<targ::Env>),
    /// [target_family](https://doc.rust-lang.org/reference/conditional-compilation.html#target_family)
    /// This also applies to the bare [`unix` and `windows`](https://doc.rust-lang.org/reference/conditional-compilation.html#unix-and-windows)
    /// predicates.
    Family(Option<targ::Family>),
    /// [target_os](https://doc.rust-lang.org/reference/conditional-compilation.html#target_os)
    Os(Option<targ::Os>),
    /// [target_pointer_width](https://doc.rust-lang.org/reference/conditional-compilation.html#target_pointer_width)
    PointerWidth(u8),
    /// [target_vendor](https://doc.rust-lang.org/reference/conditional-compilation.html#target_vendor)
    Vendor(Option<targ::Vendor>),
}

impl TargetPredicate {
    /// Returns true of the predicate matches the specified target
    ///
    /// ```
    /// use cfg_expr::{targets::*, expr::TargetPredicate as tp};
    /// let win = get_target_by_triple("x86_64-pc-windows-msvc").unwrap();
    ///
    /// assert!(
    ///     tp::Arch(Arch::x86_64).matches(win) &&
    ///     tp::Endian(Endian::little).matches(win) &&
    ///     tp::Env(Some(Env::msvc)).matches(win) &&
    ///     tp::Family(Some(Family::windows)).matches(win) &&
    ///     tp::Os(Some(Os::windows)).matches(win) &&
    ///     tp::PointerWidth(64).matches(win) &&
    ///     tp::Vendor(Some(Vendor::pc)).matches(win)
    /// );
    /// ```
    pub fn matches(self, target: &targ::TargetInfo) -> bool {
        use TargetPredicate::*;

        match self {
            Arch(a) => a == target.arch,
            Endian(end) => end == target.endian,
            Env(env) => env == target.env,
            Family(fam) => fam == target.family,
            Os(os) => os == target.os,
            PointerWidth(w) => w == target.pointer_width,
            Vendor(ven) => ven == target.vendor,
        }
    }
}

/// A single predicate in a `cfg()` expression
#[derive(Debug, PartialEq)]
pub enum Predicate<'a> {
    /// A target predicate, with the `target_` prefix
    Target(TargetPredicate),
    /// Whether rustc's test harness is [enabled](https://doc.rust-lang.org/reference/conditional-compilation.html#test)
    Test,
    /// [Enabled](https://doc.rust-lang.org/reference/conditional-compilation.html#debug_assertions)
    ///  when compiling without optimizations.
    DebugAssertions,
    /// [Enabled](https://doc.rust-lang.org/reference/conditional-compilation.html#proc_macro) for
    /// crates of the proc_macro type.
    ProcMacro,
    /// A [`feature = "<name>"`](https://doc.rust-lang.org/nightly/cargo/reference/features.html)
    Feature(&'a str),
    /// [target_feature](https://doc.rust-lang.org/reference/conditional-compilation.html#target_feature)
    TargetFeature(&'a str),
    /// A generic bare predicate key that doesn't match one of the known options, eg `cfg(bare)`
    Flag(&'a str),
    /// A generic key = "value" predicate that doesn't match one of the known options, eg `cfg(foo = "bar")`
    KeyValue { key: &'a str, val: &'a str },
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) enum InnerPredicate {
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
    Fn(Func),
    Predicate(InnerPredicate),
}

/// A parsed `cfg()` expression that can evaluated
#[derive(Debug)]
pub struct Expression {
    pub(crate) expr: SmallVec<[ExprNode; 5]>,
    // We keep the original string around for providing the arbitrary
    // strings that can make up an expression
    pub(crate) original: String,
}

impl Expression {
    /// An iterator over each predicate in the expression
    pub fn predicates(&self) -> impl Iterator<Item = Predicate<'_>> {
        self.expr.iter().filter_map(move |item| match item {
            ExprNode::Predicate(pred) => {
                let pred = pred.clone().to_pred(&self.original);
                Some(pred)
            }
            _ => None,
        })
    }

    /// Evaluates the expression, using the provided closure to determine the value of
    /// each predicate, which are then combined into a final result depending on the
    /// functions not(), all(), or any() in the expression.
    ///
    /// `eval_predicate` typically returns `bool`, but may return any type that implements
    /// the `Logic` trait.
    ///
    /// ## Examples
    ///
    /// ```
    /// use cfg_expr::{targets::*, Expression, Predicate};
    ///
    /// let linux_musl = get_target_by_triple("x86_64-unknown-linux-musl").unwrap();
    ///
    /// let expr = Expression::parse(r#"all(not(windows), target_env = "musl", any(target_arch = "x86", target_arch = "x86_64"))"#).unwrap();
    ///
    /// assert!(expr.eval(|pred| {
    ///     match pred {
    ///         Predicate::Target(tp) => tp.matches(linux_musl),
    ///         _ => false,
    ///     }
    /// }));
    /// ```
    ///
    /// Returning `Option<bool>`, where `None` indicates the result is unknown:
    ///
    /// ```
    /// use cfg_expr::{targets::*, Expression, Predicate};
    ///
    /// let expr = Expression::parse(r#"any(target_feature = "sse2", target_env = "musl")"#).unwrap();
    ///
    /// let linux_gnu = get_target_by_triple("x86_64-unknown-linux-gnu").unwrap();
    /// let linux_musl = get_target_by_triple("x86_64-unknown-linux-musl").unwrap();
    ///
    /// fn eval(expr: &Expression, target: &TargetInfo) -> Option<bool> {
    ///     expr.eval(|pred| {
    ///         match pred {
    ///             Predicate::Target(tp) => Some(tp.matches(target)),
    ///             Predicate::TargetFeature(_) => None,
    ///             _ => panic!("unexpected predicate"),
    ///         }
    ///     })
    /// }
    ///
    /// // Whether the target feature is present is unknown, so the whole expression evaluates to
    /// // None (unknown).
    /// assert_eq!(eval(&expr, linux_gnu), None);
    ///
    /// // Whether the target feature is present is irrelevant for musl, since the any() always
    /// // evaluates to true.
    /// assert_eq!(eval(&expr, linux_musl), Some(true));
    /// ```
    pub fn eval<EP, T>(&self, mut eval_predicate: EP) -> T
    where
        EP: FnMut(&Predicate<'_>) -> T,
        T: Logic,
    {
        let mut result_stack = SmallVec::<[T; 8]>::new();

        // We store the expression as postfix, so just evaluate each license
        // requirement in the order it comes, and then combining the previous
        // results according to each operator as it comes
        for node in self.expr.iter() {
            match node {
                ExprNode::Predicate(pred) => {
                    let pred = pred.to_pred(&self.original);
                    result_stack.push(eval_predicate(&pred));
                }
                ExprNode::Fn(Func::All(count)) => {
                    // all() with a comma separated list of configuration predicates.
                    let mut result = T::top();

                    for _ in 0..*count {
                        let r = result_stack.pop().unwrap();
                        result = result.and(r);
                    }

                    result_stack.push(result);
                }
                ExprNode::Fn(Func::Any(count)) => {
                    // any() with a comma separated list of configuration predicates.
                    let mut result = T::bottom();

                    for _ in 0..*count {
                        let r = result_stack.pop().unwrap();
                        result = result.or(r);
                    }

                    result_stack.push(result);
                }
                ExprNode::Fn(Func::Not) => {
                    // not() with a configuration predicate.
                    // It is true if its predicate is false
                    // and false if its predicate is true.
                    let r = result_stack.pop().unwrap();
                    result_stack.push(r.not());
                }
            }
        }

        result_stack.pop().unwrap()
    }
}

/// A propositional logic used to evaluate `Expression` instances.
///
/// An `Expression` consists of some predicates and the `any`, `all` and `not` operators. An
/// implementation of `Logic` defines how the `any`, `all` and `not` operators should be evaluated.
pub trait Logic {
    /// The result of an `all` operation with no operands, akin to Boolean `true`.
    fn top() -> Self;

    /// The result of an `any` operation with no operands, akin to Boolean `false`.
    fn bottom() -> Self;

    /// `AND`, which corresponds to the `all` operator.
    fn and(self, other: Self) -> Self;

    /// `OR`, which corresponds to the `any` operator.
    fn or(self, other: Self) -> Self;

    /// `NOT`, which corresponds to the `not` operator.
    fn not(self) -> Self;
}

/// A boolean logic.
impl Logic for bool {
    #[inline]
    fn top() -> Self {
        true
    }

    #[inline]
    fn bottom() -> Self {
        false
    }

    #[inline]
    fn and(self, other: Self) -> Self {
        self && other
    }

    #[inline]
    fn or(self, other: Self) -> Self {
        self || other
    }

    #[inline]
    fn not(self) -> Self {
        !self
    }
}

/// A three-valued logic -- `None` stands for the value being unknown.
///
/// The truth tables for this logic are described on
/// [Wikipedia](https://en.wikipedia.org/wiki/Three-valued_logic#Kleene_and_Priest_logics).
impl Logic for Option<bool> {
    #[inline]
    fn top() -> Self {
        Some(true)
    }

    #[inline]
    fn bottom() -> Self {
        Some(false)
    }

    #[inline]
    fn and(self, other: Self) -> Self {
        match (self, other) {
            // If either is false, the expression is false.
            (Some(false), _) => Some(false),
            (_, Some(false)) => Some(false),
            // If both are true, the expression is true.
            (Some(true), Some(true)) => Some(true),
            // One or both are unknown -- the result is unknown.
            _ => None,
        }
    }

    #[inline]
    fn or(self, other: Self) -> Self {
        match (self, other) {
            // If either is true, the expression is true.
            (Some(true), _) => Some(true),
            (_, Some(true)) => Some(true),
            // If both are false, the expression is false.
            (Some(false), Some(false)) => Some(false),
            // One or both are unknown -- the result is unknown.
            _ => None,
        }
    }

    #[inline]
    fn not(self) -> Self {
        match self {
            Some(v) => Some(!v),
            None => None,
        }
    }
}
