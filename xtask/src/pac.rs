//! Different output PACs and their parameters

use std::path::PathBuf;

use clap::ValueEnum;

/// Resulting PACs
#[derive(Clone, ValueEnum)]
pub enum Pac {
    Efr32fg23,
    Efr32fg25,
}

impl Pac {
    /// Get all PACs
    pub fn all() -> Vec<Pac> {
        vec![Pac::Efr32fg23, Pac::Efr32fg25]
    }

    /// Get target string of given PAC
    pub fn target(&self) -> &str {
        match self {
            Pac::Efr32fg23 => "thumbv8m.main-none-eabihf",
            Pac::Efr32fg25 => "thumbv8m.main-none-eabihf",
        }
    }

    /// Get string repr of PAC
    fn base_name(&self) -> &str {
        match self {
            Pac::Efr32fg23 => "efr32fg23",
            Pac::Efr32fg25 => "efr32fg25",
        }
    }

    /// Get SVD source of given PAC
    pub fn svd(&self) -> PathBuf {
        let str = match self {
            Pac::Efr32fg23 => "EFR32FG23/EFR32FG23B010F512IM48.svd",
            Pac::Efr32fg25 => "EFR32FG25/EFR32FG25A021F256IM56.svd",
        };
        PathBuf::from("svd").join(str)
    }

    /// Get the directory of this PA crate
    pub fn dir(&self) -> PathBuf {
        PathBuf::from("pacs").join(self.base_name())
    }

    /// Get the name of this PA crate
    pub fn name(&self) -> String {
        format!("{}-pac", self.base_name())
    }

    /// Get description of this PA crate
    pub fn description(&self) -> String {
        format!(
            "{} Peripheral Access Crate",
            self.base_name().to_uppercase()
        )
    }
}
