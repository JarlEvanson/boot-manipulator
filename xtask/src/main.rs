//! Helper crate for building and testing `boot-manipulator`.

use std::process::ExitCode;

use cli::{get_action, Action};

pub mod cli;

fn main() -> ExitCode {
    match get_action() {
        Action::Build(_) => todo!(),
    }
}
