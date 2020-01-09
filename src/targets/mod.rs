use crate::error::Reason;

mod list;
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
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub enum Endianness {
        big,
        little,
    }
}

target_enum! {
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
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub enum Family {
        unix,
        windows,
    }
}

#[derive(Debug)]
pub struct TargetInfo {
    pub triple: &'static str,
    pub os: Option<Os>,
    pub arch: Arch,
    pub env: Option<Env>,
    pub vendor: Option<Vendor>,
    pub family: Option<Family>,
    pub pointer_width: u8,
    pub endian: Endianness,
}

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
