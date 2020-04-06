const GITIGNORE_FILES: &[(&str, &[u8])] = &include!(concat!(env!("OUT_DIR"), "/gitignore_data.rs"));

fn main() {
    for (file, data) in GITIGNORE_FILES {
        println!("File {} is {} bytes", file, data.len())
    }
}
