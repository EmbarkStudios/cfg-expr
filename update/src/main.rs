use std::{fmt::Write, process::Command};

fn real_main() -> Result<(), String> {
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_owned());

    // Get the rustc version
    let output = Command::new(&rustc)
        .arg("--version")
        .output()
        .map_err(|e| format!("failed to run rustc --version: {e}"))?;

    if !output.status.success() {
        return Err(format!("rustc --version {}", output.status));
    }

    let version = String::from_utf8(output.stdout).unwrap();

    let version = version.splitn(3, ' ').nth(1).unwrap();

    // Get the list of possible targets
    let output = Command::new(&rustc)
        .args(&["--print", "target-list"])
        .output()
        .map_err(|e| format!("failed to run rustc --print target-list: {e}"))?;

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

        pub const ALL_BUILTINS: &[TargetInfo] = &[
",
    );

    // Keep one target triple per architecture, as we need a full
    // triple even if the only part that matters is the architecture
    //let mut arches = HashMap::new();

    let mut abis: Vec<String> = Vec::new();
    let mut arches: Vec<String> = Vec::new();
    let mut vendors: Vec<String> = Vec::new();
    let mut oses: Vec<String> = Vec::new();
    let mut envs: Vec<String> = Vec::new();
    let mut families: Vec<String> = Vec::new();
    let mut family_groups: Vec<Vec<String>> = Vec::new();
    let mut has_atomics: Vec<HasAtomicElement> = Vec::new();
    let mut has_atomic_groups: Vec<Vec<HasAtomicElement>> = Vec::new();
    let mut panics: Vec<String> = Vec::new();
    //let mut relocation_models: Vec<String> = Vec::new();

    for target in targets.lines() {
        let output = {
            let mut cmd = Command::new(&rustc);

            // this gives an error on 1.76.0 but not on nightly so...
            if target == "aarch64-apple-watchos" {
                cmd.arg("+nightly");
            }

            //.env("PATH", &path)
            cmd.arg("--target")
                .arg(target)
                .args(&["--print", "cfg"])
                .output()
                .map_err(|e| format!("failed to run rustc: {e}"))?
        };

        if !output.status.success() {
            return Err(format!(
                "failed to retrieve target {target}: {}",
                String::from_utf8(output.stderr)
                    .map_err(|e| format!("unable to parse stderr: {e}"))?
            ));
        }

        let kv = String::from_utf8(output.stdout).unwrap();

        //let mut num_feats = 0;
        let mut abi = None;
        let mut arch = None;
        let mut endian = None;
        let mut env = None;
        let mut family_group = Vec::new();
        let mut os = None;
        let mut width = None;
        let mut vendor = None;
        let mut panic = None;
        //let mut relocation_model = None;
        let mut has_atomic_group = Vec::new();

        for line in kv.lines() {
            let eq_ind = line.find('=');
            match eq_ind {
                None => {
                    continue;
                }
                Some(i) => {
                    let key = &line[..i];
                    let val = &line[i + 2..line.len() - 1];

                    match key {
                        "panic" => {
                            panic = Some(val);
                        }
                        "target_abi" => {
                            if !val.is_empty() {
                                abi = Some(val)
                            }
                        }
                        "target_arch" => {
                            arch = Some(val);
                            // if arches.get(val).is_none() {
                            //     arches.insert(val, target);
                            // }
                        }
                        "target_endian" => endian = Some(val),
                        "target_env" => {
                            if !val.is_empty() {
                                env = Some(val)
                            }
                        }
                        "target_family" => family_group.push(val.to_owned()),
                        "target_feature" => {

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
                        "target_has_atomic" => {
                            has_atomic_group.push(HasAtomicElement::new(val));
                        }
                        "target_os" => {
                            if val != "none" {
                                os = Some(val)
                            }
                        }
                        "target_pointer_width" => width = Some(val),
                        "target_vendor" => {
                            if !val.is_empty() {
                                vendor = Some(val)
                            }
                        }
                        // unstable
                        "relocation_model"
                        | "target_has_atomic_equal_alignment"
                        | "target_has_atomic_load_store" => {
                            //relocation_model = Some(val),
                        }
                        _ => panic!("unknown key: {line}"),
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

        fn insert_group<T: GroupElement>(
            mut group: Vec<T>,
            things: &mut Vec<T>,
            thing_groups: &mut Vec<Vec<T>>,
            group_type: &'static str,
            pub_const_prefix: &'static str,
        ) -> String {
            group.sort_unstable();
            for thing in &group {
                if let Err(i) = things.binary_search_by(|t| t.cmp(thing)) {
                    things.insert(i, thing.clone());
                }
            }

            if group.is_empty() {
                format!("{group_type}::new_const(&[])")
            } else {
                let mut group_str = format!("{group_type}::");
                write_group_str(&mut group_str, group.iter(), pub_const_prefix);

                // Can't compare Vec<String> to Vec<&str> so have to do this comparison.
                if let Err(i) = thing_groups.binary_search_by(|t| t.cmp(&group)) {
                    thing_groups.insert(i, group);
                }

                group_str
            }
        }

        insert(abi, &mut abis);
        insert(arch, &mut arches);
        insert(vendor, &mut vendors);
        insert(os, &mut oses);
        insert(env, &mut envs);
        insert(panic, &mut panics);
        //insert(relocation_model, &mut relocation_models);

        let families_str = insert_group(
            family_group,
            &mut families,
            &mut family_groups,
            "Families",
            "",
        );
        let has_atomics_str = insert_group(
            has_atomic_group,
            &mut has_atomics,
            &mut has_atomic_groups,
            "HasAtomics",
            "atomic_",
        );

        let print_opt = |kind: &str, opt: Option<&str>| {
            if let Some(val) = opt {
                format!("Some({kind}::{val})")
            } else {
                "None".into()
            }
        };

        writeln!(
            out,
            "    TargetInfo {{
        triple: Triple::new_const(\"{triple}\"),
        os: {os},
        abi: {abi},
        arch: Arch::{arch},
        env: {env},
        vendor: {vendor},
        families: {families_str},
        pointer_width: {width},
        endian: Endian::{endian},
        has_atomics: {has_atomics_str},
        panic: Panic::{panic},
    }},",
            triple = target,
            os = print_opt("Os", os),
            abi = print_opt("Abi", abi),
            arch = arch.expect("target had no arch"),
            env = print_opt("Env", env),
            vendor = print_opt("Vendor", vendor),
            width = width.expect("target had no pointer_width"),
            endian = endian.expect("target had no endian"),
            panic = panic.expect("target had no panic"),
            //rel_model = print_opt("RelocationModel", relocation_model),
        )
        .unwrap();
    }

    writeln!(out, "];").unwrap();

    write_impls(&mut out, "Abi", abis);
    write_impls(&mut out, "Arch", arches);
    write_impls(&mut out, "Vendor", vendors);
    write_impls(&mut out, "Os", oses);
    write_impls(&mut out, "Family", families);
    write_group_impls(
        &mut out,
        "Families",
        "__families_",
        "",
        "Family",
        family_groups,
    );
    write_impls(&mut out, "Env", envs);
    // Do not write impls for HasAtomic since it's an enum.
    write_group_impls(
        &mut out,
        "HasAtomics",
        "__has_atomics_",
        "atomic_",
        "HasAtomic",
        has_atomic_groups,
    );
    write_impls(&mut out, "Panic", panics);
    //write_impls(&mut out, "RelocationModel", relocation_models);

    std::fs::write("src/targets/builtins.rs", out)
        .map_err(|e| format!("failed to write target_list.rs: {e}"))?;

    let status = Command::new("rustfmt")
        .args(&["--edition", "2018", "src/targets/builtins.rs"])
        .status()
        .map_err(|e| format!("failed to run rustfmt: {e}"))?;

    if !status.success() {
        return Err(format!("failed to successfully format: {status}"));
    }

    Ok(())
}

fn write_impls(out: &mut String, typ: &'static str, builtins: Vec<String>) {
    writeln!(out, "\nimpl super::{typ} {{").unwrap();

    for thing in builtins {
        writeln!(
            out,
            "pub const {thing}: {typ} = {typ}::new_const(\"{thing}\");"
        )
        .unwrap();
    }

    writeln!(out, "}}").unwrap();
}

trait GroupElement: Clone + Eq + Ord {
    /// The name of the value (e.g. "unix" in Family::unix)
    fn value_expr(&self) -> String;

    /// The name of this element in an identifier.
    fn id_str(&self) -> String;
}

impl GroupElement for String {
    fn value_expr(&self) -> String {
        self.clone()
    }

    fn id_str(&self) -> String {
        self.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HasAtomicElement {
    IntegerSize(u16),
    Pointer,
}

impl HasAtomicElement {
    fn new(input: &str) -> Self {
        if let Ok(val) = input.parse::<u16>() {
            HasAtomicElement::IntegerSize(val)
        } else if input == "ptr" {
            HasAtomicElement::Pointer
        } else {
            panic!("unrecognized input for target_has_atomic: {input}")
        }
    }
}

impl GroupElement for HasAtomicElement {
    fn value_expr(&self) -> String {
        match self {
            Self::IntegerSize(size) => format!("IntegerSize({size})"),
            Self::Pointer => "Pointer".to_owned(),
        }
    }

    fn id_str(&self) -> String {
        match self {
            Self::IntegerSize(size) => size.to_string(),
            Self::Pointer => "ptr".to_owned(),
        }
    }
}

fn write_group_impls<T: GroupElement>(
    out: &mut String,
    group_type: &'static str,
    private_const_prefix: &'static str,
    pub_const_prefix: &'static str,
    element_type: &'static str,
    groups: Vec<Vec<T>>,
) {
    writeln!(out).unwrap();
    for group in &groups {
        write!(out, "const ").unwrap();
        write_group_str(out, group.iter(), private_const_prefix);
        write!(out, ": &[{element_type}] = ").unwrap();
        write!(out, "&[").unwrap();
        for thing in group {
            let value_expr = thing.value_expr();
            writeln!(out, "{element_type}::{value_expr}, ").unwrap();
        }
        writeln!(out, "];").unwrap();
    }

    writeln!(out, "\nimpl super::{group_type} {{").unwrap();

    for group in groups {
        write!(out, "pub const ").unwrap();
        write_group_str(out, group.iter(), pub_const_prefix);
        write!(out, ": {group_type} = {group_type}::new_const(").unwrap();
        write_group_str(out, group.iter(), private_const_prefix);
        writeln!(out, ");").unwrap();
    }

    writeln!(out, "}}").unwrap();
}

fn write_group_str<'a, T: 'a + GroupElement>(
    out: &mut String,
    group: impl Iterator<Item = &'a T> + ExactSizeIterator,
    prefix: &'static str,
) {
    out.push_str(prefix);
    let len = group.len();
    for (idx, thing) in group.into_iter().enumerate() {
        out.push_str(&thing.id_str());
        if idx < len - 1 {
            out.push_str("_");
        }
    }
}

fn main() {
    if let Err(ref e) = real_main() {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
