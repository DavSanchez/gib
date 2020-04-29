use exitcode;
use itertools::Itertools;
use std::{
    collections::HashMap,
    env,
    fs::OpenOptions,
    io::{self, Write},
    path::PathBuf,
    process,
};
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

    /// Append result to existing .gitignore
    #[structopt(short, long)]
    append: bool,

    /// Replace existing .gitignore with result
    #[structopt(short, long)]
    replace: bool,

    /// Print list of available templates to stdout. Ignores all other flags.
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
    let gitignores: HashMap<&str, (&str, &[u8])> = GITIGNORE_FILES.iter().cloned().collect();
    let opt = Gib::from_args();
    let mut out: Box<dyn Write>;

    if opt.debug {
        println!("Debug flag activated. Data shown below:");
        println!("{:#?}", opt);
    }

    // Check for list flag
    if opt.list {
        for template_key in gitignores.keys().sorted() {
            println!("{}", template_key);
        }
        process::exit(exitcode::OK);
    }

    // Check for show flag
    if !opt.show {
        // Check for out flag
        let output_dir = match opt.output {
            Some(path) => path,
            None => env::current_dir().unwrap(),
        };

        if !output_dir.exists() || !output_dir.is_dir() {
            eprintln!("Error: Output directory does not exist.");
            process::exit(exitcode::OSFILE);
        } else if output_dir.join(".gitignore").exists() && !opt.replace {
            eprintln!("Error: .gitignore file already exists at this location.");
            process::exit(exitcode::CANTCREAT);
        } else if opt.append {
            // let gitignore_file = output_dir.join(".gitignore");
            // out = Box::new(File::create(&gitignore_file).unwrap());
            out = Box::new(
                OpenOptions::new()
                    .append(true)
                    .open(output_dir.join(".gitignore"))
                    .unwrap(),
            );
            println!("Appending to .gitignore file.");
        } else if opt.replace {
            // let gitignore_file = output_dir.join(".gitignore");
            out = Box::new(
                OpenOptions::new()
                    .write(true)
                    .open(output_dir.join(".gitignore"))
                    .unwrap(),
            );
            println!("Replacing .gitignore file.");
        } else {
            // let gitignore_file = output_dir.join(".gitignore");
            out = Box::new(
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(output_dir.join(".gitignore"))
                    .unwrap(),
            );
            println!("Creating .gitignore file.");
        }
    } else {
        out = Box::new(io::stdout());
    }

    if !opt.templates.is_empty() {
        let mut writer_result: Result<(), _>;
        for key in &opt.templates {
            match gitignores.get::<str>(key) {
                Some(content) => {
                    writer_result = gib::write_contents(&mut out, content);
                }
                None => {
                    eprintln!("Error: Unrecognized template.");
                    process::exit(exitcode::DATAERR);
                }
            }
            match writer_result {
                Err(e) => {
                    eprintln!("Error: Could not write to output. {}", e);
                    process::exit(exitcode::IOERR)
                }
                _ => {}
            }
        }
    } else {
        eprintln!("Error: No template arguments provided");
        process::exit(exitcode::USAGE);
    }
    process::exit(exitcode::OK);
}
