use std::{collections::HashMap, io::Write, path::PathBuf};
use itertools::Itertools;
use structopt::StructOpt;

const GITIGNORE_FILES: &[(&str, (&str, &[u8]))] =
    &include!(concat!(env!("OUT_DIR"), "/gitignore_data.rs"));

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

    /*
    /// Append result to existing .gitignore
    #[structopt(short, long)]
    append: bool,

    /// Replace existing .gitignore with result
    #[structopt(short, long)]
    replace: bool,
    */
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

fn main() -> std::io::Result<()> {
    let gitignores: HashMap<&str, (&str, &[u8])> = GITIGNORE_FILES.iter().cloned().collect();
    let opt = Gib::from_args();
    //let mut out: dyn std::io::Write;

    // Check for list
    if opt.list {
        for template_key in gitignores.keys().sorted() {
            println!("{}", template_key);
        }
        return Ok(());
    }

    // Check for show

    // Check for out

    println!("{:#?}", opt);
    if !opt.templates.is_empty() {
        let mut out = std::io::stdout();
        match gitignores.get::<str>(&opt.templates[0]) {
            Some(contents) => {
                writeln!(out, "###############")?;
                writeln!(out, "#    {}", contents.0)?;
                writeln!(out, "###############")?;
                writeln!(out, "{}", String::from_utf8_lossy(contents.1))?;
            }
            None => {}
        }
    }

    Ok(())
}
