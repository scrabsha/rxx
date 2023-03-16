use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
    process::Command,
};

use clap::Parser;

#[derive(Debug, Parser)]
struct Options {
    in_path: String,
    #[clap(short)]
    out_path: String,
}

fn main() {
    let Options { in_path, out_path } = Options::parse();

    let tmp_out_path = Path::new(&in_path).with_extension(".s.tmp");

    let success = Command::new("rustc")
        .arg("-o")
        .arg(&tmp_out_path)
        .args(["--emit", "asm"])
        .arg(&in_path)
        .status()
        .unwrap()
        .success();

    assert!(success);

    let generated_asm = fs::read_to_string(&tmp_out_path).unwrap();
    fs::remove_file(tmp_out_path).unwrap();

    let mut out_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&out_path)
        .unwrap();

    let generated_asm = generated_asm.replace('\\', "\\\\").replace('"', "\\\"");

    generated_asm
        .lines()
        .try_for_each(|line| writeln!(out_file, "asm(\"{line}\");"))
        .unwrap();

    let executable_path = Path::new(&out_path).with_extension("");
    let executable_path = executable_path.display();

    let base_root = Command::new("rustc")
        .arg("--print")
        .arg("sysroot")
        .output()
        .unwrap()
        .stdout;

    let base_root = String::from_utf8(base_root).unwrap();
    let base_root = base_root.trim();

    println!("gcc -o {executable_path} {out_path} {base_root}/lib/libstd-*.so")
}
