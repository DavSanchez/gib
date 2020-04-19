use std::path::PathBuf;
use structopt::StructOpt;

const GITIGNORE_FILES: &[(&str, &[u8])] = &include!(concat!(env!("OUT_DIR"), "/gitignore_data.rs"));

#[derive(StructOpt, Debug)]
#[structopt(name = "gib")]
struct Gib {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Print result to stdout
    #[structopt(short, long)]
    show: bool,

    /// Print list of available templates to stdout
    #[structopt(short, long)]
    list: bool,

    /// Output file
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,

    /// Template files to use
    #[structopt(name = "TEMPLATE")]
    templates: Vec<String>,
}

fn main() {
    let opt = Gib::from_args();
    println!("{:#?}", opt);
    if !opt.templates.is_empty() {
        for gitignore in GITIGNORE_FILES {
            if opt.templates[0] == gitignore.0.to_lowercase() {
                println!("###############");
                println!("#     {}",gitignore.0);
                println!("###############");
                println!("{}", String::from_utf8_lossy(gitignore.1));
                break;
            }
        }
    }
}
