use crate::error::Reason;

mod builtins;

/// A list of all of the [builtin](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_target/spec/index.html#modules)
/// targets known to rustc, as of 1.43.1
pub use builtins::ALL_BUILTINS;

/// The "architecture" field
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Arch<'a>(pub &'a str);

/// The "vendor" field, which in practice is little more than an arbitrary modifier.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vendor<'a>(pub &'a str);

/// The "operating system" field, which sometimes implies an environment, and
/// sometimes isn't an actual operating system.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Os<'a>(pub &'a str);

/// The "environment" field, which specifies an ABI environment on top of the
/// operating system. In many configurations, this field is omitted, and the
/// environment is implied by the operating system.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Env<'a>(pub &'a str);

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
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub enum Endian {
        big,
        little,
    }
}

target_enum! {
    /// All of the target families known to rustc
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub enum Family {
        /// Everything that isn't windows, and has a family!
        unix,
        /// The lone wolf of target families.
        windows,
    }
}

/// Contains information regarding a particular target known to rustc
#[derive(Debug)]
pub struct TargetInfo<'a> {
    /// The target's unique identifier
    pub triple: &'a str,
    /// The target's operating system, if any. Used by the
    /// [target_os](https://doc.rust-lang.org/reference/conditional-compilation.html#target_os)
    /// predicate.
    pub os: Option<Os<'a>>,
    /// The target's CPU architecture. Used by the
    /// [target_arch](https://doc.rust-lang.org/reference/conditional-compilation.html#target_arch)
    /// predicate.
    pub arch: Arch<'a>,
    /// The target's ABI/libc used, if any. Used by the
    /// [target_env](https://doc.rust-lang.org/reference/conditional-compilation.html#target_env)
    /// predicate.
    pub env: Option<Env<'a>>,
    /// The target's vendor, if any. Used by the
    /// [target_vendor](https://doc.rust-lang.org/reference/conditional-compilation.html#target_vendor)
    /// predicate.
    pub vendor: Option<Vendor<'a>>,
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

// use crate::ParseError;

// impl<'a> std::str::FromStr for TargetInfo<'a> {
//     type Err = ParseError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let mut components = s.split('-');

//         let mut cur;
//         cur = components.next();

//         let arch = if let Some(s) = cur {
//             match Arch::from_str(s) {
//                 Ok(arch) => {
//                     cur = components.next();
//                     arch
//                 }
//                 Err(reason) => {
//                     return Err(ParseError {
//                         original: s.to_owned(),
//                         span: 0..s.len(),
//                         reason,
//                     })
//                 }
//             }
//         } else {
//             // Require architecture for now
//             return Err(ParseError {
//                 original: s.to_owned(),
//                 span: 0..s.len(),
//                 reason: Reason::Empty,
//             });
//         };

//         let mut res = Self {
//             triple: s,
//             os: None,
//             arch,
//             env: None,
//             vendor: None,
//             family: None,
//             pointer_width: 0,
//             endian: Endian::little,
//         };

//         let mut has_vendor = false;
//         let mut has_operating_system = false;
//         if let Some(s) = current_part {
//             if let Ok(vendor) = Vendor::from_str(s) {
//                 has_vendor = true;
//                 result.vendor = vendor;
//                 current_part = parts.next();
//             }
//         }

//         if !has_operating_system {
//             if let Some(s) = current_part {
//                 if let Ok(operating_system) = OperatingSystem::from_str(s) {
//                     has_operating_system = true;
//                     result.operating_system = operating_system;
//                     current_part = parts.next();
//                 }
//             }
//         }

//         let mut has_environment = false;
//         if let Some(s) = current_part {
//             if let Ok(environment) = Environment::from_str(s) {
//                 has_environment = true;
//                 result.environment = environment;
//                 current_part = parts.next();
//             }
//         }

//         // The binary format is frequently omitted; if that's the case here,
//         // infer it from the other fields.
//         if !has_binary_format {
//             result.binary_format = default_binary_format(&result);
//         }

//         if let Some(s) = current_part {
//             Err(
//                 if !has_vendor && !has_operating_system && !has_environment && !has_binary_format {
//                     ParseError::UnrecognizedVendor(s.to_owned())
//                 } else if !has_operating_system && !has_environment && !has_binary_format {
//                     ParseError::UnrecognizedOperatingSystem(s.to_owned())
//                 } else if !has_environment && !has_binary_format {
//                     ParseError::UnrecognizedEnvironment(s.to_owned())
//                 } else if !has_binary_format {
//                     ParseError::UnrecognizedBinaryFormat(s.to_owned())
//                 } else {
//                     ParseError::UnrecognizedField(s.to_owned())
//                 },
//             )
//         } else {
//             Ok(result)
//         }
//     }
// }

/// Attempts to find the `TargetInfo` for the specified target triple
///
/// ```
/// assert!(cfg_expr::targets::get_builtin_target_by_triple("x86_64-unknown-linux-musl").is_some());
/// ```
pub fn get_builtin_target_by_triple(triple: &str) -> Option<&'static TargetInfo<'static>> {
    ALL_BUILTINS
        .binary_search_by(|ti| ti.triple.cmp(triple))
        .map(|i| &ALL_BUILTINS[i])
        .ok()
}

/// Retrieves the version of rustc for which the built-in targets were
/// retrieved from. Targets may be added and removed between different rustc
/// versions.
///
/// ```
/// assert_eq!("1.43.1", cfg_expr::targets::rustc_version());
/// ```
pub fn rustc_version() -> &'static str {
    builtins::RUSTC_VERSION
}

#[cfg(test)]
mod test {
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
            6,
            super::ALL_BUILTINS
                .iter()
                .filter(|ti| ti.os == Some(super::Os::ios))
                .count()
        );
    }
}
