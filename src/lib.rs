/* Modules declaration */
mod eval;
mod interp;
mod rle;
mod tape;
mod utils;

/* Lib re-export */
pub use {
    interp::{
        RunError,
        InterpRun,
        Interpreter,
        InterpreterBuilder
        },
    eval::{
        eval_instr,
        EvalError,
        InstructionSet
        },
    rle::RLEInstructionSet,
    utils::DisplayMode
    };