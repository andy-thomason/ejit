
use ejit_build::{build_aarch_base, build_aarch_vector, build_x86_base, build_x86_vector};

fn main() {
    if std::env::var("EJIT_BUILD_AARCH64").map(|v| v == "1").unwrap_or_default() {
        build_aarch_base();
        build_aarch_vector();
    }
    if std::env::var("EJIT_BUILD_X86_64").map(|v| v == "1").unwrap_or_default() {
        build_x86_base();
        build_x86_vector();
    }
}
