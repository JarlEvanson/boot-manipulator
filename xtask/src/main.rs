//! Helper crate for building and testing `boot-manipulator`.

use std::{
    ffi::OsString,
    fmt::{self, Display},
    io,
    path::{Path, PathBuf},
    process::ExitCode,
};

use cli::{get_action, Action, Arch, BuildArguments, Feature, RunArguments};

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

fn run(build_arguments: BuildArguments, run_arguments: RunArguments) -> Result<(), RunError> {
    let arch = build_arguments.arch;

    let boot_manipulator = build_boot_manipulator(build_arguments)?;
    let fat_directory = build_fat_directory(arch, boot_manipulator, &[], &[])
        .map_err(RunError::BuildFatDirectoryError)?;

    run_qemu(arch, &fat_directory, run_arguments)?;

    Ok(())
}

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

impl Display for RunError {
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

fn run_qemu(
    arch: Arch,
    fat_directory: &Path,
    run_arguments: RunArguments,
) -> Result<(), QemuError> {
    let name = match arch {
        Arch::X86_64 => "qemu-system-x86_64",
    };

    let mut cmd = std::process::Command::new(name);

    // Disable unnecessary devices
    cmd.arg("-nodefaults");

    cmd.args(["-boot", "menu=on,splash-time=0"]);
    match arch {
        Arch::X86_64 => {
            // Target fairly modern cpu and machine
            cmd.args(["-machine", "q35"]);
            cmd.args(["-cpu", "max"]);

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

    let mut outputs_path = PathBuf::with_capacity(50);
    outputs_path.push("run");
    outputs_path.push(arch.as_str());
    outputs_path.push("outputs");

    #[cfg(unix)]
    {
        let mode = nix::sys::stat::Mode::from_bits(0o666).unwrap();

        match nix::unistd::mkfifo(&outputs_path.join("serial.in"), mode) {
            Ok(()) => {},
            Err(error) if error == nix::errno::Errno::EEXIST => {},
            Err(error) => todo!("{error}"),
        }

        match nix::unistd::mkfifo(&outputs_path.join("serial.out"), mode) {
            Ok(()) => {},
            Err(error) if error == nix::errno::Errno::EEXIST => {},
            Err(error) => todo!("{error}"),
        }

        cmd.args(["-serial", "pipe:run/x86_64/outputs/serial"]);
    }

    run_cmd(cmd)?;

    #[cfg(unix)]
    {
        std::fs::remove_file(&outputs_path.join("serial.in")).unwrap();
        std::fs::remove_file(&outputs_path.join("serial.out")).unwrap();
    }

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
pub fn build_fat_directory(
    arch: Arch,
    executable_path: PathBuf,
    additional_files: &[(&Path, &str)],
    additional_binary_files: &[(&[u8], &str)],
) -> Result<PathBuf, std::io::Error> {
    let mut fat_directory = PathBuf::with_capacity(50);
    fat_directory.push("run");
    fat_directory.push(arch.as_str());
    fat_directory.push("fat_directory");

    let mut boot_directory = fat_directory.join("EFI");
    boot_directory.push("BOOT");
    if !boot_directory.exists() {
        std::fs::create_dir_all(&boot_directory)?;
    }

    let boot_file_name = match arch {
        Arch::X86_64 => "BOOTX64.EFI",
    };

    std::fs::copy(executable_path, boot_directory.join(boot_file_name))?;

    for &(file, name) in additional_files {
        std::fs::copy(file, fat_directory.join(name))?;
    }

    for &(bytes, name) in additional_binary_files {
        std::fs::write(fat_directory.join(name), bytes)?;
    }

    Ok(fat_directory)
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
