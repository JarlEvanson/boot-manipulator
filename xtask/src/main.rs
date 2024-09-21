//! Helper crate for building and testing `boot-manipulator`.

use std::{
    fmt::{self, Display},
    io,
    path::PathBuf,
    process::ExitCode,
};

use cli::{get_action, Action, BuildArguments, Feature};

pub mod cli;

fn main() -> ExitCode {
    match get_action() {
        Action::Build(arguments) => match build_boot_manipulator(arguments) {
            Ok(path) => println!("boot-manipulator located at \"{}\"", path.display()),
            Err(error) => {
                eprintln!("{error}");
                return ExitCode::FAILURE;
            }
        },
    }

    ExitCode::SUCCESS
}

fn build_boot_manipulator(arguments: BuildArguments) -> Result<PathBuf, BuildError> {
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("build");
    cmd.args(["--package", "boot-manipulator"]);

    cmd.args(["--target", arguments.arch.as_target_triple()]);
    if arguments.release {
        cmd.arg("--release");
    }

    if !arguments.features.is_empty() {
        let features = arguments
            .features
            .iter()
            .map(Feature::as_str)
            .collect::<Vec<_>>()
            .join(",");

        cmd.args(["--features", &features]);
    }

    let mut binary_location = PathBuf::with_capacity(50);
    binary_location.push("target");
    binary_location.push(arguments.arch.as_target_triple());
    if arguments.release {
        binary_location.push("release");
    } else {
        binary_location.push("debug");
    }
    binary_location.push("boot-manipulator.efi");

    run_cmd(cmd)?;

    Ok(binary_location)
}

#[derive(Debug)]
struct BuildError(RunCommandError);

impl From<RunCommandError> for BuildError {
    fn from(value: RunCommandError) -> Self {
        Self(value)
    }
}

impl Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error while building boot-manipulator: {}", self.0)
    }
}

/// Runs a [`Command`][c], handling non-zero exit codes and other failures.
///
/// [c]: std::process::Command
pub fn run_cmd(mut cmd: std::process::Command) -> Result<(), RunCommandError> {
    println!("Running command: {cmd:?}");

    let status = cmd.status()?;
    if !status.success() {
        return Err(RunCommandError::CommandFailed {
            code: status.code(),
        });
    }

    Ok(())
}

/// Various errors that can occur while running a command.
#[derive(Debug)]
pub enum RunCommandError {
    /// An error occurred while launching the command.
    ProcessError(io::Error),
    /// The command exited with a non-zero exit code.
    CommandFailed {
        /// The exit of code of the command.
        code: Option<i32>,
    },
}

impl From<io::Error> for RunCommandError {
    fn from(value: io::Error) -> Self {
        Self::ProcessError(value)
    }
}

impl Display for RunCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProcessError(error) => write!(f, "error launching command: {error}"),
            Self::CommandFailed { code: Some(code) } => {
                write!(f, "command failed with exit status {code}")
            }
            Self::CommandFailed { code: None } => write!(f, "command terminated by signal"),
        }
    }
}
