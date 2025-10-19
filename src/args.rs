use {
    clap::*,
    std::path::PathBuf,
    braincooker::DisplayMode
    };


/* Value prunning settings */
#[derive(Clone, Copy, ValueEnum)]
pub enum LoopPrune {
    One,
    All
    }

/* Pointer, and cell size */
#[derive(Clone, Copy, ValueEnum)]
pub enum DataSize {
    U8,
    U16,
    U32
    }

// #[derive(Clone, Copy, PartialEq)]
// pub enum Arch {
//     X86_64,
//     Arm64,
//     RiscV64,
//     }


#[derive(Parser)]
#[command(author, about, version, propagate_version = true)]
pub struct Args {
    /// Action to be executed
    #[command(subcommand)]
    pub command: CMD,
    }

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct Inputs {
    /// Raw source code
    pub input: Option<String>,
    /// Path to a file with source code
    #[clap(short, long)]
    pub input_file: Option<PathBuf>,
    }

#[derive(Args)]
pub struct Settings {
    /// Whether to show progress informations
    #[clap(short = 'D', long, action)]
    pub debug_display: bool,
    /// Whether to prune comment loops
    #[clap(short, long, value_enum)]
    pub loop_prune: Option<LoopPrune>
    }

#[derive(Subcommand)]
pub enum CMD {
    /// Run Brainfuck code with interpreter
    Interp {
        /// Possible input sources
        #[clap(flatten)]
        inputs: Inputs,
        /// General settings
        #[clap(flatten)]
        settings: Settings,
        /// Pointer size, number of cells
        #[clap(short, long, value_enum, default_value_t = DataSize::U16)]
        pointer_size: DataSize,
        /// Cell size
        #[clap(short, long, value_enum, default_value_t = DataSize::U8)]
        cell_size: DataSize,
        /// Way of displaying value of a cell
        #[clap(short, long, value_enum, default_value_t = DisplayMode::ASCII)]
        display_mode: DisplayMode
        },
    /// Compile Brainfuck code into executable file
    Comp {
        /// Possible input sources
        #[clap(flatten)]
        inputs: Inputs,
        /// General settings
        #[clap(flatten)]
        settings: Settings,
        /// Output file path
        #[clap(short, long)]
        output_file: PathBuf
        }
    }


impl CMD {
    /* Getters */
    pub const fn get_inputs(&self) -> &Inputs {
        match self {
            CMD::Interp { inputs, .. } => inputs,
            CMD::Comp { inputs, .. } => inputs
            }
        }
    pub const fn get_settings(&self) -> &Settings {
        match self {
            CMD::Interp { settings, .. } => settings,
            CMD::Comp { settings, .. } => settings
            }
        }
    }