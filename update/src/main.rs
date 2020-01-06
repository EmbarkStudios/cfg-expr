use std::{fmt::Write, process::Command};

fn real_main() -> Result<(), String> {
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_owned());

    let mut path = std::env::var("PATH").unwrap_or_else(|_| "".to_owned());

    let sep = if cfg!(unix) { ':' } else { ';' };

    path.push(sep);
    write!(path, "{}", std::env::current_dir().unwrap().display()).unwrap();

    // Get the list of possible targets
    let output = Command::new(&rustc)
        .env("PATH", &path)
        .args(&["--print", "target-list"])
        .output()
        .map_err(|e| format!("failed to run --print target-list: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "rustc --print target-list returned {}",
            output.status
        ));
    }

    let targets = String::from_utf8(output.stdout).unwrap();
    let mut all_targets = String::with_capacity(4 * 1024);

    all_targets.push_str(
        "/*
 *
 * AUTO-GENERATED BY ./update
 * DO NOT MODIFY
 *
 * cargo run --manifest-path update/Cargo.toml
 */",
    );

    all_targets.push_str(
        "

use super::*;

pub const ALL_TARGETS: &[TargetInfo] = &[
",
    );

    // Keep one target triple per architecture, as we need a full
    // triple even if the only part that matters is the architecture
    //let mut arches = HashMap::new();

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

        let mut num_feats = 0;
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

        // match num_feats {
        //     0 => features.push_str("0x0"),
        //     _ => features.truncate(features.len() - 3),
        // }

        writeln!(
            all_targets,
            "    TargetInfo {{
        triple: \"{triple}\",
        os: {os},
        arch: Arch::{arch},
        env: {env},
        vendor: {vendor},
        family: {family},
        pointer_width: {width},
        endian: Endianness::{endian},
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

    writeln!(all_targets, "];").unwrap();

    std::fs::write("src/targets/list.rs", all_targets)
        .map_err(|e| format!("failed to write target_list.rs: {}", e))?;

    let status = Command::new("rustfmt")
        .args(&["--edition", "2018", "src/targets/list.rs"])
        .status()
        .map_err(|e| format!("failed to run rustfmt: {}", e))?;

    if !status.success() {
        return Err(format!("failed to successfuly format: {}", status));
    }

    // Now, grab all of the available features for each unique architecture
    // for (arch, target) in arches {
    //     let output = Command::new(&rustc)
    //         .env("PATH", &path)
    //         .arg("--target")
    //         .arg(target)
    //         .args(&["--print", "target-features"])
    //         .output()
    //         .map_err(|e| format!("failed to run rustc: {}", e))?;

    //     if !output.status.success() {
    //         return Err(format!(
    //             "failed to retrieve target-features {}: {}",
    //             target,
    //             String::from_utf8(output.stderr)
    //                 .map_err(|e| format!("unable to parse stderr: {}", e))?
    //         ));
    //     }

    //     let kv = String::from_utf8(output.stdout).unwrap();

    //     for line in kv {
    //         // The output includes additional text that we don't need
    //         if !line.starts_with("    ") {
    //             continue;
    //         }

    //         // Each kv is of the form "    <name><spaces>-<space><description>"
    //         let name_end = line[4..]
    //             .find(' ')
    //             .ok_or_else(|| format!("invalid target-feature line: {}", line))?;

    //         let name = &line[4..name_end + 4];

    //         // Meh :P
    //         let name = name.replace('-', "_");
    //         let name = name.replace('.', "_");
    //     }
    // }

    Ok(())
}

fn main() {
    // Workaround for https://github.com/rust-lang/rust/issues/36156
    // the ios targets attempt to find an SDK path, and then just hide
    // the target altogether if it doesn't exist, but we don't care about
    // that, we just want to get the metadata for the target, so we
    // cheat and create a script that just echos our current path that
    // is enought to satisfy rustc so that it spits out the info we want
    if std::env::args().find(|a| a == "--show-sdk-path").is_some() {
        println!("{}", std::env::current_dir().unwrap().display());
        return;
    }

    if let Err(ref e) = real_main() {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
