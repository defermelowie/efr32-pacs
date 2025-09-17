//! Functionality for xtask binary

use std::fs::remove_file;

use anyhow::Result;
use itertools::Itertools;
use xshell::{Shell, cmd};

mod pac;
pub use pac::Pac;

pub fn setup(sh: Shell, pacs: &[Pac]) -> Result<()> {
    // Install compile targets
    let targets = pacs.iter().map(|p| p.target()).sorted().dedup();
    cmd!(sh, "rustup target add {targets...}").run()?;

    Ok(())
}

/// Generate rust code from SVD files
pub fn generate(sh: Shell, pacs: &[Pac]) -> Result<()> {
    setup(sh.clone(), pacs)?;

    // Install tools
    let tools = vec![("svd2rust", "0.37.0"), ("form", "0.13.0")];
    for (name, version) in tools {
        cmd!(sh, "cargo install {name} --version {version}").run()?;
    }

    for pac in pacs {
        let svd_path = pac.svd();
        let crate_dir = pac.dir();
        let crate_name = pac.name();
        let lib_path = crate_dir.join("lib.rs");

        cmd!(sh, "cargo new {crate_dir} --name {crate_name} --lib").run()?;
        cmd!(sh, "svd2rust -i {svd_path} -o {crate_dir}").run()?;
        cmd!(sh, "form -i {crate_dir}/lib.rs -o {crate_dir}/src").run()?;
        remove_file(lib_path)?;
    }

    cmd!(sh, "cargo fmt").run()?;
    Ok(())
}

/// Build generated rust code
pub fn build(sh: Shell, pacs: &[Pac]) -> Result<()> {
    setup(sh.clone(), pacs)?;

    for pac in pacs {
        let crate_name = pac.name();
        let target = pac.target();
        cmd!(sh, "cargo build --package {crate_name} --target {target}").run()?;
    }
    Ok(())
}
