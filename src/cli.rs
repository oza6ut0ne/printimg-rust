use structopt::{clap::AppSettings::DeriveDisplayOrder, StructOpt};

#[derive(StructOpt)]
#[structopt(name = "printimg", setting(DeriveDisplayOrder))]
pub struct Opt {
    #[structopt(name = "PATH")]
    pub path: Option<String>,

    /// Rotate counterclockwise
    #[structopt(short, long)]
    pub rotate: bool,

    /// Disable limit on image height
    #[structopt(short, long)]
    pub protrude: bool,

    /// Disable super resolution
    #[structopt(short, long)]
    pub flat: bool,

    #[structopt(short, long)]
    pub verbose: bool,

    /// Prints build information of OpenCV
    #[structopt(short, long, conflicts_with = "PATH")]
    pub build_info: bool,
}
