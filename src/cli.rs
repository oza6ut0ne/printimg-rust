use structopt::{clap::AppSettings::DeriveDisplayOrder, StructOpt};

#[derive(StructOpt)]
#[structopt(name = "printimg", setting(DeriveDisplayOrder))]
pub struct Opt {
    #[structopt(name = "PATH")]
    pub path: Option<String>,

    /// Rotate counterclockwise (-r, -rr, -rrr)
    #[structopt(short, long, parse(from_occurrences))]
    pub rotate: u8,

    /// Disable limit on image height
    #[structopt(short, long)]
    pub protrude: bool,

    /// Disable super resolution
    #[structopt(short, long)]
    pub flat: bool,

    /// Do not suppress standard error output
    #[cfg(feature = "opencv")]
    #[structopt(short, long)]
    pub verbose: bool,

    /// Prints build information
    #[structopt(short, long, conflicts_with = "PATH")]
    pub build_info: bool,
}
