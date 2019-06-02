use super::{
    mirroring, CpuMapper, Dynamic, Error, Mapper, NameTableChoice, PaletteRam, PatternTableChoice,
    PpuAddress, PpuMapper, NAME_TABLE_BYTES, PATTERN_TABLE_BYTES,
};
use mirroring::Mirroring;
use mos6502::Address;

big_array! { BigArray; }

const PRG_ROM_BYTES: usize = 32 * 1024;
const CHR_ROM_BYTES: usize = 8 * 1024;
const NAME_TABLE_RAM_BYTES: usize = 2 * NAME_TABLE_BYTES;

#[derive(Serialize, Deserialize, Clone)]
pub struct Nrom<M: Mirroring> {
    #[serde(with = "BigArray")]
    prg_rom: [u8; PRG_ROM_BYTES],
    #[serde(with = "BigArray")]
    chr_rom: [u8; CHR_ROM_BYTES],
    #[serde(with = "BigArray")]
    name_table_ram: [u8; NAME_TABLE_RAM_BYTES],
    palette_ram: PaletteRam,
    mirroring: M,
}

impl<M: Mirroring> Nrom<M> {
    pub fn new(mirroring: M, prg_rom_raw: &[u8], chr_rom_raw: &[u8]) -> Result<Self, Error> {
        let mut prg_rom = [0; PRG_ROM_BYTES];
        let mut chr_rom = [0; CHR_ROM_BYTES];
        const HALF_PRG_ROM_BYTES: usize = PRG_ROM_BYTES / 2;
        match prg_rom_raw.len() {
            PRG_ROM_BYTES => (&mut prg_rom).copy_from_slice(prg_rom_raw),
            HALF_PRG_ROM_BYTES => {
                // copy the prg data into each half of prg_rom
                (&mut prg_rom[0..HALF_PRG_ROM_BYTES]).copy_from_slice(prg_rom_raw);
                (&mut prg_rom[HALF_PRG_ROM_BYTES..]).copy_from_slice(prg_rom_raw);
            }
            _ => return Err(Error::UnexpectedPrgRomSize),
        }
        match chr_rom_raw.len() {
            CHR_ROM_BYTES => (&mut chr_rom).copy_from_slice(chr_rom_raw),
            _ => return Err(Error::UnexpectedChrRomSize),
        }
        let name_table_ram = [0; NAME_TABLE_RAM_BYTES];
        let palette_ram = PaletteRam::default();
        Ok(Self {
            prg_rom,
            chr_rom,
            name_table_ram,
            palette_ram,
            mirroring,
        })
    }
}

impl<M: Mirroring> PpuMapper for Nrom<M> {
    fn ppu_write_u8(&mut self, address: PpuAddress, data: u8) {
        let address = address % 0x4000;
        match address {
            0x0000..=0x0FFF => println!("unimplemented pattern table write"),
            0x1000..=0x1FFF => println!("unimplemented pattern table write"),
            0x2000..=0x23FF => self.name_table_ram[address as usize - 0x2000] = data,
            0x2400..=0x27FF => self.name_table_ram[address as usize - 0x2400] = data,
            0x2800..=0x2BFF => self.name_table_ram[address as usize - 0x2400] = data,
            0x2C00..=0x2FFF => self.name_table_ram[address as usize - 0x2800] = data,
            0x3000..=0x33FF => self.name_table_ram[address as usize - 0x3000] = data,
            0x3400..=0x37FF => self.name_table_ram[address as usize - 0x3400] = data,
            0x3800..=0x3BFF => self.name_table_ram[address as usize - 0x3400] = data,
            0x3C00..=0x3EFF => self.name_table_ram[address as usize - 0x3800] = data,
            0x3F00..=0x3FFF => self.palette_ram.write_u8(address as u8, data),
            _ => unreachable!(),
        }
    }
    fn ppu_read_u8(&self, address: PpuAddress) -> u8 {
        let address = address % 0x4000;
        match address {
            0x0000..=0x1FFF => self.chr_rom[address as usize],
            0x2000..=0x23FF => self.name_table_ram[address as usize - 0x2000],
            0x2400..=0x27FF => self.name_table_ram[address as usize - 0x2400],
            0x2800..=0x2BFF => self.name_table_ram[address as usize - 0x2400],
            0x2C00..=0x2FFF => self.name_table_ram[address as usize - 0x2800],
            0x3000..=0x33FF => self.name_table_ram[address as usize - 0x3000],
            0x3400..=0x37FF => self.name_table_ram[address as usize - 0x3400],
            0x3800..=0x3BFF => self.name_table_ram[address as usize - 0x3400],
            0x3C00..=0x3EFF => self.name_table_ram[address as usize - 0x3800],
            0x3F00..=0x3FFF => self.palette_ram.read_u8(address as u8),
            _ => unreachable!(),
        }
    }
    fn ppu_pattern_table(&self, choice: PatternTableChoice) -> &[u8] {
        let base_address = choice.base_address() as usize;
        &self.chr_rom[base_address..(base_address + PATTERN_TABLE_BYTES)]
    }
    fn ppu_name_table(&self, choice: NameTableChoice) -> &[u8] {
        let address_offset = M::name_table_base_address(choice) as usize;
        &self.name_table_ram[address_offset..(address_offset + NAME_TABLE_BYTES)]
    }
    fn ppu_palette_ram(&self) -> &[u8] {
        &self.palette_ram.ram
    }
}

impl<M: Mirroring> CpuMapper for Nrom<M> {
    fn cpu_read_u8(&mut self, address: Address) -> u8 {
        self.cpu_read_u8_read_only(address)
    }
    fn cpu_write_u8(&mut self, address: Address, data: u8) {
        eprintln!(
            "unexpected cartridge write of {:X} to address {:X}",
            data, address
        );
    }
    fn cpu_read_u8_read_only(&self, address: Address) -> u8 {
        match address {
            0x8000..=0xFFFF => self.prg_rom[address as usize % 0x8000],
            other => {
                eprintln!("unexpected cartridge read from address {:X}", other);
                0
            }
        }
    }
}

impl<M: mirroring::SpecificMapperDynamic> Mapper for Nrom<M> {
    fn clone_dynamic(&self) -> Dynamic {
        M::nrom(self.clone())
    }
}