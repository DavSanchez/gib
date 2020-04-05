use std::{
    env,
    error::Error,
    fs::{self, File},
    io::Write,
    path::Path,
};

const GITIGNORE_DIR: &str = "./gitignore/";

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("gitignore_data.rs");
    let mut gitignore_data = File::create(&dest_path)?;

    writeln!(&mut gitignore_data, r#"["#,)?;

    for file in fs::read_dir(GITIGNORE_DIR)? {
        let file = file?;

        if !file.file_type()?.is_file() {
            continue;
        }

        match file.path().extension() {
            Some(ext) => {
                if ext != "gitignore" {
                    continue;
                }
            }
            None => continue,
        }

        match file.path().file_stem() {
            Some(f_name) => {
                writeln!(
                    &mut gitignore_data,
                    r#"("{}", include_bytes!("{}")),"#,
                    f_name.to_str().unwrap().to_lowercase(),
                    file.path().display(),
                )?;
            }
            None => continue,
        }
    }

    writeln!(&mut gitignore_data, r#"];"#,)?;

    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
