use std::fs::File;
use std::vec::Vec;
mod mem;
mod cpu;
mod gb;
use mem::GameBoyMEM;
use cpu::CPU;
use gb::{Instr, match_instr, LoadRegInstr, ArithInstr};

use std::os::unix::fs::FileExt;



fn load_bootROM(gameboy : &mut GB) {
    let mut f : File = File::open("DMG_ROM.bin").unwrap();  //loads bootrom
    let mut file: [u8; 256] = [0;256];
    f.read_at(&mut file, 0);
    gameboy.mem.chain_write(Vec::from(file), 0x100);
    //now that we've placed everything in memory, we need to run all the instructions


    //first set up Instruction objects
    let mut anom_instr = Instr{ };
    let mut load_instr = LoadRegInstr{};
    let mut arith_instr = ArithInstr{};
    let mut opcode : u8 = 0;

    //start the loop
    loop {
        opcode = gameboy.fetch_inst();
        match_instr(gameboy, opcode , anom_instr, load_instr, arith_instr);


    }


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
    load_bootROM(&mut OurBoi);

    




}


