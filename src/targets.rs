use crate::error::Reason;
use std::borrow::Cow;

mod builtins;

/// A list of all of the [builtin](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_target/spec/index.html#modules)
/// targets known to rustc, as of 1.54.0
pub use builtins::ALL_BUILTINS;

/// The unique identifier for a target.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Triple(pub Cow<'static, str>);

/// The "architecture" field
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Arch(pub Cow<'static, str>);

/// The "vendor" field, which in practice is little more than an arbitrary modifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Vendor(pub Cow<'static, str>);

/// The "operating system" field, which sometimes implies an environment, and
/// sometimes isn't an actual operating system.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Os(pub Cow<'static, str>);

/// The target family, which describes a set of targets grouped in some logical manner, typically by
/// operating system. This includes values like `unix` and `windows`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Family(pub Cow<'static, str>);

/// The "environment" field, which specifies an ABI environment on top of the
/// operating system. In many configurations, this field is omitted, and the
/// environment is implied by the operating system.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Env(pub Cow<'static, str>);

macro_rules! field_impls {
    ($kind:ident) => {
        impl $kind {
            /// Constructs a new instance of this field.
            ///
            /// This method accepts both owned `String`s and `&'static str`s.
            #[inline]
            pub fn new(val: impl Into<Cow<'static, str>>) -> Self {
                Self(val.into())
            }

            /// Constructs a new instance of this field from a `&'static str`.
            #[inline]
            pub const fn new_const(val: &'static str) -> Self {
                Self(Cow::Borrowed(val))
            }

            /// Returns the string for the field.
            #[inline]
            pub fn as_str(&self) -> &str {
                &*self.0
            }
        }

        impl AsRef<str> for $kind {
            #[inline]
            fn as_ref(&self) -> &str {
                &*self.0
            }
        }

        impl std::fmt::Display for $kind {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}

field_impls!(Triple);
field_impls!(Arch);
field_impls!(Vendor);
field_impls!(Os);
field_impls!(Family);
field_impls!(Env);

macro_rules! target_enum {
    (
        $(#[$outer:meta])*
        pub enum $kind:ident {
            $(
                $(#[$inner:ident $($args:tt)*])*
                $name:ident $(= $value:expr)?,
            )+
        }
    ) => {
        $(#[$outer])*
        #[allow(non_camel_case_types)]
        pub enum $kind {
            $(
                $(#[$inner $($args)*])*
                $name $(= $value)?,
            )+
        }

        impl_from_str! {
            $kind {
                $(
                    $(#[$inner $($args)*])*
                    $name $(= $value)?,
                )+
            }
        }
    };
}

macro_rules! impl_from_str {
    (
        $kind:ident {
            $(
                $(#[$attr:ident $($args:tt)*])*
                $name:ident $(= $value:expr)?,
            )+
        }
    ) => {
        impl std::str::FromStr for $kind {
            type Err = Reason;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(stringify!($name) => Ok(Self::$name),)+
                    _ => Err(Reason::Unexpected(&[$(stringify!($name),)+])),
                }
            }
        }
    };
}

target_enum! {
    /// The endian types known to rustc
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub enum Endian {
        big,
        little,
    }
}

/// Contains information regarding a particular target known to rustc
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TargetInfo {
    /// The target's unique identifier
    pub triple: Triple,
    /// The target's operating system, if any. Used by the
    /// [target_os](https://doc.rust-lang.org/reference/conditional-compilation.html#target_os)
    /// predicate.
    pub os: Option<Os>,
    /// The target's CPU architecture. Used by the
    /// [target_arch](https://doc.rust-lang.org/reference/conditional-compilation.html#target_arch)
    /// predicate.
    pub arch: Arch,
    /// The target's ABI/libc used, if any. Used by the
    /// [target_env](https://doc.rust-lang.org/reference/conditional-compilation.html#target_env)
    /// predicate.
    pub env: Option<Env>,
    /// The target's vendor, if any. Used by the
    /// [target_vendor](https://doc.rust-lang.org/reference/conditional-compilation.html#target_vendor)
    /// predicate.
    pub vendor: Option<Vendor>,
    /// The target's family, if any. Used by the
    /// [target_family](https://doc.rust-lang.org/reference/conditional-compilation.html#target_family)
    /// predicate.
    pub family: Option<Family>,
    /// The size of the target's pointer type. Used by the
    /// [target_pointer_width](https://doc.rust-lang.org/reference/conditional-compilation.html#target_pointer_width)
    /// predicate.
    pub pointer_width: u8,
    /// The target's endianness. Used by the
    /// [target_endian](https://doc.rust-lang.org/reference/conditional-compilation.html#target_endian)
    /// predicate.
    pub endian: Endian,
}

/// Attempts to find the `TargetInfo` for the specified target triple
///
/// ```
/// assert!(cfg_expr::targets::get_builtin_target_by_triple("x86_64-unknown-linux-musl").is_some());
/// ```
pub fn get_builtin_target_by_triple(triple: &str) -> Option<&'static TargetInfo> {
    ALL_BUILTINS
        .binary_search_by(|ti| ti.triple.as_ref().cmp(triple))
        .map(|i| &ALL_BUILTINS[i])
        .ok()
}

/// Retrieves the version of rustc for which the built-in targets were
/// retrieved from. Targets may be added and removed between different rustc
/// versions.
///
/// ```
/// assert_eq!("1.58.0", cfg_expr::targets::rustc_version());
/// ```
pub fn rustc_version() -> &'static str {
    builtins::RUSTC_VERSION
}

#[cfg(test)]
mod test {
    use crate::targets::get_builtin_target_by_triple;
    use std::collections::{BTreeSet, HashSet};

    // rustc's target-list is currently sorted lexicographically
    // by the target-triple, so ensure that stays the case
    #[test]
    fn targets_are_sorted() {
        for window in super::ALL_BUILTINS.windows(2) {
            assert!(window[0].triple < window[1].triple);
        }
    }

    // Ensure our workaround for https://github.com/rust-lang/rust/issues/36156
    // still functions
    #[test]
    fn has_ios() {
        assert_eq!(
            8,
            super::ALL_BUILTINS
                .iter()
                .filter(|ti| ti.os == Some(super::Os::ios))
                .count()
        );
    }

    // Ensure that TargetInfo can be used as keys for btree and hash-based data structures.
    #[test]
    fn set_map_key() {
        let target_info =
            get_builtin_target_by_triple("x86_64-unknown-linux-gnu").expect("known target");

        let mut btree_set = BTreeSet::new();
        btree_set.insert(target_info);

        let mut hash_set = HashSet::new();
        hash_set.insert(target_info);
    }
}
