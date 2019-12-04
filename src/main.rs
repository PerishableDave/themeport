mod parsers;

use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use structopt::StructOpt;
use crate::parsers::iterm;


#[derive(StructOpt, Debug)]
#[structopt(name = "themeport")]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

struct TermColor {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let file = File::open(opt.input)?;
    let reader = BufReader::new(file);

    iterm::parse(reader);

    Ok(())
}
