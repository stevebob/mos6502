use mos6502_assembler::Block;
pub use mos6502_model::machine::{Address, Cpu, MemoryReadOnly};

#[cfg(test)]
pub mod test;
#[cfg(test)]
pub mod test_framework;

mod arithmetic;
mod counter;
mod factorial;
mod infinite_loop;
mod jump_indirect;
mod load_accumulator_immediate;
mod load_and_store_all_addressing_modes;
mod memory_operations;
mod software_interrupt;
mod stack_basic;
mod stack_status_register;
mod store_accumulator;
mod wide_factorial;
pub use arithmetic::*;
pub use counter::*;
pub use factorial::*;
pub use infinite_loop::*;
pub use jump_indirect::*;
pub use load_accumulator_immediate::*;
pub use load_and_store_all_addressing_modes::*;
pub use memory_operations::*;
pub use software_interrupt::*;
pub use stack_basic::*;
pub use stack_status_register::*;
pub use store_accumulator::*;
pub use wide_factorial::*;

pub const PRG_START: Address = 0xC000;

pub trait Sample {
    fn program(block: &mut Block);
    fn num_steps() -> usize;
    fn check_result<M: MemoryReadOnly>(cpu: &Cpu, memory: &M);
}

pub(crate) mod prelude {
    pub use super::{Sample, PRG_START};
    pub use mos6502_assembler::*;
    pub use mos6502_model::addressing_mode::*;
    pub use mos6502_model::assembler_instruction::*;
    pub use mos6502_model::interrupt_vector;
    pub use mos6502_model::machine::{status, Address, Cpu, MemoryReadOnly};
}
