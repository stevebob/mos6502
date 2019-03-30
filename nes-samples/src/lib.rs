extern crate assembler;
extern crate ines;
extern crate mos6502;
extern crate nes;

pub mod single_block {
    pub use assembler::*;
    pub use instruction::*;
    pub use mos6502::*;
    pub use nes::*;
    use std::io::{self, Write};
    pub use AddressOperand::*;

    use ines::*;

    pub const PRG_START: Address = nrom::PRG_START_HI;
    pub const INTERRUPT_VECTOR_START_PC_OFFSET: Address = interrupt_vector::START_PC_LO - PRG_START;

    pub fn assemble_ines_file_to_stdout(block: &Block) {
        let mut prg_rom = Vec::new();
        block
            .assemble(nrom::PRG_START_HI, ines::PRG_ROM_BLOCK_BYTES, &mut prg_rom)
            .expect("Failed to assemble");
        let ines = Ines {
            header: Header {
                num_prg_rom_blocks: 1,
                num_chr_rom_blocks: 0,
            },
            prg_rom,
            chr_rom: Vec::new(),
        };
        let mut output = Vec::new();
        ines.encode(&mut output);
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write(&output).expect("Failed to write output");
    }

    pub fn with_block<F: FnOnce(&mut Block)>(f: F) {
        let mut a = Block::new();
        f(&mut a);
        assemble_ines_file_to_stdout(&a);
    }
}
