//! Helper crate for building and testing `boot-manipulator`.

use std::{fmt, io, path::PathBuf, process::ExitCode};

use cli::{binary_suffix, get_action, target_triple, Action, BuildArguments, Feature};

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

/// Build `boot-manipulator` for the specified [`Arch`][a] with the specified [`Feature`]s.
///
/// [a]: crate::cli::Arch
fn build_boot_manipulator(arguments: BuildArguments) -> Result<PathBuf, BuildError> {
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("build");
    cmd.args(["--package", "boot-manipulator"]);

    let target_triple = target_triple(arguments.arch, arguments.platform);

    cmd.args(["--target", target_triple]);
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
    binary_location.push(target_triple);
    if arguments.release {
        binary_location.push("release");
    } else {
        binary_location.push("debug");
    }
    let mut binary_name = "boot-manipulator".to_owned();
    if let Some(binary_suffix) = binary_suffix(arguments.arch, arguments.platform) {
        binary_name.push('.');
        binary_name.push_str(binary_suffix);
    }
    binary_location.push(binary_name);

    run_cmd(cmd)?;

    Ok(binary_location)
}

/// Various errors that can occur while building `boot-manipulator`.
#[derive(Debug)]
struct BuildError(RunCommandError);

impl From<RunCommandError> for BuildError {
    fn from(value: RunCommandError) -> Self {
        Self(value)
    }
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error while building boot-manipulator: {}", self.0)
    }
}

/// Runs a [`Command`][c], handling non-zero exit codes and other failures.
///
/// # Errors
/// - [`RunCommandError::ProcessError`] is returned when an error occurs while launching the
///     command.
/// - [`RunCommandError::CommandFailed`] is returned when an error ocurrs while running the
///     command.
///
/// [c]: std::process::Command
fn run_cmd(mut cmd: std::process::Command) -> Result<(), RunCommandError> {
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
enum RunCommandError {
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

impl fmt::Display for RunCommandError {
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
