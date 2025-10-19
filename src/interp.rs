use {
    log::{
        info,
        warn
        },
    std::io::{
        stdin,
        stdout,
        Result as IOResult,
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


/* The Interpreter container for running code */
pub struct Interpreter<T = u16, U = u8> {
    tape: Tape<T, U>,
    output: BufWriter<Box<dyn Write>>,
    input: BufReader<Box<dyn Read>>,
    read_buffer: String,
    display_mode: DisplayMode
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
    fn run(&mut self, instr: &InstructionSet) -> IOResult<()>;
    }

impl<T, U> InterpRun for Interpreter<T, U>
where T: TapePointer, U: TapeCell {    
    /* Run the source code's instructions */
    fn run(&mut self, instr: &InstructionSet) -> IOResult<()> {
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
            count += 1;
            }

        /* Last flush before execution ends */
        self.output.write(b"\n")?;
        self.output.flush()?;

        /* Debug information */
        info!("Number of instructions: {instr_len}");
        info!("Number of executed instructions: {count}");

        Ok(())
        }
    }

impl Interpreter<(), ()> {
    /* Retrive the Builder container */
    #[inline]
    pub const fn builder() -> InterpreterBuilder {
        InterpreterBuilder {
            display_mode: None,
            output: None,
            input: None
            }
        }
    }

impl<T, U> Interpreter<T, U>
where T: TapePointer, U: TapeCell { 
    fn write(&mut self) -> IOResult<()> {
        /* Get output data based on display mode, and byte's type */
        let value = self.tape.get();

        /* Get bytes representing the value */
        let bytes = match self.display_mode {
            /* Print as ASCII if value is graphic */
            DisplayMode::ASCII if is_ascii_printable(value) => {
                let converted = value.to_u8(); 
                /* Unsafe note - unwrap is safe, because guard only allows u8 values */
                let unwrapped = unsafe {
                    converted.unwrap_unchecked()
                    };

                vec![unwrapped]
                },
            /* Print fallback for ASCII */
            DisplayMode::ASCII => 
                format!("{value:#0size$X}",
                    size = 2 + 2 * value.to_ne_bytes().as_ref().len()
                    ).into_bytes(),
            /* Print raw numeric value*/
            DisplayMode::Numeric =>
                value.to_string()
                    .into_bytes()
            };
        
        /* Write to the output */
        self.output.write(&bytes)?;

        Ok(())
        }

    fn read(&mut self) -> IOResult<()> {
        /* Cautionary output flush */
        self.output.flush()?;

        /* Try to get input byte, as long as it isn't correct */
        loop {
            /* Clear buffer, and read */
            self.read_buffer.clear();
            self.input.read_line(&mut self.read_buffer)?;

            /* Check whether is correct, then set, and break */
            if let Ok(new_value) = parse_cell_value(&self.read_buffer.trim()) {
                self.tape.set(new_value);
                return Ok(());
                }
        
            /* Information for the user */
            warn!("Please input correct data!");
            }
        }
    }


/* The Interpreter Builder container */
pub struct InterpreterBuilder {
    display_mode: Option<DisplayMode>,
    output: Option<BufWriter<Box<dyn Write>>>,
    input: Option<BufReader<Box<dyn Read>>>
    }

impl InterpreterBuilder {
    /* Build the Interpreter form the Builder container */
    pub fn build<T, U>(self) -> Interpreter<T, U>
    where T: TapePointer, U: TapeCell  {
        Interpreter {
            tape: Tape::default(),
            read_buffer: String::with_capacity(8),
            display_mode: self.display_mode.unwrap_or_default(),
            output: self.output.unwrap_or(
                BufWriter::new(Box::new(stdout().lock()))
                ),
            input: self.input.unwrap_or(
                BufReader::new(Box::new(stdin().lock()))
                )
            }
        }

    /* Setters */
    pub const fn display_mode(mut self, value: DisplayMode) -> Self {
        self.display_mode = Some(value);
        self
        }
    pub fn output(mut self, value: Box<dyn Write>) -> Self {
        self.output = Some(BufWriter::new(value));
        self
        }
    pub fn input(mut self, value: Box<dyn Read>) -> Self {
        self.input = Some(BufReader::new(value));
        self
        }
    }