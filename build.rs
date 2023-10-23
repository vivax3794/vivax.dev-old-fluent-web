use std::process;

fn main() {
    println!("cargo:rerun-if-changed=src_fluent");

    process::Command::new("fluent_web").output().unwrap();
}
