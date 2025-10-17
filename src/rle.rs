use {
    core::num::NonZeroU16,
    crate::eval::Instruction
    };


/* Run-Length Encoding helper type */
#[derive(PartialEq, Debug)]
pub struct RLE<T> ( NonZeroU16, T );

impl RLE<()> {
    pub const MAX: u16 =  u16::MAX;
    }

impl<T> RLE<T>
where T: Copy {
    /* Constructor function */
    #[inline]
    pub const fn new(count: u16, value: T) -> Self {
        let count = NonZeroU16::new(count)
            .expect("Error: Recieved a Zero value"); 
        
        Self ( count, value )
        }

    /* Getter */
    #[inline]
    pub const fn get(&self) -> (u16, T) {
        let &RLE(count, value) = self;

        (count.get(), value)
        }
    }


/* Container for an optimised instruction set */
#[derive(PartialEq, Debug)]
pub struct RLEInstructionSet (
    pub(crate) Box<[RLE<Instruction>]>
    );

#[cfg(test)]
mod test {
    use {
        core::iter::repeat_n,
        crate::{
            eval::{
                eval_instr,
                Instruction::*
                },
            rle::*
            }
        };

    #[test]
    fn rle_basic() {
        let value = RLE::new(8, true)
            .get();
        let other = (8, true);

        assert_eq!(value, other);
        }

    #[test]
    #[should_panic]
    fn rle_incorrect() {
        RLE::new(0, true);
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
        let instr_str: String = repeat_n('+', RLE::MAX as usize + 1)
            .collect();

        let instructions = eval_instr(&instr_str)
            .expect("Unreachable")
            .encode_run_length();

        let rle = RLEInstructionSet(Box::new([
            RLE::new(RLE::MAX, Increment),
            RLE::new(1, Increment),
            ]));

        assert_eq!(instructions, rle);
        }
    }