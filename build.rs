use chrono::prelude::*;

//Local::now().format("%Y-%m-%d")
//cargo:rustc-env=FOO=bar

fn main() {
    println!(
        "cargo:rustc-env=CARGO_DATE={}",
        Local::now().format("%d.%m.%Y")
    );
}
