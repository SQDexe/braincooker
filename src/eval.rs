use {
    thiserror::Error,
    std::collections::HashMap,
    core::{
        hint::unreachable_unchecked,
        ops::Index
        },
    crate::utils::RLE,
    };


/* Language instruction set */
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Instruction {
    Right,
    Left,
    Increment,
    Decrement,
    LoopOpen,
    LoopClose,
    Output,
    Input
    }

/* Evaluation's result output type */
#[derive(PartialEq, Debug, Error)]
pub enum EvalError {
    #[error("Open loop oveload was found at: {0}")]
    LoopOverload(usize),
    #[error("Unnecessery loop closing was found at: {0}")]
    UnnecesseryBracket(usize),
    #[error("Unclosed loop(s) was(were) found in number of: {0}")]
    UnclosedBracket(u16)
    }

/* Function for evaluation, checking, sanitisation of provided instructions */
pub fn eval_instr(instr_str: &str) -> Result<InstructionSet, EvalError> {
    let mut output = Vec::with_capacity(instr_str.len());
    /* Increments for loop opening, decrements for loop closing */
    let mut loop_count: u16 = 0;

    /* Iterate over the characters, and indices */
    for (i, chr) in instr_str.chars().enumerate() {
        /* Discard if character is not correct */
        let inst = match chr {
            '>' => Instruction::Right,
            '<' => Instruction::Left,
            '+' => Instruction::Increment,
            '-' => Instruction::Decrement,
            '[' => {
                /* Check for loop start */
                loop_count = match loop_count.checked_add(1) {
                    Some(count) => count,
                    _ => return Err(EvalError::LoopOverload(i))
                    };
                
                Instruction::LoopOpen
                },
            ']' => {
                /* Check for loop end */
                loop_count = match loop_count.checked_sub(1) {
                    Some(count) => count,
                    _ => return Err(EvalError::UnnecesseryBracket(i))
                    };

                Instruction::LoopClose
                },
            '.' => Instruction::Output,
            ',' => Instruction::Input,
            _ => continue
            };

        /* Add the instruction to the list */
        output.push(inst);
        }

    /* Check for any other unmatched brackets */
    if loop_count != 0 {
        return Err(EvalError::UnclosedBracket(loop_count));
        }

    /* Resize the list for space saving */
    output.shrink_to_fit();

    /* Final product */
    Ok(InstructionSet(output))
    }


/* Container for sanitised instructions */
#[derive(PartialEq, Debug)]
pub struct InstructionSet (
    Vec<Instruction>
    );

impl Index<usize> for InstructionSet {
    type Output = Instruction;

    /* Index access operation */
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
        }
    }

