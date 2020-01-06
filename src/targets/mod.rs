use crate::error::Reason;

mod list;
pub use list::ALL_TARGETS;

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

// #[derive(Clone, Copy, PartialEq, Debug)]
// #[allow(non_camel_case_types)]
// pub enum Features {
//     /// [SSE](https://en.wikipedia.org/wiki/Streaming_SIMD_Extensions) — Streaming SIMD Extensions
//     sse = 0x1,
//     /// [SSE2](https://en.wikipedia.org/wiki/SSE2) — Streaming SIMD Extensions 2
//     sse2 = 0x2,
//     /// [SSE3](https://en.wikipedia.org/wiki/SSE3) — Streaming SIMD Extensions 3
//     sse3 = 0x4,
//     /// [SSSE3](https://en.wikipedia.org/wiki/SSSE3) — Supplemental Streaming SIMD Extensions 3
//     ssse3 = 0x8,
//     /// [SSE4.1](https://en.wikipedia.org/wiki/SSE4#SSE4.1) — Streaming SIMD Extensions 4.1
//     sse41 = 0x10,
//     /// [SSE4.2](https://en.wikipedia.org/wiki/SSE4#SSE4.2) — Streaming SIMD Extensions 4.2
//     sse42 = 0x20,
//     /// [popcnt](https://www.felixcloutier.com/x86/popcnt) — Count of bits set to 1
//     popcnt = 0x40,
//     /// [fxsave](https://www.felixcloutier.com/x86/fxsave) and [fxrstor](https://www.felixcloutier.com/x86/fxrstor) — Save and restore x87 FPU, MMX Technology, and SSE State
//     fxsr = 0x80,
//     /// A static C runtime is available.
//     crt_static = 0x100,
//     /// [rdrand](https://en.wikipedia.org/wiki/RDRAND) — Read random number
//     rdrand = 0x200,
//     /// [rdseed](https://en.wikipedia.org/wiki/RDRAND) — Read random seed
//     rdseed = 0x400,
// }

// impl std::ops::BitOr for Features {
//     type Output = u32;
//     fn bitor(self, other: Features) -> Self::Output {
//         self as u32 | other as u32
//     }
// }

// impl std::ops::BitOr<u32> for Features {
//     type Output = u32;
//     fn bitor(self, other: u32) -> Self::Output {
//         self as u32 | other
//     }
// }

// impl std::ops::BitOr<Features> for u32 {
//     type Output = u32;
//     fn bitor(self, other: Features) -> Self::Output {
//         self | other as u32
//     }
// }

// impl std::ops::BitAnd for Features {
//     type Output = u32;
//     fn bitand(self, other: Features) -> Self::Output {
//         self as u32 & other as u32
//     }
// }

// impl std::ops::BitAnd<u32> for Features {
//     type Output = u32;
//     fn bitand(self, other: u32) -> Self::Output {
//         self as u32 & other
//     }
// }

// impl std::ops::BitAnd<Features> for u32 {
//     type Output = u32;
//     fn bitand(self, other: Features) -> Self::Output {
//         self & other as u32
//     }
// }

// impl std::str::FromStr for Features {
//     type Err = Reason;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Ok(match s {
//             "sse" => Self::sse,
//             "sse2" => Self::sse2,
//             "sse3" => Self::sse3,
//             "ssse3" => Self::ssse3,
//             "sse4.1" => Self::sse41,
//             "sse4.2" => Self::sse42,
//             "popcnt" => Self::popcnt,
//             "fxsr" => Self::fxsr,
//             "crt-static" => Self::crt_static,
//             "rdrand" => Self::rdrand,
//             "rdseed" => Self::rdseed,
//             _ => {
//                 return Err(Reason::Unexpected(&[
//                     "sse",
//                     "sse2",
//                     "sse3",
//                     "ssse3",
//                     "sse4.1",
//                     "sse4.2",
//                     "popcnt",
//                     "fxsr",
//                     "crt-static",
//                     "rdrand",
//                     "rdseed",
//                 ]))
//             }
//         })
//     }
// }

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

#[cfg(test)]
mod test {
    // rustc's target-list is currently sorted lexicographically
    // by the target-triple, so ensure that stays the case
    #[test]
    fn targets_are_sorted() {
        for window in super::ALL_TARGETS.windows(2) {
            assert!(window[0].triple < window[1].triple);
        }
    }

    // Ensure our workaround for https://github.com/rust-lang/rust/issues/36156
    // still functions
    #[test]
    fn has_ios() {
        assert_eq!(
            6,
            super::ALL_TARGETS
                .iter()
                .filter(|ti| ti.os == Some(super::Os::ios))
                .count()
        );
    }
}
