extern crate minifb;
use minifb::{Key, Window, WindowOptions};
use std::fs::File;
use std::vec::Vec;
pub mod mem;
pub mod cpu;
pub mod gb;
pub mod enums;
use mem::GameBoyMEM;
use cpu::CPU;
use gb::{Instr, match_instr, LoadRegInstr, ArithInstr, GB};

use std::os::unix::fs::FileExt;



fn load_ROM(gameboy : &mut GB) {
    let mut f : File = File::open("DMG_ROM.bin").unwrap();  //loads bootrom
    let mut file: [u8; 256] = [0;256];
    f.read_at(&mut file, 0);

    //now that we've placed everything in memory, we need to run all the instructions


    //first set up Instruction objects
    let mut anom_instr = Instr::new();
    let mut load_instr = LoadRegInstr::new();
    let mut arith_instr = ArithInstr::new();
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
    //initialize instruction templates
   

    //first load bootrom
    load_bootROM(&mut OurBoi);

    




}



#[test]
fn test_loads() {
        


}