impl InstructionSet {
    /* Get number of instructions */
    #[inline]
    pub const fn len(&self) -> usize {
        self.0.len()
        }
    /* Get whether is empty */
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
        }

    /* Function for prunning an optional, "comment loop" that can be created on first instruction */
    pub fn prune_comment_loop(&mut self) -> bool {
        /* Early return */
        if self.is_empty() {
            return false;
            }

        /* Create an iterator over the collection */
        let mut iter = self.0.iter()
            .enumerate();

        /* Return from the fuction, if first instruction is not a loop opening */
        match iter.next() {
            Some((_, &Instruction::LoopOpen)) => (),
            _ => return false
            };

        /* Increments for loop opening, decrements for loop closing */
        let mut loop_count: u16 = 0;

        /* Loop until a loop closing appears, then split instructions at next index, and reassing the value */
        while let Some((i, &value)) = iter.next() {
            match value {
                Instruction::LoopClose if loop_count == 0 => {
                    self.0 = self.0.split_off(i + 1);
                    return true;
                    },
                Instruction::LoopOpen =>
                    loop_count += 1,
                Instruction::LoopClose =>
                    loop_count -= 1,
                _ => ()
                }
            }

        /* Unsafe note - it is safe, because it should always exit earlier */
        unsafe
            { unreachable_unchecked() }
        }

    /* Function for prunning all "comment loops" which appear one after the other*/
    pub fn prune_all_loops(&mut self) -> usize {
        let mut count = 0;

        /* Try to prune "comment loops" as long as possible */
        while self.prune_comment_loop() {
            count += 1;
            }
        
        count
        }

    /* Function for building a jump table based on loop openings, and closings */
    pub fn build_jump_table(&self) -> JumpTable {
        let mut output = HashMap::new();

        /* Early return */
        if self.is_empty() {
            return JumpTable(output);
            }

        /* Stack for loop openings */
        let mut loop_stack = Vec::new();

        /* Iterate over instructions, and indices */
        for (i, inst) in self.0.iter().enumerate() {
            match inst {
                Instruction::LoopOpen =>
                    /* Push loop opening index */
                    loop_stack.push(i),
                Instruction::LoopClose => {
                    /* Get coresponding index, current index */
                    /* Unsafe note - unwrap is safe, because the instruction set was sanitised during evaluation */
                    let (start, end) = unsafe {
                        (loop_stack.pop().unwrap_unchecked(), i)
                        };

                    /* Push jumps - opening <-> closing */
                    output.insert(start, end);
                    output.insert(end, start);                    
                    },
                _ => continue
                }
            }

        /* Resize the map for space saving */
        output.shrink_to_fit();

        /* Final product */
        JumpTable(output)
        }

    /* Function for compressing the Instruction Set */
    pub fn encode_run_length(&self) -> RLEInstructionSet {
        let mut output = Vec::with_capacity(self.len());

        /* Early return */
        if self.is_empty() {
            return RLEInstructionSet(output.into_boxed_slice());
            }

        /* Create a peekable iterator over the collection */
        let mut iter = self.0.iter()
            .peekable();

        /* Iterate over the collection */
        while let Some(&curr) = iter.next() {
            /* Count can not be 0 */
            let mut count = 1;

            /* Peek further, as long as it's the same Instruction, and is smaller than 0xffff */
            while let Some(&&next) = iter.peek() {
                match curr == next && count < u16::MAX {
                    /* Push main iteration further */
                    true => {
                        iter.next();
                        count += 1;
                        },
                    /* Stop peeking */
                    false => break
                    }
                }

            /* Push the count, and Instruction to the list */
            output.push(RLE::new(count, curr));
            }

        /* Final product */
        RLEInstructionSet(output.into_boxed_slice())
        }
    }


/* Container for a jump table, based on provided instructions */
pub struct JumpTable (
    HashMap<usize, usize>
    );

impl Index<usize> for JumpTable {
    type Output = usize;

    /* Index access operation */
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[&index]
        }
    }


/* Container for an optimised instruction set */
#[derive(PartialEq, Debug)]
pub struct RLEInstructionSet (
    Box<[RLE<Instruction>]>
    );


#[cfg(test)]
mod test {
    use {
        core::iter::repeat_n,
        crate::{
            eval::{
                *,
                Instruction::*
                },
            utils::*
            }
        };

    #[test]
    fn eval_basic() {
        let instructions = eval_instr("++>+++++[<+>-]++++++++[<++++++>-]<.")
            .expect("Unreachable");
        let output = InstructionSet(vec![
            Increment, Increment,
            Right,
            Increment, Increment, Increment, Increment,  Increment,
            LoopOpen,
            Left,
            Increment,
            Right,
            Decrement,
            LoopClose,
            Increment, Increment, Increment, Increment, Increment, Increment, Increment, Increment,
            LoopOpen,
            Left,
            Increment, Increment, Increment, Increment, Increment, Increment,
            Right,
            Decrement,
            LoopClose,
            Left,
            Output
            ]);

        assert_eq!(instructions, output);
        }

    #[test]
    fn eval_comments() {
        let comments_str =
"[ https://pl.wikipedia.org/wiki/Brainfuck ]
,>,>++++++[-<--------<-------->>] przechowuje dwie cyfry w (0) i (1) od obu odejmujemy 48
<<[                               powtarzaj dopóki dzielna jest niezerowa
>[->+>+<<]                        kopiuj dzielnik z (1) do (2) i (3) (1) się zeruje
>[-<<-                            odejmujemy 1 od dzielnej (0) i dzielnika (2) dopóki (2) nie jest 0
[>]>>>[<[>>>-<<<[-]]>>]<<]        jeżeli dzielna jest zerem wyjdź z pętli
>>>+                              dodaj jeden do ilorazu w (5)
<<[-<<+>>]                        kopiuj zapisany dzielnik z (3) do (1)
<<<]                              przesuń wskaźnik na (0) i powtórz pętlę
>[-]>>>>[-<<<<<+>>>>>]            kopiuj iloraz z (5) do (0)
<<<<++++++[-<++++++++>]<.         dodaj 48 i drukuj wynik";
        let no_comments_str = "[..],>,>++++++[-<--------<-------->>]<<[>[->+>+<<]>[-<<-[>]>>>[<[>>>-<<<[-]]>>]<<]>>>+<<[-<<+>>]<<<]>[-]>>>>[-<<<<<+>>>>>]<<<<++++++[-<++++++++>]<.";

        let comments = eval_instr(comments_str);
        let no_comments = eval_instr(no_comments_str);

        assert_eq!(comments, no_comments);
        }

