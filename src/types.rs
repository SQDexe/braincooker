use clap::ValueEnum;


/* Value visualisation mode */
#[derive(Clone, Copy, ValueEnum)]
pub enum DisplayMode {
    ASCII,
    Numeric
    }

/* Pointer, and cell size type */
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