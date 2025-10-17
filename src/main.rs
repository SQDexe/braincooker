/* Modules declaration */
mod args;

use {
    anyhow::Result as DynResult,
    clap::Parser,
    std::{
        fs::{
            read_to_string,
            File
            },
        io::Write
        },
    core::hint::unreachable_unchecked,
    crate::args::*,
    braincooker::*
    };

/* Main entrypoint */
fn main() -> DynResult<()> {
    /* Parse CLI arguments */
    let Args {
        command,
        inputs: InputArgs { input, input_file },
        loop_prune,
        debug_display
        } = Args::parse();

    /* Match correct source code input */
    let instr_str = match (input, input_file) {
        /* Raw text input */
        (Some(value), None) =>
            value,
        /* A file path */
        (None, Some(path)) =>
            read_to_string(path)?,
        /* Unsafe note - it is safe, because Clap should disallow any other combination */
        _ => unsafe
            { unreachable_unchecked() }
        };

    /* Get sanitised instructions */
    let mut instr = eval_instr(&instr_str)?;

    /* Prune comment loops according to the settings */
    match loop_prune {
        Some(LoopPrune::One) => {
            let was_prunned = instr.prune_comment_loop();
            if debug_display {
                eprintln!("Info: {} loop was prunned", select!(was_prunned, "A", "No"));
                }
            },
        Some(LoopPrune::All) => {
            let prunned_loops = instr.prune_all_loops();
            if debug_display {
                eprintln!("Info: {prunned_loops} loop(s) was(were) prunned")
                }
            },
        None => ()
        };

    /* Execute matching command */
    match command {
        CMD::Interp { pointer_size, cell_size, display_mode } => {
            /* Construct a builder, and pass the settings */
            let interp_build = Interpreter::builder()
                .display_mode(display_mode)
                .debug_display(debug_display);

            /* Construct a fitting Interpreter, based on arguments */
            let mut interp: Box<dyn InterpRun> = match (pointer_size, cell_size) {
                (DataSize::U8, DataSize::U8) =>
                    Box::new(interp_build.build::<u8, u8>()),
                (DataSize::U8, DataSize::U16) =>
                    Box::new(interp_build.build::<u8, u16>()),
                (DataSize::U8, DataSize::U32) =>
                    Box::new(interp_build.build::<u8, u32>()),

                (DataSize::U16, DataSize::U8) =>
                    Box::new(interp_build.build::<u16, u8>()),
                (DataSize::U16, DataSize::U16) =>
                    Box::new(interp_build.build::<u16, u16>()),
                (DataSize::U16, DataSize::U32) =>
                    Box::new(interp_build.build::<u16, u32>()),

                (DataSize::U32, DataSize::U8) =>
                    Box::new(interp_build.build::<u32, u8>()),
                (DataSize::U32, DataSize::U16) =>
                    Box::new(interp_build.build::<u32, u16>()),
                (DataSize::U32, DataSize::U32) =>
                    Box::new(interp_build.build::<u32, u32>()),
                };

            /* Execute instructions */
            interp.run(instr)?;
            },
        CMD::Comp { output_file } => {
            let mut file = File::create(output_file)?;

            let buf = "Compiled :P".as_bytes();

            file.write_all(buf)?;
            }
        }

    Ok(())
    }


/* Macro for cleaner if-else statements */
#[macro_export]
macro_rules! select {
    ($bool:expr, $truthy:expr, $falsy:expr) => {
        match $bool {
            true => $truthy,
            false => $falsy
            }
        };
    }