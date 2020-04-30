use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    process,
    str::EscapeDefault,
};

const GITIGNORE_DIR: &str = "gitignore";

fn main() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push(GITIGNORE_DIR);

    let mut path_vec = Vec::new();

    visit_dirs(&d, &mut path_vec).expect("Could not navigate templates directory.");
    if path_vec.is_empty() {
        eprintln!(
            "Could not get any templates. \
             Please open an issue at \
             https://github.com/DavSanchez/gib/issues/new"
        );
        process::exit(1);
    }

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR variable not defined.");
    let dest_path = Path::new(&out_dir).join("gitignore_data.rs");
    let mut gitignore_data = File::create(&dest_path).unwrap();

    writeln!(&mut gitignore_data, r#"["#,).unwrap();
    for path in path_vec {
        let (filename, filepath) = match extract_escaped_filename(&path) {
            Some((filename, filepath)) => (filename, filepath),
            None => continue,
        };
        writeln!(
            &mut gitignore_data,
            r###"("{}", ("{}", include_bytes!("{}"))),"###,
            filename.to_lowercase(),
            filename,
            filepath,
        ).unwrap();
    }
    writeln!(&mut gitignore_data, r#"]"#,).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}

fn extract_escaped_filename(path: &PathBuf) -> Option<(&str, EscapeDefault)> {
    let filename = path.file_stem()?.to_str()?;
    let filepath = path.to_str()?;
    Some((filename, filepath.escape_default()))
}

fn visit_dirs(dir: &Path, path_vec: &mut Vec<PathBuf>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, path_vec)?;
            } else {
                match path.extension() {
                    Some(ext) => {
                        if ext == "gitignore" {
                            path_vec.push(path)
                        }
                    }
                    None => continue,
                }
            }
        }
    }
    Ok(())
}
