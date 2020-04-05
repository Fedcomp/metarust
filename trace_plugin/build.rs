use chrono::prelude::*;

fn main() {
    println!(
        "cargo:rustc-env=CARGO_DATE={}",
        Local::now().format("%d.%m.%Y")
    );
}
