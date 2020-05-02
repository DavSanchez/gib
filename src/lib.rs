use exitcode;
use itertools::Itertools;
use std::{
    collections::HashMap,
    env,
    fs::{File, OpenOptions},
    io::{self, Write},
    path::PathBuf,
};
use structopt::StructOpt;

enum FileMode {
    Create,
    Append,
    Replace,
}

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

pub fn gib_cli() -> Result<(), i32> {
    let gitignores: HashMap<&str, (&str, &[u8])> = GITIGNORE_FILES.iter().cloned().collect();
    let opt = Gib::from_args();

    if gitignores.is_empty() {
        return error_exit(
            "Templates unavailable. \
            Please file a bug: \
            https://github.com/DavSanchez/gib/issues/new",
            exitcode::CONFIG,
        );
    }

    // Check for list flag
    if opt.list {
        return output_list(gitignores);
    }

    let mut out: Box<dyn Write> = Box::new(io::stdout());

    // Check for show flag
    if !opt.show {
        // Check for out flag
        let file_mode: FileMode;
        let output_dir = match opt.output {
            Some(path) => path,
            None => env::current_dir().unwrap(),
        };

        if !output_dir.exists() || !output_dir.is_dir() {
            return error_exit("Output directory does not exist", exitcode::OSFILE);
        } else if output_dir.join(".gitignore").exists() && !(opt.replace || opt.append) {
            return error_exit(
                ".gitignore file already exists at this location",
                exitcode::CANTCREAT,
            );
        } else if opt.append {
            file_mode = FileMode::Append;
        } else if opt.replace {
            file_mode = FileMode::Replace;
        } else {
            file_mode = FileMode::Create;
        }

        match open_gitignore_mode(output_dir, file_mode) {
            Ok(file) => out = Box::new(file),
            Err(e) => {
                return error_exit(
                    &format!("Could not create or open file. {}", e),
                    exitcode::IOERR,
                )
            }
        }
    }

    if !opt.templates.is_empty() {
        let mut writer_result: Result<_, _>;
        for key in &opt.templates {
            match gitignores.get::<str>(key) {
                Some(content) => {
                    writer_result = write_contents(&mut out, content);
                }
                None => {
                    return error_exit(&format!("Unrecognized template {}", key), exitcode::DATAERR)
                }
            }
            if let Err(_) = writer_result {
                return error_exit("Could not write output. Aborting", exitcode::IOERR);
            }
        }
        if let Err(e) = out.flush() {
            return error_exit(
                &format!("Could not flush the buffer. {}", e),
                exitcode::IOERR,
            );
        }
        if !opt.show {
            println!("Created .gitignore file.");
        }
    } else {
        return error_exit("No template arguments provided", exitcode::USAGE);
    }

    Ok(())
}

fn output_list(gitignores: HashMap<&str, (&str, &[u8])>) -> Result<(), i32> {
    for template_key in gitignores.keys().sorted() {
        println!("{}", template_key);
    }
    Ok(())
}

fn open_gitignore_mode(path: PathBuf, mode: FileMode) -> Result<File, std::io::Error> {
    let mut file_options = OpenOptions::new();

    match mode {
        FileMode::Create => file_options.write(true).create_new(true),
        FileMode::Append => file_options.append(true),
        FileMode::Replace => file_options.write(true),
    };

    file_options.open(path.join(".gitignore"))
}

fn write_contents(
    mut writer: impl std::io::Write,
    content: &(&str, &[u8]),
) -> Result<(), std::io::Error> {
    writeln!(writer, "###############")?;
    writeln!(writer, "#   {}", content.0)?;
    writeln!(writer, "###############")?;
    writeln!(writer, "{}", String::from_utf8_lossy(content.1))?;
    Ok(())
}

fn error_exit(error: &str, code: exitcode::ExitCode) -> Result<(), i32> {
    eprintln!("Error: {}.", error);
    Err(code)
}
