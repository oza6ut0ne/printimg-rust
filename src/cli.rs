use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "printimg")]
pub struct Opt {
    #[structopt(name = "PATH")]
    pub path: String,

    #[structopt(short, long)]
    pub verbose: bool,

    /// Disable super resolution
    #[structopt(short, long)]
    pub flat: bool,
}
