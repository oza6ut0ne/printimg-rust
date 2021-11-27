use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "printimg")]
pub struct Opt {
    #[structopt(name = "PATH")]
    pub path: String,

    #[structopt(short, long)]
    pub verbose: bool,

    /// Rotate counterclockwise
    #[structopt(short, long)]
    pub rotate: bool,

    /// Disable limit on image height
    #[structopt(short, long)]
    pub protrude: bool,

    /// Disable super resolution
    #[structopt(short, long)]
    pub flat: bool,
}
