use cfg_expr::{
    expr::Predicate,
    targets::{get_target_by_triple, ALL as all},
    Expression,
};

macro_rules! tg_match {
    ($pred:expr, $target:expr) => {
        match $pred {
            Predicate::Target(tg) => tg.matches(&$target),
            _ => panic!("not a target predicate"),
        }
    };

    ($pred:expr, $target:expr, $feats:expr) => {
        match $pred {
            Predicate::Target(tg) => tg.matches(&$target),
            Predicate::TargetFeature(feat) => $feats.iter().find(|f| *f == feat).is_some(),
            _ => panic!("not a target predicate"),
        }
    };
}

#[test]
fn target_family() {
    let matches_any_family = Expression::parse("any(unix, target_family = \"windows\")").unwrap();
    let impossible = Expression::parse("all(windows, target_family = \"unix\")").unwrap();

    for target in all {
        match target.family {
            Some(_) => {
                assert!(matches_any_family.eval(|pred| { tg_match!(pred, target) }));
                assert!(!impossible.eval(|pred| { tg_match!(pred, target) }));
            }
            None => {
                assert!(!matches_any_family.eval(|pred| { tg_match!(pred, target) }));
                assert!(!impossible.eval(|pred| { tg_match!(pred, target) }));
            }
        }
    }
}

#[test]
fn very_specific() {
    let specific = Expression::parse(
        r#"all(
            target_os = "windows",
            target_arch = "x86",
            windows,
            target_env = "msvc",
            target_feature = "fxsr",
            target_feature = "sse",
            target_feature = "sse2",
            target_pointer_width = "32",
            target_endian = "little",
            not(target_vendor = "uwp"),
        )"#,
    )
    .unwrap();

    for target in all {
        assert_eq!(
            target.triple == "i686-pc-windows-msvc" || target.triple == "i586-pc-windows-msvc",
            specific.eval(|pred| { tg_match!(pred, target, &["fxsr", "sse", "sse2"]) }),
            "expected true for i686-pc-windows-msvc, but got true for {}",
            target.triple,
        );
    }

    let specific = Expression::parse(
        r#"cfg(
        all(
            target_arch = "wasm32", 
            target_vendor = "unknown", 
            target_os = "unknown", 
            target_env = ""
        )
    )"#,
    )
    .unwrap();

    for target in all {
        assert_eq!(
            target.triple == "wasm32-unknown-unknown",
            specific.eval(|pred| { tg_match!(pred, target) }),
            "failed {}",
            target.triple,
        );
    }
}

#[test]
fn complex() {
    let complex = Expression::parse(r#"cfg(all(unix, not(any(target_os="macos", target_os="android", target_os="emscripten"))))"#).unwrap();

    // Should match linuxes
    let linux_gnu = get_target_by_triple("x86_64-unknown-linux-gnu").unwrap();
    let linux_musl = get_target_by_triple("x86_64-unknown-linux-musl").unwrap();

    assert!(complex.eval(|pred| tg_match!(pred, linux_gnu)));
    assert!(complex.eval(|pred| tg_match!(pred, linux_musl)));

    // Should *not* match windows or mac or android
    let windows_msvc = get_target_by_triple("x86_64-pc-windows-msvc").unwrap();
    let mac = get_target_by_triple("x86_64-apple-darwin").unwrap();
    let android = get_target_by_triple("aarch64-linux-android").unwrap();

    assert!(!complex.eval(|pred| tg_match!(pred, windows_msvc)));
    assert!(!complex.eval(|pred| tg_match!(pred, mac)));
    assert!(!complex.eval(|pred| tg_match!(pred, android)));

    let complex =
        Expression::parse(r#"all(not(target_os = "ios"), not(target_os = "android"))"#).unwrap();

    assert!(complex.eval(|pred| tg_match!(pred, linux_gnu)));
    assert!(complex.eval(|pred| tg_match!(pred, linux_musl)));
    assert!(complex.eval(|pred| tg_match!(pred, windows_msvc)));
    assert!(complex.eval(|pred| tg_match!(pred, mac)));
    assert!(!complex.eval(|pred| tg_match!(pred, android)));
}

#[test]
fn features() {
    let enabled = ["good", "bad", "ugly"];

    let many_features = Expression::parse(
        r#"all(feature = "good", feature = "bad", feature = "ugly", not(feature = "nope"))"#,
    )
    .unwrap();

    assert!(many_features.eval(|pred| {
        match pred {
            Predicate::Feature(name) => {
                println!("CHECKING FEATURE `{}`", name);
                enabled.contains(name)
            }
            _ => false,
        }
    }));

    let feature_and_target_feature =
        Expression::parse(r#"all(feature = "make_fast", target_feature = "sse4.2")"#).unwrap();

    assert!(feature_and_target_feature.eval(|pred| {
        match pred {
            Predicate::Feature(name) => *name == "make_fast",
            Predicate::TargetFeature(feat) => *feat == "sse4.2",
            _ => false,
        }
    }));
}
