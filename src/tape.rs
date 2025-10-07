use {
    min_max_traits::Max,
    num_traits::{
        Unsigned,
        ConstZero,
        ConstOne,
        WrappingAdd,
        WrappingSub,
        ToPrimitive,
        ToBytes
        },
    std::string::ToString,
    core::{
        fmt::UpperHex,
        iter::repeat_n,
        str::FromStr
        }
    };

/* Trait for Tape's Pointer which will serve both as pointer of a cell, and bound for number of cells */
pub trait TapePointer:
    Sized + Max +
    Unsigned + ConstZero + ConstOne + WrappingAdd + WrappingSub + ToPrimitive {}

impl<T> TapePointer for T where T:
    Sized + Max +
    Unsigned + ConstZero + ConstOne + WrappingAdd + WrappingSub + ToPrimitive {}

/* Trait for Tape's Cell which will hold a value, and allow conversions for reading, and writing */
pub trait TapeCell:
    Sized + Copy + UpperHex + From<u8> + ToString + FromStr +
    Unsigned + ConstZero + ConstOne + WrappingAdd + WrappingSub + ToPrimitive + ToBytes {}

impl<T> TapeCell for T where T:
    Sized + Copy + UpperHex + From<u8> + ToString + FromStr +
    Unsigned + ConstZero + ConstOne + WrappingAdd + WrappingSub + ToPrimitive + ToBytes {}

/* Container for pointer, and it's array */
pub struct Tape<T, U> {
    pointer: T,
    array: Box<[U]>
    }

impl<T: TapePointer, U: TapeCell> Default for Tape<T, U> {
    /* Default constructor method */
    fn default() -> Self {
        /* Declaration of size, with additional assertion to halt the execution in caso of invalid pointer size */
        let size = T::MAX
            .to_usize()
            .expect("Error: Couldn't safely convert to the intended pointer size")
            .checked_add(1)
            .expect("Error: Couldn't create a Tape with intended size");

        /* Struct declaration */
        Self {
            pointer: T::ZERO,
            array: repeat_n(U::ZERO, size)
                .collect()
            }
        }
    }

impl<T: TapePointer, U: TapeCell> Tape<T, U> {
    /* Helper function, for quick conversion into a pointer */
    fn ptr(&self) -> usize {
        /* Unsafe note - unwrap is safe, because it was asserted earlier */
        unsafe {
            self.pointer.to_usize()
                .unwrap_unchecked()
            }
        }

    /* Moves pointer to the right, logical equivalent to '>' */
    pub fn right(&mut self) {
        self.pointer = self.pointer.wrapping_add(&T::ONE);
        }
    /* Moves pointer to the left, logical equivalent to '<' */
    pub fn left(&mut self) {
        self.pointer = self.pointer.wrapping_sub(&T::ONE);
        }

    /* Increments cell at the current pointer location, logical equivalent to '+' */
    pub fn increment(&mut self) {
        let ptr = self.ptr();
        self.array[ptr] = self.array[ptr].wrapping_add(&U::ONE);
        }
    /* Decrements cell at the current pointer location, logical equivalent to '-' */
    pub fn decrement(&mut self) {
        let ptr = self.ptr();
        self.array[ptr] = self.array[ptr].wrapping_sub(&U::ONE);
        }

    /* Get cell value at the current pointer location */
    pub fn get(&self) -> U {
        self.array[self.ptr()]
        }
    /* Set cell value at the current pointer location */
    pub fn set(&mut self, value: U) {
        self.array[self.ptr()] = value;
        }

    /* Check whether cell value at the current pointer location is equal to zero */
    pub fn is_zero(&self) -> bool {
        self.array[self.ptr()] == U::ZERO
        }
    }


#[cfg(test)]
mod test {
    use crate::tape::*;

    #[test]
    fn tape_basic() {
        let tape = Tape::<u8, u8>::default();

        assert_eq!(tape.get(), 0);
        }

    #[test]
    fn tape_cell_size_u8() {
        let mut tape = Tape::<u8, u8>::default();
        
        tape.decrement();

        assert_eq!(tape.get(), u8::MAX);
        }

    #[test]
    fn tape_cell_size_u16() {
        let mut tape = Tape::<u8, u16>::default();
        
        tape.decrement();

        assert_eq!(tape.get(), u16::MAX);
        }

    #[test]
    fn tape_cell_size_u32() {
        let mut tape = Tape::<u8, u32>::default();
        
        tape.decrement();

        assert_eq!(tape.get(), u32::MAX);
        }

    #[test]
    fn tape_ptr_size_u8() {
        let mut tape = Tape::<u8, u8>::default();

        let value = 69;
        
        tape.set(value);

        (0 ..= u8::MAX).for_each(|_| tape.right());

        assert_eq!(tape.get(), value);
        }

    #[test]
    fn tape_ptr_size_u16() {
        let mut tape = Tape::<u16, u8>::default();

        let value = 69;
        
        tape.set(value);

        (0 ..= u16::MAX).for_each(|_| tape.right());

        assert_eq!(tape.get(), value);
        }

    /* Long running test */
    #[test]
    #[ignore]
    fn tape_ptr_size_u32() {
        let mut tape = Tape::<u32, u8>::default();

        let value = 69;
        
        tape.set(value);

        (0 ..= u32::MAX).for_each(|_| tape.right());

        assert_eq!(tape.get(), value);
        }

    #[test]
    fn tape_len_u8() {
        let Tape { array, .. } = Tape::<u8, u8>::default();
        let length = u8::MAX as usize + 1;

        assert_eq!(array.len(), length);
        }

    #[test]
    fn tape_len_u16() {
        let Tape { array, .. } = Tape::<u16, u8>::default();
        let length = u16::MAX as usize + 1;

        assert_eq!(array.len(), length);
        }

    /* Long running test */
    #[test]
    #[ignore]
    fn tape_len_u32() {
        let Tape { array, .. } = Tape::<u32, u8>::default();
        let length = u32::MAX as usize + 1;

        assert_eq!(array.len(), length);
        }
    }