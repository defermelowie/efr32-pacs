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

/// Create index.html for GitHub Pages
pub fn create_index(pacs: &[Pac]) -> Result<()> {
    use std::fs;

    let mut html = String::from(r#"<!DOCTYPE html>
<html>
<head>
    <title>EFR32 PACs Documentation</title>
    <meta charset="utf-8">
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        h1 { color: #333; }
        ul { list-style-type: none; padding: 0; }
        li { margin: 10px 0; }
        a { text-decoration: none; color: #0366d6; font-size: 18px; }
        a:hover { text-decoration: underline; }
        .description { color: #666; font-size: 14px; margin-left: 20px; }
    </style>
</head>
<body>
    <h1>EFR32 PACs Documentation</h1>
    <p>Generated Rust documentation for EFR32 Peripheral Access Crates (PACs)</p>
    <ul>
"#);

    for pac in pacs {
        let crate_name = pac.name();
        let doc_name = crate_name.replace('-', "_");
        let description = match pac {
            Pac::Efr32fg23 => "EFR32FG23 Peripheral Access Crate",
        };

        html.push_str(&format!(
            r#"        <li><a href="{doc_name}/">{crate_name}</a><div class="description">{description}</div></li>
"#
        ));
    }

    html.push_str(r#"    </ul>
</body>
</html>
"#);

    fs::create_dir_all("target/doc")?;
    fs::write("target/doc/index.html", html)?;

    println!("Created index.html for {} PACs", pacs.len());
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

/// Build rustdoc for generated rust code
pub fn doc(sh: Shell, pacs: &[Pac]) -> Result<()> {
    setup(sh.clone(), pacs)?;

    for pac in pacs {
        let crate_name = pac.name();
        let target = pac.target();
        cmd!(sh, "cargo doc --package {crate_name} --target {target}").run()?;
    }

    // Automatically create index.html for GitHub Pages
    create_index(pacs)?;

    Ok(())
}
