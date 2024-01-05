use std;

//first we'll define some special values for the memory map. Refer to http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf for details
//boundaries are listed from highest mem address to lowest
const INTR_ENABLE_REG: u16 = 0xFFFF;
const INTERNAL_RAM_END : u16 = 0xFFFE;
const INTERNAL_RAM_START : u16 = 0xFF80;
const IO_PORT_END : u16 = 0xFF4C;
const IO_PORT_START : u16 = 0xFF00;
const OAM_END : u16 = 0xFEA0;
const OAM_START : u16 = 0xFE00;
const ECHO_RAM_END : u16 = 0xFDFF;
const ECHO_RAM_START : u16 = 0xE000;
const RAM_END : u16 = 0xDFFF;
const RAM_START : u16 = 0xC000;
const SWITCH_RAM_END : u16 = 0xBFFF;
const SWITCH_RAM_START : u16 = 0xA000;
const VID_RAM_END : u16 = 0x9FFF;
const VID_RAM_START : u16 = 0x8000;
const SWITCH_ROM_END : u16 = 0x7FFF;
const SWITCH_ROM_START : u16 = 0x4000;
const ROM_BANK_END : u16 = 0x3FFF;
const ROM_BANK_START : u16 = 0;


pub struct GameBoyMEM {
    pub memory : [u8; 65536],
} 

impl GameBoyMEM {
    pub fn new() -> GameBoyMEM {
    GameBoyMEM {memory : [0; 65536],}
    }

    pub fn write_to_addr(&mut self, val : u8, addr : u8){
        self.memory[addr as usize] = val;
    }

    pub fn chain_write(&mut self, val : Vec<u8>, start_addr : u8)  {
        let mut i  = start_addr;
        for value in val {
            self.memory[i] = value;
            i = i + 1;
        }
    }

    pub fn read_at_addr(&mut self, addr : u8) -> u8 {
        self.memory[addr as usize] 

    }

}