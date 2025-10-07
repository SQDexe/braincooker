use {
    clap::*,
    std::path::PathBuf,
    crate::types::*
    };


#[derive(Parser)]
#[command(author, about, version, propagate_version = true)]
#[group(id = "inputs", required = true, multiple = false)]
pub struct Args {
    /// Raw input wrapped inside "..."
    #[clap(group = "inputs")]
    pub input: Option<String>,
    /// Path to a file with Brainfuck code
    #[clap(short, long, group = "inputs")]
    pub input_file: Option<PathBuf>,
    #[command(subcommand)]
    pub command: CMD,
    }

#[derive(Subcommand)]
pub enum CMD {
    /// Run Brainfuck code with interpreter
    Interp(InterpArgs),
    /// Compile Brainfuck code into executable file
    Comp
    }

#[derive(Args)]
pub struct InterpArgs {
    #[clap(short, long, value_enum, default_value_t = DataSize::U16)]
    pub pointer_size: DataSize,
    #[clap(short, long, value_enum, default_value_t = DataSize::U8)]
    pub cell_size: DataSize,
    #[clap(short, long, value_enum, default_value_t = DisplayMode::ASCII)]
    pub display_mode: DisplayMode
    }