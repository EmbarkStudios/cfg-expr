use crate::error::Reason;

mod list;

/// A list of all of the [builtin](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_target/spec/index.html#modules)
/// targets known to rustc, as of 1.40
pub use list::ALL_TARGETS as ALL;

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
    /// All of the operating systems known to rustc
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub enum Os {
        haiku,
        openbsd,
        freebsd,
        redox,
        vxworks,
        uefi,
        emscripten,
        netbsd,
        fuchsia,
        cloudabi,
        wasi,
        solaris,
        cuda,
        dragonfly,
        l4re,
        android,
        macos,
        hermit,
        linux,
        windows,
        unknown,
        ios,
    }
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
    /// All of the target environments known to rustc
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub enum Env {
        uclibc,
        sgx,
        eabihf,
        relibc,
        gnu,
        musl,
        msvc,
        gnueabihf,
    }
}

target_enum! {
    /// All of the target vendors known to rustc
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub enum Vendor {
        pc,
        unknown,
        uwp,
        nvidia,
        sun,
        fortanix,
        wrs,
        rumprun,
        apple,
    }
}

target_enum! {
    /// All of the CPU architectures known to rustc
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub enum Arch {
        x86_64,
        wasm32,
        msp430,
        mips,
        powerpc,
        arm,
        mips64,
        sparc64,
        hexagon,
        riscv64,
        aarch64,
        powerpc64,
        riscv32,
        sparc,
        nvptx64,
        x86,
        s390x,
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
pub struct TargetInfo {
    /// The target's unique identifier
    pub triple: &'static str,
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
/// assert!(cfg_expr::targets::get_target_by_triple("x86_64-unknown-linux-musl").is_some());
/// ```
pub fn get_target_by_triple(triple: &str) -> Option<&'static TargetInfo> {
    ALL.binary_search_by(|ti| ti.triple.cmp(triple))
        .map(|i| &ALL[i])
        .ok()
}

#[cfg(test)]
mod test {
    // rustc's target-list is currently sorted lexicographically
    // by the target-triple, so ensure that stays the case
    #[test]
    fn targets_are_sorted() {
        for window in super::ALL.windows(2) {
            assert!(window[0].triple < window[1].triple);
        }
    }

    // Ensure our workaround for https://github.com/rust-lang/rust/issues/36156
    // still functions
    #[test]
    fn has_ios() {
        assert_eq!(
            6,
            super::ALL
                .iter()
                .filter(|ti| ti.os == Some(super::Os::ios))
                .count()
        );
    }
}
