/* Modules declaration */
mod args;
mod eval;
mod interp;
mod tape;
mod types;
mod utils;


use {
    anyhow::Result as DynResult,
    clap::Parser,
    std::fs::read_to_string,
    core::hint::unreachable_unchecked,
    crate::{
        args::*,
        eval::*,
        interp::*,
        types::*
        }
    };

/* Main entrypoint */
fn main() -> DynResult<()> {
    /* Parse CLI arguments */
    let Args { input, input_file, command } = Args::parse();

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
    let instr = eval_instr(&instr_str)?;

    /* Execute matching command */
    match command {
        CMD::Interp(args) => 
            run_interp(instr, args),
        CMD::Comp =>
            run_comp()
        }
    }

/* Interpreter execution */
fn run_interp(mut instr: InstructionSet, args: InterpArgs) -> DynResult<()> {
    let InterpArgs { pointer_size, cell_size, display_mode } = args;

    instr.prune_all_loops();

    let interp_build = InterpreterBuilder::new()
        .display_mode(display_mode);

    /* Construct a fitting Interpreter, based on arguments */
    let mut interp: Box<dyn InterpRun> = match (pointer_size, cell_size) {
        (DataSize::U8, DataSize::U8) =>
            Box::new(interp_build.build::<u8, u8>()),
        (DataSize::U16, DataSize::U8) =>
            Box::new(interp_build.build::<u16, u8>()),
        (DataSize::U32, DataSize::U8) =>
            Box::new(interp_build.build::<u32, u8>()),

        (DataSize::U8, DataSize::U16) =>
            Box::new(interp_build.build::<u8, u16>()),
        (DataSize::U16, DataSize::U16) =>
            Box::new(interp_build.build::<u16, u16>()),
        (DataSize::U32, DataSize::U16) =>
            Box::new(interp_build.build::<u32, u16>()),

        (DataSize::U8, DataSize::U32) =>
            Box::new(interp_build.build::<u8, u32>()),
        (DataSize::U16, DataSize::U32) =>
            Box::new(interp_build.build::<u16, u32>()),
        (DataSize::U32, DataSize::U32) =>
            Box::new(interp_build.build::<u32, u32>()),
        };

    /* Execute instructions */
    interp.run(instr)?;

    Ok(())
    }

/* Compiler execution */
fn run_comp() -> DynResult<()> {
    println!(":P");

    Ok(())
    }