use std::{
    env,
    error::Error,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};

const GITIGNORE_DIR: &str = "gitignore";

fn main() -> Result<(), Box<dyn Error>> {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push(GITIGNORE_DIR);

    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("gitignore_data.rs");
    let mut gitignore_data = File::create(&dest_path)?;

    let mut path_vec = Vec::new();

    visit_dirs(&d, &mut path_vec)?;

    writeln!(&mut gitignore_data, r#"["#,)?;
    for path in path_vec {
        let filename = path.file_stem().unwrap().to_str().unwrap();
        let filepath = path.display();
        writeln!(
            &mut gitignore_data,
            r#"("{}", include_bytes!("{}")),"#,
            filename, filepath,
        )?;
    }
    writeln!(&mut gitignore_data, r#"]"#,)?;

    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
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
