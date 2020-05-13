use chrono::prelude::*;

fn main() {
    println!(
        "cargo:rustc-env=CARGO_BUILD_DATE={}",
        Local::now().format("%d.%m.%Y")
    );
}