    #[test]
    fn eval_err_overload() {
        let size = u16::MAX as usize;
        let instr_str: String = repeat_n('[', size + 1)
            .collect();
        let instr = eval_instr(&instr_str);
        let output = Err(EvalError::LoopOverload(size));

        assert_eq!(instr, output);
        }

    #[test]
    fn eval_err_unclosed() {
        let instr = eval_instr("++[->++++[.[+]<]");
        let output = Err(EvalError::UnclosedBracket(1));

        assert_eq!(instr, output);
        }

    #[test]
    fn eval_err_mutiple_unclosed() {
        let instr = eval_instr("[[[[]");
        let output = Err(EvalError::UnclosedBracket(3));

        assert_eq!(instr, output);
        }

    #[test]
    fn eval_err_unnecessary() {
        let instr = eval_instr(",.]");
        let output = Err(EvalError::UnnecesseryBracket(2));

        assert_eq!(instr, output);
        }

    #[test]
    fn prune_basic() {
        let mut instructions = eval_instr("[+++]>+<-")
            .expect("Unreachable");
        let got_pruned = instructions.prune_comment_loop();
        let pruned = InstructionSet(vec![
            Right,
            Increment,
            Left,
            Decrement
            ]);

        assert!(got_pruned);
        assert_eq!(instructions, pruned);
        }

    #[test]
    fn prune_no_loop() {
        let mut instructions = eval_instr(">+<-")
            .expect("Unreachable");
        let not_pruned = ! instructions.prune_comment_loop();
        let pruned = InstructionSet(vec![
            Right,
            Increment,
            Left,
            Decrement
            ]);

        assert!(not_pruned);
        assert_eq!(instructions, pruned);
        }

    #[test]
    fn prune_only_loop() {
        let mut instructions = eval_instr("[+++]")
            .expect("Unreachable");
        let got_pruned = instructions.prune_comment_loop();
        let pruned = InstructionSet(vec![]);

        assert!(got_pruned);
        assert_eq!(instructions, pruned);
        }

    #[test]
    fn prune_mutiple_loops() {
        let mut instructions = eval_instr("[+++][---][>][<],.")
            .expect("Unreachable");
        let count = instructions.prune_all_loops();
        let pruned = InstructionSet(vec![Input, Output]);

        assert_eq!(count, 4);
        assert_eq!(instructions, pruned);
        }

    #[test]
    fn prune_loops_within_loops() {
        let mut instructions = eval_instr("[[+++][---][[>][<]]],.")
            .expect("Unreachable");
        let got_pruned = instructions.prune_comment_loop();
        let pruned = InstructionSet(vec![Input, Output]);

        assert!(got_pruned);
        assert_eq!(instructions, pruned);
        }

    #[test]
    fn instr_rle_basic() {
        let instructions = eval_instr("++>+++++[<+>-]++++++++[<++++++>-]<.")
            .expect("Unreachable")
            .encode_run_length();

        let rle = RLEInstructionSet(Box::new([
            RLE::new(2, Increment),
            RLE::new(1, Right),
            RLE::new(5, Increment),
            RLE::new(1, LoopOpen),
            RLE::new(1, Left),
            RLE::new(1, Increment),
            RLE::new(1, Right),
            RLE::new(1, Decrement),
            RLE::new(1, LoopClose),
            RLE::new(8, Increment),
            RLE::new(1, LoopOpen),
            RLE::new(1, Left),
            RLE::new(6, Increment),
            RLE::new(1, Right),
            RLE::new(1, Decrement),
            RLE::new(1, LoopClose),
            RLE::new(1, Left),
            RLE::new(1, Output)
            ]));

        assert_eq!(instructions, rle);
        }

    #[test]
    fn instr_rle_many() {
        let instr_str: String = repeat_n('+', u16::MAX as usize + 1)
            .collect();

        let instructions = eval_instr(&instr_str)
            .expect("Unreachable")
            .encode_run_length();

        let rle = RLEInstructionSet(Box::new([
            RLE::new(u16::MAX, Increment),
            RLE::new(1, Increment),
            ]));

        assert_eq!(instructions, rle);
        }
    }