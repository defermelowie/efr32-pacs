//! Binary for [cargo-xtask](https://github.com/matklad/cargo-xtask) pattern.

#[derive(clap::Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
    #[arg(short, long)]
    pac: Option<xtask::Pac>,
}

#[derive(clap::Subcommand)]
enum Cmd {
    /// Prepare environment
    Setup,
    /// Generate rust code from SVDs
    Generate,
    /// Build generated rust code
    Build,
    /// Test generated rust code
    Test,
}

fn main() -> anyhow::Result<()> {
    let cli = <Cli as clap::Parser>::parse();
    let pacs = cli.pac.map_or_else(|| xtask::Pac::all(), |p| vec![p]);
    let sh = xshell::Shell::new()?;
    match cli.command {
        Cmd::Setup => xtask::setup(sh, &pacs)?,
        Cmd::Generate => xtask::generate(sh, &pacs)?,
        Cmd::Build => xtask::build(sh, &pacs)?,
        Cmd::Test => xtask::test(sh, &pacs)?,
    };
    Ok(())
}
