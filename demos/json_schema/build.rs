//! Build script for the `json_schema` demo crate.
//!
//! Scans `src/versions/` for files named `vN.rs` (where `N` is a version number) and generates
//! two files in `OUT_DIR`:
//!
//! - `versions.rs`: a `pub mod vN;` declaration for each version, ordered numerically by `N`,
//!   included by `src/lib.rs` inside the `versions` module.
//! - `latest.rs`: a `pub use versions::vN as latest;` re-export of the highest-numbered
//!   version, included by `src/lib.rs` at the crate root.
//!
//! This lets new schema versions be added by simply dropping a new `vN.rs` file into
//! `src/versions/`, without editing `src/lib.rs`.

use std::env;
use std::fs;
use std::path::Path;

/// Generates `$OUT_DIR/versions.rs` and `$OUT_DIR/latest.rs` from the versioned schema modules
/// found in `src/versions/`.
///
/// # Side effects
/// - Reads the entries of `src/versions/` (relative to `CARGO_MANIFEST_DIR`).
/// - Writes `$OUT_DIR/versions.rs` and `$OUT_DIR/latest.rs`, which are included by
///   `src/lib.rs`.
/// - Emits a `cargo:rerun-if-changed` directive for `src/versions/` so Cargo reruns this script
///   when its contents change.
///
/// # Panics
/// Panics if the `CARGO_MANIFEST_DIR` or `OUT_DIR` environment variables are not set, if
/// `src/versions/` cannot be read, if it contains no `vN.rs` files, or if the generated files
/// cannot be written.
fn main() {
    // Get current versions directory
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let versions_dir = Path::new(&manifest_dir).join("src").join("versions");
    println!("cargo:rerun-if-changed={}", versions_dir.display());

    // Get list of version modules, ordered numerically by version number
    let mut modules: Vec<(u64, String)> = fs::read_dir(&versions_dir)
        .expect("failed to read src/versions directory")
        .map(|entry| entry.expect("failed to read directory entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "rs"))
        .filter_map(|path| {
            let name = path.file_stem()?.to_str()?.to_string();
            let number = name.strip_prefix('v')?.parse::<u64>().ok()?;
            Some((number, name))
        })
        .collect();
    modules.sort_by_key(|(number, _)| *number);

    // Generate a `pub mod vN;` declaration for each version module
    let mut versions = String::new();
    for (_, name) in &modules {
        let module_path = versions_dir
            .join(format!("{name}.rs"))
            .to_str()
            .unwrap()
            .replace('\\', "/");
        versions.push_str(&format!("#[path = \"{module_path}\"]\n"));
        versions.push_str(&format!("pub mod {name};\n"));
    }

    // Re-export the highest-numbered version module as `latest`
    let (_, latest_name) = modules
        .last()
        .expect("src/versions must contain at least one vN.rs file");
    let latest = [
        format!("/// Alias for the latest schema version (`{latest_name}`).\n"),
        format!("pub use versions::{latest_name} as latest;\n"),
    ]
    .concat();

    // Write both files to out dir
    let out_dir = env::var("OUT_DIR").unwrap();
    let versions_file = Path::new(&out_dir).join("versions.rs");
    let latest_file = Path::new(&out_dir).join("latest.rs");
    fs::write(versions_file, versions).expect("failed to write versions.rs");
    fs::write(latest_file, latest).expect("failed to write latest.rs");
}
