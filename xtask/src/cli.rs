//! Command line parsing and command construction.

/// The action to carry out.
pub enum Action {
    /// Builds `boot-manipulator` and `boot-manipulator-cli`.
    Build(BuildArguments),
}

/// Arguments necessary to determine how to build `boot-manipulator`.
pub struct BuildArguments {
    /// The architecture for which `boot-manipulator` should be built.
    pub arch: Arch,
    /// Whether `boot-manipulator` should be built in release mode.
    pub release: bool,
    /// The features that `boot-manipulator` should have enabled.
    pub features: Vec<Feature>,
}

/// Parses arguments to construct an [`Action`].
pub fn get_action() -> Action {
    let mut matches = command_parser().get_matches();
    let (subcommand_name, mut subcommand_matches) =
        matches.remove_subcommand().expect("subcommand required");
    match subcommand_name.as_str() {
        "build" => Action::Build(parse_build_arguments(&mut subcommand_matches)),
        name => unreachable!("unexpected subcommand {name:?}"),
    }
}

fn parse_build_arguments(matches: &mut clap::ArgMatches) -> BuildArguments {
    let arch = matches
        .remove_one::<Arch>("arch")
        .expect("arch is a required argument");
    let release = matches.remove_one::<bool>("release").unwrap_or(false);
    let features = matches
        .remove_many::<Feature>("features")
        .map(|features| features.collect::<Vec<Feature>>())
        .unwrap_or(Vec::new());

    BuildArguments {
        arch,
        release,
        features,
    }
}

/// Returns the clap command parser.
fn command_parser() -> clap::Command {
    let arch_arg = clap::Arg::new("arch")
        .long("arch")
        .value_parser(clap::builder::EnumValueParser::<Arch>::new())
        .required(true);

    let release_arg = clap::Arg::new("release")
        .help("Build boot-manipulator in release mode")
        .long("release")
        .short('r')
        .action(clap::ArgAction::SetTrue);

    let features_arg = clap::Arg::new("features")
        .help("List of features to active for boot-manipulator")
        .long("features")
        .short('F')
        .value_delimiter(',')
        .action(clap::ArgAction::Append);

    let build_subcommand = clap::Command::new("build")
        .about("Builds boot-manipulator and boot-manipulator-cli")
        .arg(arch_arg.help(
            "The architecture for which boot-manipulator and boot-manipulator-cli should be built",
        ))
        .arg(release_arg)
        .arg(features_arg);

    clap::Command::new("xtask")
        .about("Developer utility for running various tasks in boot-manipulator")
        .subcommand(build_subcommand)
        .subcommand_required(true)
        .arg_required_else_help(true)
}

/// Various features supported by `boot-manipulator`.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Feature {}

impl Feature {
    /// Returns the [`Feature`] in is textual representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            _ => unreachable!(),
        }
    }
}

/// The architectures supported by `boot-manipulator`.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Arch {
    /// The `x86_64` architecture.
    X86_64,
}

impl Arch {
    /// Returns the [`Arch`] as its rustc target triple.
    pub fn as_target_triple(&self) -> &'static str {
        match self {
            Self::X86_64 => "x86_64-unknown-uefi",
        }
    }

    /// Returns the [`Arch`] as its textual representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::X86_64 => "x86_64",
        }
    }
}

impl clap::ValueEnum for Arch {
    fn value_variants<'a>() -> &'a [Self] {
        static ARCHES: &[Arch] = &[Arch::X86_64];

        ARCHES
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(clap::builder::PossibleValue::new(self.as_str()))
    }
}
