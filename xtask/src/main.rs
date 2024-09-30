//! Helper crate for building and testing `boot-manipulator`.

use std::{
    ffi::OsString,
    fmt, io,
    path::{Path, PathBuf},
    process::ExitCode,
};

use cli::{
    binary_suffix, get_action, target_triple, Action, Arch, BuildArguments, Feature, RunArguments,
};

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
        Action::Run {
            build_arguments,
            run_arguments,
        } => match run(build_arguments, run_arguments) {
            Ok(()) => {}
            Err(error) => {
                eprintln!("{error}");
                return ExitCode::FAILURE;
            }
        },
    }

    ExitCode::SUCCESS
}

/// Build `boot-manipulator` for the specified [`Arch`] with the specified [`Feature`]s.
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

/// Build `boot-manipulator` for the specified [`Arch`] with the specified [`Feature`]s and then
/// run it in QEMU.
fn run(build_arguments: BuildArguments, run_arguments: RunArguments) -> Result<(), RunError> {
    let arch = build_arguments.arch;

    let boot_manipulator = build_boot_manipulator(build_arguments)?;
    let fat_directory =
        build_fat_directory(arch, &boot_manipulator).map_err(RunError::BuildFatDirectoryError)?;

    run_qemu(arch, &fat_directory, run_arguments)?;

    Ok(())
}

/// Various errors that can occur while building and running `boot-manipulator`.
#[derive(Debug)]
enum RunError {
    /// An error occurred while building `boot_manipulator`.
    BuildFailed(BuildError),
    /// An error occurred while building the FAT directory.
    BuildFatDirectoryError(std::io::Error),
    /// An error occurred while running QEMU.
    QemuError(QemuError),
}

impl From<BuildError> for RunError {
    fn from(value: BuildError) -> Self {
        Self::BuildFailed(value)
    }
}

impl From<QemuError> for RunError {
    fn from(value: QemuError) -> Self {
        Self::QemuError(value)
    }
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BuildFailed(error) => error.fmt(f),
            Self::BuildFatDirectoryError(error) => {
                write!(f, "error while building FAT directory: {error}")
            }
            Self::QemuError(error) => error.fmt(f),
        }
    }
}

/// Runs QEMU with the given folder as an emulated FAT directory.
fn run_qemu(
    arch: Arch,
    fat_directory: &Path,
    run_arguments: RunArguments,
) -> Result<(), QemuError> {
    let name = match arch {
        Arch::X86 => "qemu-system-i386",
        Arch::X86_64 => "qemu-system-x86_64",
    };

    let mut cmd = std::process::Command::new(name);

    // Disable unnecessary devices
    cmd.arg("-nodefaults");

    cmd.args(["-boot", "menu=on,splash-time=0"]);
    match arch {
        Arch::X86 | Arch::X86_64 => {
            // Target a fairly modern cpu and machine
            cmd.args(["-cpu", "max"]);
            cmd.args(["-machine", "q35"]);

            // Allocate a little memory.
            cmd.args(["-m", "512M"]);

            // Use VGA graphics as the windowing interface.
            cmd.args(["-vga", "std"]);

            if std::env::consts::OS == "linux" {
                cmd.arg("-enable-kvm");
            }
        }
    }

    // Use OVMF code file.
    let mut ovmf_code_arg = OsString::from("if=pflash,format=raw,readonly=on,file=");
    ovmf_code_arg.push(run_arguments.ovmf_code);
    cmd.arg("-drive").arg(ovmf_code_arg);

    // Use OVMF vars file.
    let mut ovmf_vars_arg = OsString::from("if=pflash,format=raw,readonly=on,file=");
    ovmf_vars_arg.push(run_arguments.ovmf_vars);
    cmd.arg("-drive").arg(ovmf_vars_arg);

    // Use the given `fat_directory`.
    let mut fat_drive_arg = OsString::from("format=raw,file=fat:rw:");
    fat_drive_arg.push(fat_directory);
    cmd.arg("-drive").arg(fat_drive_arg);

    run_cmd(cmd)?;

    Ok(())
}

/// Various errors that can occur while running QEMU.
#[derive(Debug)]
pub struct QemuError(RunCommandError);

impl From<RunCommandError> for QemuError {
    fn from(value: RunCommandError) -> Self {
        Self(value)
    }
}

impl fmt::Display for QemuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error while running QEMU: {}", self.0)
    }
}

/// Sets up the FAT directory used for UEFI.
///
/// # Errors
/// Returns any [`io::Error`]s that occur.
pub fn build_fat_directory(arch: Arch, boot_manipulator: &Path) -> Result<PathBuf, io::Error> {
    let mut fat_directory = PathBuf::with_capacity(50);
    fat_directory.push("run");
    fat_directory.push(arch.as_str());
    fat_directory.push("fat_directory");

    std::fs::create_dir_all(&fat_directory)?;

    std::fs::copy(boot_manipulator, fat_directory.join("boot-manipulator.efi"))?;

    Ok(fat_directory)
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
