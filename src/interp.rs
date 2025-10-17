use {
    thiserror::Error,
    std::io::{
        stdin,
        stdout,
        BufReader,
        BufWriter,
        Write,
        Read,
        BufRead
        },
    crate::{
        eval::*,
        tape::*,
        utils::*
        }
    };


/* Run's execution result type */
#[derive(PartialEq, Debug, Error)]
pub enum RunError {
    #[error("A problem occured while writing to the output")]
    Write,
    #[error("A problem occured while flushing the output")]
    Flush,
    #[error("A problem occured while reading from the input")]
    Read
    }


/* The Interpreter container for running code */
pub struct Interpreter<T = u16, U = u8> {
    tape: Tape<T, U>,
    output: BufWriter<Box<dyn Write>>,
    input: BufReader<Box<dyn Read>>,
    read_buffer: String,
    display_mode: DisplayMode,
    debug_display: bool
    }


impl<T, U> Default for Interpreter<T, U>
where T: TapePointer, U: TapeCell {
    /* The default Interpreter settings */
    fn default() -> Self {
        Interpreter::builder()
            .build()
        }
    }


/* Trait for generic ability to run the Interpreter */
pub trait InterpRun {
    fn run(&mut self, instr: InstructionSet) -> Result<(), RunError>;
    }

impl<T, U> InterpRun for Interpreter<T, U>
where T: TapePointer, U: TapeCell {    
    /* Run the source code's instructions */
    fn run(&mut self, instr: InstructionSet) -> Result<(), RunError> {
        let instr_len = instr.len();

        /* Generate the jump table for loops */
        let jump_table = instr.build_jump_table();

        /* Helper types for the instructions' execution */
        let mut instr_ptr: usize = 0;

        /* Debug variable */
        let mut count: u64 = 0;

        /* Main loop */
        while instr_ptr < instr_len {
            /* Get instruction's type, and execute it */
            match instr[instr_ptr] {
                Instruction::Right => 
                    self.tape.right(),
                Instruction::Left =>
                    self.tape.left(),
                Instruction::Increment =>
                    self.tape.increment(),
                Instruction::Decrement =>
                    self.tape.decrement(),
                Instruction::LoopOpen =>
                    if self.tape.is_zero() {
                        instr_ptr = jump_table[instr_ptr];
                        },
                Instruction::LoopClose => 
                    if ! self.tape.is_zero() {
                        instr_ptr = jump_table[instr_ptr];
                        },            
                Instruction::Output => 
                    self.write()?,
                Instruction::Input =>
                    self.read()?
                }

            /* Increment instruction pointer with every loop */
            instr_ptr += 1;

            /* Debug information */
            count += self.debug_display as u64;
            }

        /* Last flush before execution ends */
        if self.output.flush().is_err() {
            return Err(RunError::Flush);
            }

        /* Debug information */
        if self.debug_display {
            eprintln!(
"o> ------- INFORMATION ------- <o
| Number of instructions: {instr_len}
| Num of executed instructions: {count}
o> --------------------------- <o"
                );
            }

        Ok(())
        }
    }

impl Interpreter<(), ()> {
    /* Retrive the Builder container */
    #[inline]
    pub const fn builder() -> InterpreterBuilder {
        InterpreterBuilder {
            output: None,
            input: None,
            display_mode: None,
            debug_display: None
            }
        }
    }

impl<T, U> Interpreter<T, U>
where T: TapePointer, U: TapeCell { 
    fn write(&mut self) -> Result<(), RunError> {
        /* Get output data based on display mode, and byte's type */
        let value = self.tape.get();

        /* Get bytes representing the value */
        let bytes = match self.display_mode {
            /* Print as ASCII if value is graphic */
            /* Unsafe note - unwrap is safe, because guard only allows u8 values */
            DisplayMode::ASCII if is_ascii_printable(value) => unsafe
                { vec![value.to_u8().unwrap_unchecked()] },
            /* Print fallback for ASCII */
            DisplayMode::ASCII => 
                format!(
                    "{value:#0size$X}",
                    size = 2 + 2 * value.to_ne_bytes().as_ref().len()
                    ).into_bytes(),
            /* Print raw numeric value*/
            DisplayMode::Numeric =>
                value.to_string()
                    .into_bytes()
            };
        
        /* Write to the output */
        if self.output.write(&bytes).is_err() {
            return Err(RunError::Write);
            }

        Ok(())
        }

    fn read(&mut self) -> Result<(), RunError> {
        /* Cautionary output flush */
        if self.output.flush().is_err() {
            return Err(RunError::Flush);
            }

        /* Try to get input byte, as long as it isn't correct */
        loop {
            /* Clear buffer, and read */
            self.read_buffer.clear();
            if self.input.read_line(&mut self.read_buffer).is_err() {
                return Err(RunError::Read);
                }

            /* Check whether is correct, then set, and break */
            if let Ok(new_value) = parse_line(&self.read_buffer) {
                self.tape.set(new_value);
                return Ok(());
                }
        
            /* Information for the user */
            if self.debug_display {
                eprintln!("Please input correct data!");
                }
            }
        }
    }


/* The Interpreter Builder container */
pub struct InterpreterBuilder {
    output: Option<BufWriter<Box<dyn Write>>>,
    input: Option<BufReader<Box<dyn Read>>>,
    display_mode: Option<DisplayMode>,
    debug_display: Option<bool>
    }

impl InterpreterBuilder {
    /* Build the Interpreter form the Builder container */
    pub fn build<T, U>(self) -> Interpreter<T, U>
    where T: TapePointer, U: TapeCell  {
        Interpreter {
            tape: Tape::default(),
            output: self.output.unwrap_or(
                BufWriter::new(Box::new(stdout().lock()))
                ),
            input: self.input.unwrap_or(
                BufReader::new(Box::new(stdin().lock()))
                ),
            read_buffer: String::with_capacity(8),
            display_mode: self.display_mode.unwrap_or_default(),
            debug_display: self.debug_display.unwrap_or_default()
            }
        }

    /* Setters */
    pub fn output(mut self, value: Box<dyn Write>) -> Self {
        self.output = Some(BufWriter::new(value));
        self
        }
    pub fn input(mut self, value: Box<dyn Read>) -> Self {
        self.input = Some(BufReader::new(value));
        self
        }
    pub const fn display_mode(mut self, value: DisplayMode) -> Self {
        self.display_mode = Some(value);
        self
        }
    pub const fn debug_display(mut self, value: bool) -> Self {
        self.debug_display = Some(value);
        self
        }
    }