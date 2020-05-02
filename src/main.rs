use exitcode;
use std::process;

fn main() {
    process::exit(match gib::gib_cli() {
        Ok(_) => exitcode::OK,
        Err(code) => code,
    });
}
