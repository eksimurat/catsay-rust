extern crate colored;
extern crate structopt;

use colored::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meow!")]
    /// What does the cat say?
    message: String,

    #[structopt(short = "d", long = "dead")]
    /// Make the cat appear dead
    dead: bool,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Load the cat figure from a specified file
    catfile: Option<std::path::PathBuf>,
}

macro_rules! print_cat {
    ($options:expr) => {
        let eye = if $options.dead { "X" } else { "O" };
        println!("{}", $options.message);

        match $options.catfile {
            Some(path) => {
                let cat_template = std::fs::read_to_string(&path)
                    .expect(&format!("could not read file {:?}", path));
                let cat_picture = cat_template.replace("{eye}", eye);
                println!("{}", &cat_picture);
            }
            None => {
                println!(" \\");
                println!("  \\");
                println!("     /\\_/\\");
                println!("    ( {eye} {eye} )", eye = eye.red().bold());
                println!("    =( I )=")
            }
        }
    };
}

fn main() {
    let options = Options::from_args();
    print_cat!(options);
}
