//! Command line parsing and [`Action`] construction.

/// The action to carry out.
pub enum Action {
    /// Builds `boot-manipulator` and `boot-manipulator-cli`.
    Build(BuildArguments),
}

/// Arguments necessary to determine how to build `boot-manipulator`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BuildArguments {
    /// The architecture for which `boot-manipulator` should be built.
    pub arch: Arch,
    /// The platform for which `boot-manipulator` should be built.
    pub platform: Platform,
    /// Whether `boot-manipulator` should be built in release mode.
    pub release: bool,
    /// The features that `boot-manipulator` should have enabled.
    pub features: Vec<Feature>,
}

/// Parses arguments to construct an [`Action`].
#[expect(clippy::missing_panics_doc)]
pub fn get_action() -> Action {
    let mut matches = command_parser().get_matches();
    let (subcommand_name, mut subcommand_matches) =
        matches.remove_subcommand().expect("subcommand required");
    match subcommand_name.as_str() {
        "build" => Action::Build(parse_build_arguments(&mut subcommand_matches)),
        name => unreachable!("unexpected subcommand {name:?}"),
    }
}

/// Extracts build arguments from the given parsed arguments.
fn parse_build_arguments(matches: &mut clap::ArgMatches) -> BuildArguments {
    let arch = matches
        .remove_one::<Arch>("arch")
        .expect("arch is a required argument");
    let platform = matches
        .remove_one::<Platform>("platform")
        .expect("platform is a required argument");

    let release = matches.remove_one::<bool>("release").unwrap_or(false);
    let features = matches
        .remove_many::<Feature>("features")
        .map(|features| features.collect::<Vec<Feature>>())
        .unwrap_or_default();

    BuildArguments {
        arch,
        platform,
        release,
        features,
    }
}

/// Returns the clap command parser.
fn command_parser() -> clap::Command {
    let arch_arg = clap::Arg::new("arch")
        .help("The architecture for which this subcommand should run")
        .long("arch")
        .value_parser(clap::builder::EnumValueParser::<Arch>::new())
        .required(true);

    let platform_arg = clap::Arg::new("platform")
        .help("The platform for which this subcommand should run")
        .long("platform")
        .value_parser(clap::builder::EnumValueParser::<Platform>::new())
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
        .value_parser(clap::builder::EnumValueParser::<Feature>::new())
        .action(clap::ArgAction::Append);

    let build_subcommand = clap::Command::new("build")
        .about("Builds boot-manipulator and boot-manipulator-cli")
        .arg(arch_arg)
        .arg(platform_arg)
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
    /// Returns the [`Feature`] in its textual representation.
    pub fn as_str(&self) -> &'static str {
        unreachable!()
    }
}

impl clap::ValueEnum for Feature {
    fn value_variants<'a>() -> &'a [Self] {
        /// List of all of the supported features.
        static FEATURES: &[Feature] = &[];

        FEATURES
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(clap::builder::PossibleValue::new(self.as_str()))
    }
}

/// Returns the target triple for the pair of [`Arch`] and [`Platform`].
pub fn target_triple(arch: Arch, platform: Platform) -> &'static str {
    match (arch, platform) {
        (Arch::X86_64, Platform::Uefi) => "x86_64-unknown-uefi",
        (Arch::X86, Platform::Uefi) => "i686-unknown-uefi",
    }
}

/// Returns the suffix for the binary, if one exists for the pair of the [`Arch`] and [`Platform`].
pub fn binary_suffix(arch: Arch, platform: Platform) -> Option<&'static str> {
    match (arch, platform) {
        (Arch::X86_64 | Arch::X86, Platform::Uefi) => Some("efi"),
    }
}

/// The architectures supported by `boot-manipulator`.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Arch {
    /// The `x86` architecture.
    X86,
    /// The `x86_64` architecture.
    X86_64,
}

impl Arch {
    /// Returns the [`Arch`] as its textual representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::X86 => "x86",
            Self::X86_64 => "x86_64",
        }
    }
}

impl clap::ValueEnum for Arch {
    fn value_variants<'a>() -> &'a [Self] {
        /// A list of all of the supported architectures.
        static ARCHES: &[Arch] = &[Arch::X86, Arch::X86_64];

        ARCHES
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(clap::builder::PossibleValue::new(self.as_str()))
    }
}

/// The platforms supported by `boot-manipulator`.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Platform {
    /// The UEFI platform.
    Uefi,
}

impl Platform {
    /// Returns the [`Platform`] as its textual representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Uefi => "uefi",
        }
    }
}

impl clap::ValueEnum for Platform {
    fn value_variants<'a>() -> &'a [Self] {
        /// A list of all of the supported platforms.
        static PLATFORMS: &[Platform] = &[Platform::Uefi];

        PLATFORMS
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(clap::builder::PossibleValue::new(self.as_str()))
    }
}
