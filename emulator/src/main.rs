use std::fs::File;
use std::vec::Vec;
mod mem;
mod cpu;
mod instr;
use mem::GameBoyMEM;
use cpu::CPU;
use instr::Instr;

use std::os::unix::fs::FileExt;

struct GB {
    pub cpu : CPU,
    pub mem : GameBoyMEM
}

fn load_bootROM(gameboy : &mut GB) {
    let mut f : File = File::open("DMG_ROM.bin").unwrap();  //loads bootrom
    let mut file: [u8; 256] = [0;256];
    f.read_at(&mut file, 0);


}

fn main() {
    
    // for byte in file {
    //     println!("{}", byte);
    // }

    //initialize Gameboy obj
    let mut OurBoi = &GB { cpu : CPU::new(), mem : GameBoyMEM::new()};
    //initialize instruction template
    let mut instr_template = &Instr{ CPU_cycles : 0,
                                            op_type : " ".to_string(),
                                            opcode : 0,
                                            src_addr : 0,
                                            target_addr : 0,
                                            immm : 0,
                                            pc_incr : 0,
                                    };
    //first load bootrom
    




}


