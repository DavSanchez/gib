use exitcode;
use itertools::Itertools;
use std::{collections::HashMap, env, fs::File, io::Write, path::PathBuf};
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
        std::process::exit(exitcode::OK);
    }

    // Check for show flag
    if !opt.show {
        // Check for out
        let output_dir = match opt.output {
            Some(path) => path,
            None => env::current_dir().unwrap(),
        };

        // println!("Destination path: {}", output_dir.display());
        // println!("Path exists? {}",output_dir.exists());

        if !output_dir.exists() {
            eprintln!("Error: Output directory does not exist.");
            std::process::exit(exitcode::OSFILE);
        } else if output_dir.join(".gitignore").exists() {
            eprintln!("Error: .gitignore file already exists at this location.");
            std::process::exit(exitcode::CANTCREAT);
        } else {
            let gitignore_file = output_dir.join(".gitignore");
            out = Box::new(File::create(&gitignore_file).unwrap());
        }
    } else {
        out = Box::new(std::io::stdout());
    }

    if !opt.templates.is_empty() {
        match gitignores.get::<str>(&opt.templates[0]) {
            Some(contents) => {
                writeln!(&mut out, "###############").unwrap();
                writeln!(&mut out, "#   {}", contents.0).unwrap();
                writeln!(&mut out, "###############").unwrap();
                writeln!(&mut out, "{}", String::from_utf8_lossy(contents.1)).unwrap();
            }
            None => {
                eprintln!("Error: Unrecognized template(s).");
                std::process::exit(exitcode::DATAERR);
            }
        }
    } else {
        eprintln!("Error: No template arguments provided");
        std::process::exit(exitcode::USAGE);
    }
    println!("Created .gitignore file.");
    std::process::exit(exitcode::OK);
}
