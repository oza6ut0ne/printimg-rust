use clap::{AppSettings::DeriveDisplayOrder, Parser};

#[derive(Parser)]
#[clap(name = "printimg", version, setting(DeriveDisplayOrder))]
pub struct Opt {
    #[clap(name = "PATH")]
    pub path: Option<String>,

    /// Rotate counterclockwise (-r, -rr, -rrr)
    #[clap(short, long, parse(from_occurrences))]
    pub rotate: u8,

    /// Disable limit on image height
    #[clap(short, long)]
    pub protrude: bool,

    /// Disable super resolution
    #[clap(short, long)]
    pub flat: bool,

    /// Do not suppress standard error output
    #[cfg(feature = "opencv")]
    #[clap(short, long)]
    pub verbose: bool,

    /// Prints build information
    #[clap(short, long, conflicts_with = "PATH")]
    pub build_info: bool,
}
