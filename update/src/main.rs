use std::{fmt::Write, process::Command};

fn real_main() -> Result<(), String> {
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_owned());

    let mut path = std::env::var("PATH").unwrap_or_else(|_| "".to_owned());

    let sep = if cfg!(unix) { ':' } else { ';' };

    path.push(sep);
    write!(
        path,
        "{}",
        std::env::current_exe().unwrap().parent().unwrap().display()
    )
    .unwrap();

    // Get the rustc version
    let output = Command::new(&rustc)
        .env("PATH", &path)
        .arg("--version")
        .output()
        .map_err(|e| format!("failed to run rustc --version: {}", e))?;

    if !output.status.success() {
        return Err(format!("rustc --version {}", output.status));
    }

    let version = String::from_utf8(output.stdout).unwrap();

    let version = version.splitn(3, ' ').nth(1).unwrap();

    // Get the list of possible targets
    let output = Command::new(&rustc)
        .env("PATH", &path)
        .args(&["--print", "target-list"])
        .output()
        .map_err(|e| format!("failed to run rustc --print target-list: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "rustc --print target-list returned {}",
            output.status
        ));
    }

    let targets = String::from_utf8(output.stdout).unwrap();
    let mut out = String::with_capacity(4 * 1024);

    out.push_str(
        "/*
 *
 * AUTO-GENERATED BY ./update
 * DO NOT MODIFY
 *
 * cargo run --manifest-path update/Cargo.toml
 */",
    );

    out.push_str(
        "

    #![allow(non_upper_case_globals, non_camel_case_types)]

    use super::*;

    ",
    );

    write!(
        out,
        "pub(crate) const RUSTC_VERSION: &str = \"{}\";",
        version
    )
    .unwrap();

    out.push_str(
        "

        pub const ALL_BUILTINS: &[TargetInfo<'static>] = &[
",
    );

    // Keep one target triple per architecture, as we need a full
    // triple even if the only part that matters is the architecture
    //let mut arches = HashMap::new();

    let mut arches: Vec<String> = Vec::new();
    let mut vendors: Vec<String> = Vec::new();
    let mut oses: Vec<String> = Vec::new();
    let mut envs: Vec<String> = Vec::new();

    for target in targets.lines() {
        let output = Command::new(&rustc)
            .env("PATH", &path)
            .arg("--target")
            .arg(target)
            .args(&["--print", "cfg"])
            .output()
            .map_err(|e| format!("failed to run rustc: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "failed to retrieve target {}: {}",
                target,
                String::from_utf8(output.stderr)
                    .map_err(|e| format!("unable to parse stderr: {}", e))?
            ));
        }

        let kv = String::from_utf8(output.stdout).unwrap();

        //let mut num_feats = 0;
        let mut arch = None;
        let mut endian = None;
        let mut env = None;
        let mut family = None;
        let mut os = None;
        let mut width = None;
        let mut vendor = None;

        for line in kv.lines() {
            let eq_ind = line.find('=');
            match eq_ind {
                None => {
                    continue;
                }
                Some(i) => {
                    let key = &line[7..i];
                    let val = &line[i + 2..line.len() - 1];

                    match key {
                        "arch" => {
                            arch = Some(val);
                            // if arches.get(val).is_none() {
                            //     arches.insert(val, target);
                            // }
                        }
                        "endian" => endian = Some(val),
                        "env" => {
                            if !val.is_empty() {
                                env = Some(val)
                            }
                        }
                        "family" => family = Some(val),
                        "feature" => {

                            // num_feats += 1;
                            // write!(
                            //     features,
                            //     "Features::{} as u32 | ",
                            //     match val {
                            //         "sse4.1" => "sse41",
                            //         "sse4.2" => "sse42",
                            //         "crt-static" => "crt_static",
                            //         passthrough => passthrough,
                            //     }
                            // )
                            // .unwrap();
                        }
                        "os" => {
                            if val != "none" {
                                os = Some(val)
                            }
                        }
                        "pointer_width" => width = Some(val),
                        "vendor" => {
                            if !val.is_empty() {
                                vendor = Some(val)
                            }
                        }
                        _ => panic!("unknown target option {}", line),
                    }
                }
            }
        }

        fn insert(thing: Option<&str>, things: &mut Vec<String>) {
            if let Some(v) = thing {
                if let Err(i) = things.binary_search_by(|t| t.as_str().cmp(v)) {
                    things.insert(i, v.to_owned());
                }
            }
        }

        insert(arch, &mut arches);
        insert(vendor, &mut vendors);
        insert(os, &mut oses);
        insert(env, &mut envs);

        writeln!(
            out,
            "    TargetInfo {{
        triple: \"{triple}\",
        os: {os},
        arch: Arch::{arch},
        env: {env},
        vendor: {vendor},
        family: {family},
        pointer_width: {width},
        endian: Endian::{endian},
    }},",
            triple = target,
            os = os
                .map(|os| format!("Some(Os::{})", os))
                .unwrap_or_else(|| "None".to_owned()),
            arch = arch.expect("target had no arch"),
            env = env
                .map(|e| format!("Some(Env::{})", e))
                .unwrap_or_else(|| "None".to_owned()),
            vendor = vendor
                .map(|v| format!("Some(Vendor::{})", v))
                .unwrap_or_else(|| "None".to_owned()),
            family = family
                .map(|f| format!("Some(Family::{})", f))
                .unwrap_or_else(|| "None".to_owned()),
            width = width.expect("target had no pointer_width"),
            endian = endian.expect("target had no endian"),
        )
        .unwrap();
    }

    writeln!(out, "];").unwrap();

    write_impls(&mut out, "Arch", arches);
    write_impls(&mut out, "Vendor", vendors);
    write_impls(&mut out, "Os", oses);
    write_impls(&mut out, "Env", envs);

    std::fs::write("src/targets/builtins.rs", out)
        .map_err(|e| format!("failed to write target_list.rs: {}", e))?;

    let status = Command::new("rustfmt")
        .args(&["--edition", "2018", "src/targets/builtins.rs"])
        .status()
        .map_err(|e| format!("failed to run rustfmt: {}", e))?;

    if !status.success() {
        return Err(format!("failed to successfuly format: {}", status));
    }

    Ok(())
}

fn write_impls(out: &mut String, typ: &'static str, builtins: Vec<String>) {
    writeln!(out, "\nimpl<'a> super::{}<'a> {{", typ).unwrap();

    for thing in builtins {
        writeln!(
            out,
            "pub const {}: {}<'static> = {}(\"{}\");",
            thing, typ, typ, thing
        )
        .unwrap();
    }

    writeln!(out, "}}").unwrap();
}

fn main() {
    // Workaround for https://github.com/rust-lang/rust/issues/36156
    // the ios targets attempt to find an SDK path, and then just hide
    // the target altogether if it doesn't exist, but we don't care about
    // that, we just want to get the metadata for the target, so we
    // cheat and create a script that just echos our current path that
    // is enough to satisfy rustc so that it spits out the info we want
    if std::env::args().find(|a| a == "--show-sdk-path").is_some() {
        println!("{}", std::env::current_dir().unwrap().display());
        return;
    }

    if let Err(ref e) = real_main() {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
