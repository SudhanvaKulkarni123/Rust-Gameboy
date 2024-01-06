use std;
use crate::mem::GameBoyMEM;
use crate::cpu::CPU;


struct GB {
    pub cpu : CPU,
    pub mem : GameBoyMEM
}

impl GB {
    pub fn fetch_inst(&self) -> u8 {
        self.mem.read_at_addr(self.cpu.pc)
    }
}


fn Nop() {}

/*the idea is to use Rust's "Trait" feature -
 read more here : https://doc.rust-lang.org/book/ch10-02-traits.html
 I'll define one common Instruction struct that will mainly be used for handling edge case.
 I'll also define special instruction structs for Load and Arithmetic type operations since they correspond to a lot of opcodes
*/
pub struct Instr {
    CPU_cycles : u8,
    op_type : String,           //might not need this
    target_addr : u16,
    src_addr : u16,
    target_reg : String,
    src_reg : String,
    opcode : u8,
    imm : u16,
    pc_incr : u8,
    pub uses_imm : bool,

}
impl Instr {
        pub fn new() -> Instr {
            Instr {
                CPU_cycles: 0,
                op_type: String::new(),
                target_addr: 0,
                src_addr: 0,
                target_reg: String::new(),
                src_reg: String::new(),
                opcode: 0,
                imm: 0,
                pc_incr: 0,
                uses_imm: false,
            }
        }
    }
pub struct LoadRegInstr {
    CPU_cycles : u8,
    uses_imm : bool,
    src_reg : String,
    dst_reg : String,
    imm : u16,
    imm_length : u8,
    op_type : String

}
impl LoadRegInstr {
        pub fn new() -> LoadRegInstr {
            LoadRegInstr {
                CPU_cycles: 0,         // Set default values as needed
                uses_imm: false,
                src_reg: String::new(), // You may want to initialize with a specific value
                dst_reg: String::new(),
                imm: 0,
                imm_length: 0,
                op_type: String::new(),
            }
        }
    }

pub struct ArithInstr {
    CPU_cycles : u8,
    uses_imm : bool,
    src_reg : String,
    dst_reg : String,
    imm : u16,
    imm_length : u8,
}

impl ArithInstr {
        pub fn new() -> ArithInstr {
            ArithInstr {
                CPU_cycles: 0,
                uses_imm: false,
                src_reg: String::new(),
                dst_reg: String::new(),
                imm: 0,
                imm_length: 0,
            }
        }
    }



pub trait ExecInstr {
    fn execute(&self, gameboy : &mut GB);
    fn set_regs(&mut self, src : String, dst : String);
    fn set_imm(&mut self, imm_size : u8, gameboy : &mut GB) ;
    fn set_cputime(&mut self, num_cycles : u8);
    fn set_locs(&mut self, src : u16, dst : u16);
    fn set_name(&mut self, name: String);
}




//define software implementation of required functions

impl ExecInstr for LoadRegInstr {
    fn execute(&self, gb : &mut GB) {
        //LD nn,n
        match self.op_type {
                String::from("LD nn,n") => { gb.cpu.write_to(self.dst_reg, self.imm);},
                String::from("LD r1,r2") => {
                        if self.src_reg == String::from("hl") {
                                gb.cpu.write_to(self.dst_reg, gb.mem.read_at_addr(gb.cpu.hl()));
                        } else if self.dst_reg == String::from("hl") {
                                gb.mem.write_to_addr(gb.cpu.read_from(self.src_reg) ,gb.cpu.hl());
                        } else {
                                gb.cpu.write_to(self.dst_reg, gb.cpu.read_from(self.src_reg));
                        }
                },
                String::from("LD A,n") => {
                        if self.uses_imm {
                                gb.cpu.write_to(self.dst_reg, self.imm as u8);
                        } else {
                                match self.src_reg {
                                        String::from("use_imm") => {gb.cpu.write_to(self.dst_reg, gb.mem.read_at_addr(self.imm));},
                                        String::from("hl") => {gb.cpu.write_to(self.dst_reg, gb.mem.read_at_addr(gb.cpu.hl()));},
                                        String::from("bc") => {gb.cpu.write_to(self.dst_reg, gb.mem.read_at_addr(gb.cpu.bc()));},
                                        String::from("de") => {gb.cpu.write_to(self.dst_reg, gb.mem.read_at_addr(gb.cpu.de()));},
                                        _ => {gb.cpu.write_to(self.dst_reg, gb.cpu.read_from(self.src_reg));},
                                }
                                
                        }
                }
                String::from("LD n,A") => {
                        match self.src_reg {
                                String::from("use_imm") => {gb.mem.write_to_addr(gb.cpu.read_from(self.src_reg), self.imm);},
                                String::from("hl") => {gb.mem.write_to_addr(gb.cpu.read_from(self.src_reg), gb.cpu.hl());},
                                String::from("bc") => {gb.mem.write_to_addr(gb.cpu.read_from(self.src_reg), gb.cpu.bc());},
                                String::from("de") => {gb.mem.write_to_addr(gb.cpu.read_from(self.src_reg), gb.cpu.de());},
                                _ => {gb.cpu.write_to(self.dst_reg, gb.cpu.read_from(self.src_reg));},
                        }
                }
                String::from("")

        }
       

        
    }
    fn set_regs(&mut self, src : String, dst : String) {
        self.src_reg = src;
        self.dst_reg = dst;
    }
    fn set_cputime(&mut self, num_cycles : u8) {
        self.CPU_cycles = num_cycles;
        
    }
    fn set_imm(&mut self, imm_size : u8, gameboy : &mut GB) {
        if imm_size == 0 {Ok(())}
        if imm_size == 8 {
                self.imm = gameboy.mem.read_at_addr(gameboy.cpu.pc + 1) as u16
        }
        if imm_size == 16 {
                self.imm = ((gameboy.mem.read_at_addr(gameboy.cpu.pc + 2) as u16) << 8) + (gameboy.mem.read_at_addr(gameboy.cpu.pc + 1) as u16)
        }


        
    }
    fn set_locs(&mut self, src : u16, dst : u16) {
    }
    fn set_name(&mut self, name: String) {
        self.op_type = name;
    }


}


impl ExecInstr for Instr {
    fn execute(&self, gameboy : &mut GB) {
        if self.opcode == 0 {
                Nop()
        }




        gameboy.cpu.pc = gameboy.cpu.pc  + self.pc_incr;



    }
    fn set_cputime(&mut self, num_cycles : u8) {
        self.CPU_cycles = num_cycles;
        
    }
    fn set_imm(&mut self, imm_size : u8, gameboy : &mut GB) {
        if imm_size == 0 {Ok(())}
        if imm_size == 8 {
                self.imm = gameboy.mem.read_at_addr(gameboy.cpu.pc + 1) as u16
        }
        if imm_size == 16 {
                self.imm = ((gameboy.mem.read_at_addr(gameboy.cpu.pc + 2) as u16) << 8) + (gameboy.mem.read_at_addr(gameboy.cpu.pc + 1) as u16)
        }


        
    }
    fn set_regs(&mut self, src : String, dst : String) {

        
    }
    fn set_locs(&mut self, src : u16, dst : u16) {

    }
    fn set_name(&mut self, name: String) {
        self.op_type = name;
    }

}

//traits for load register instructions
impl ExecInstr for LoadRegInstr {
    fn execute(&self) {
        
    }
    fn set_cputime(&mut self, num_cycles : u8) {
        
    }
    fn set_imm(&mut self, imm_size : u8) {
        
    }
    fn set_regs(&mut self, src : String, dst : String) {
        
    }
    fn set_locs(&mut self, src : u16, dst : u16) {

    }
    fn set_name(&mut self, name: String) {
        self.op_type = name;
    }

}

//not to happy about this design choice of multiple instruction structs. Will have to review it later
pub fn match_instr(&mut gb : GB, opcode : u8, instr : &mut Instr, load_instr : &mut LoadRegInstr, arith : &mut ArithInstr) -> u8 {
    match opcode {
        0x00 => {},
        0x01 => {},
        0x02 => {},
        0x03 => {},
        0x04 => {},
        0x05 => {},
        0x06 => {
                load_instr.set_name("LD nn,n".to_string());
                load_instr.set_cputime(8);
                load_instr.set_regs("".to_string(),"b".to_string());
                load_instr.uses_imm = true;
                load_instr.set_imm(8);
                load_instr.execute()
                },
        0x07 => {},
        0x08 => {},
        0x09 => {},
        0x0A => {},
        0x0B => {},
        0x0C => {},
        0x0D => {},
        0x0E => {instr.set_name("LD nn,n".to_string());
        instr.set_cputime(8);
        instr.set_regs("".to_string(),"c".to_string());
        instr.uses_imm = true;
        instr.set_imm(8);},
        0x0F => {},
        0x10 => {},
        0x11 => {},
        0x12 => {},
        0x13 => {},
        0x14 => {},
        0x15 => {},
        0x16 => {instr.set_name("LD nn,n".to_string());
                instr.set_cputime(8);
                instr.set_regs("".to_string(),"b".to_string());
                instr.uses_imm = true;
                instr.set_imm(8);
                },
        0x17 => {},
        0x18 => {},
        0x19 => {},
        0x1A => {},
        0x1B => {},
        0x1C => {},
        0x1D => {},
        0x1E => {instr.set_name("LD nn,n".to_string());
        instr.set_cputime(8);
        instr.set_regs("".to_string(),"c".to_string());
        instr.uses_imm = true;
        instr.set_imm(8);},
        0x1F => {},
        0x20 => {},
        0x21 => {},
        0x22 => {},
        0x23 => {},
        0x24 => {},
        0x25 => {},
        0x26 => {instr.set_name("LD nn,n".to_string());
                instr.set_cputime(8);
                instr.set_regs("".to_string(),"b".to_string());
                instr.uses_imm = true;
                instr.set_imm(8);
                },
        0x27 => {},
        0x28 => {},
        0x29 => {},
        0x2A => {},
        0x2B => {},
        0x2C => {},
        0x2D => {},
        0x2E => {instr.set_name("LD nn,n".to_string());
        instr.set_cputime(8);
        instr.set_regs("".to_string(),"c".to_string());
        instr.uses_imm = true;
        instr.set_imm(8);},
        0x2F => {},
        0x30 => {},
        0x31 => {},
        0x32 => {},
        0x33 => {},
        0x34 => {},
        0x35 => {},
        0x36 => {instr.set_name("LD nn,n".to_string());
                instr.set_cputime(8);
                instr.set_regs("".to_string(),"b".to_string());
                instr.uses_imm = true;
                instr.set_imm(8);
                },
        0x37 => {},
        0x38 => {},
        0x39 => {},
        0x3A => {},
        0x3B => {},
        0x3C => {},
        0x3D => {},
        0x3E => {instr.set_name("LD nn,n".to_string());
        instr.set_cputime(8);
        instr.set_regs("".to_string(),"c".to_string());
        instr.uses_imm = true;
        instr.set_imm(8);},
        0x3F => {},
        0x40 => {},
        0x41 => {},
        0x42 => {},
        0x43 => {},
        0x44 => {},
        0x45 => {},
        0x46 => {instr.set_name("LD nn,n".to_string());
                instr.set_cputime(8);
                instr.set_regs("".to_string(),"b".to_string());
                instr.uses_imm = true;
                instr.set_imm(8);
                },
        0x47 => {},
        0x48 => {},
        0x49 => {},
        0x4A => {},
        0x4B => {},
        0x4C => {},
        0x4D => {},
        0x4E => {instr.set_name("LD nn,n".to_string());
        instr.set_cputime(8);
        instr.set_regs("".to_string(),"c".to_string());
        instr.uses_imm = true;
        instr.set_imm(8);},
        0x4F => {},
        0x50 => {},
        0x51 => {},
        0x52 => {},
        0x53 => {},
        0x54 => {},
        0x55 => {},
        0x56 => {instr.set_name("LD nn,n".to_string());
                instr.set_cputime(8);
                instr.set_regs("".to_string(),"b".to_string());
                instr.uses_imm = true;
                instr.set_imm(8);
                },
        0x57 => {},
        0x58 => {},
        0x59 => {},
        0x5A => {},
        0x5B => {},
        0x5C => {},
        0x5D => {},
        0x5E => {instr.set_name("LD nn,n".to_string());
        instr.set_cputime(8);
        instr.set_regs("".to_string(),"c".to_string());
        instr.uses_imm = true;
        instr.set_imm(8);},
        0x5F => {},
        0x60 => {},
        0x61 => {},
        0x62 => {},
        0x63 => {},
        0x64 => {},
        0x65 => {},
        0x66 => {instr.set_name("LD nn,n".to_string());
                instr.set_cputime(8);
                instr.set_regs("".to_string(),"b".to_string());
                instr.uses_imm = true;
                instr.set_imm(8);
                },
        0x67 => {},
        0x68 => {},
        0x69 => {},
        0x6A => {},
        0x6B => {},
        0x6C => {},
        0x6D => {},
        0x6E => {instr.set_name("LD nn,n".to_string());
        instr.set_cputime(8);
        instr.set_regs("".to_string(),"c".to_string());
        instr.uses_imm = true;
        instr.set_imm(8);},
        0x6F => {},
        0x70 => {},
        0x71 => {},
        0x72 => {},
        0x73 => {},
        0x74 => {},
        0x75 => {},
        0x76 => {instr.set_name("LD nn,n".to_string());
                instr.set_cputime(8);
                instr.set_regs("".to_string(),"b".to_string());
                instr.uses_imm = true;
                instr.set_imm(8);
                },
        0x77 => {},
        0x78 => {},
        0x79 => {},
        0x7A => {},
        0x7B => {},
        0x7C => {},
        0x7D => {},
        0x7E => {instr.set_name("LD nn,n".to_string());
        instr.set_cputime(8);
        instr.set_regs("".to_string(),"c".to_string());
        instr.uses_imm = true;
        instr.set_imm(8);},
        0x7F => {},
        0x80 => {},
        0x81 => {},
        0x82 => {},
        0x83 => {},
        0x84 => {},
        0x85 => {},
        0x86 => {instr.set_name("LD nn,n".to_string());
                instr.set_cputime(8);
                instr.set_regs("".to_string(),"b".to_string());
                instr.uses_imm = true;
                instr.set_imm(8);
                },
        0x87 => {},
        0x88 => {},
        0x89 => {},
        0x8A => {},
        0x8B => {},
        0x8C => {},
        0x8D => {},
        0x8E => {instr.set_name("LD nn,n".to_string());
        instr.set_cputime(8);
        instr.set_regs("".to_string(),"c".to_string());
        instr.uses_imm = true;
        instr.set_imm(8);},
        0x8F => {},
        0x90 => {},
        0x91 => {},
        0x92 => {},
        0x93 => {},
        0x94 => {},
        0x95 => {},
        0x96 => {instr.set_name("LD nn,n".to_string());
                instr.set_cputime(8);
                instr.set_regs("".to_string(),"b".to_string());
                instr.uses_imm = true;
                instr.set_imm(8);
                },
        0x97 => {},
        0x98 => {},
        0x99 => {},
        0x9A => {},
        0x9B => {},
        0x9C => {},
        0x9D => {},
        0x9E => {instr.set_name("LD nn,n".to_string());
        instr.set_cputime(8);
        instr.set_regs("".to_string(),"c".to_string());
        instr.uses_imm = true;
        instr.set_imm(8);},
        0x9F => {},
        0xA0 => {},
        0xA1 => {},
        0xA2 => {},
        0xA3 => {},
        0xA4 => {},
        0xA5 => {},
        0xA6 => {instr.set_name("LD nn,n".to_string());
                instr.set_cputime(8);
                instr.set_regs("".to_string(),"b".to_string());
                instr.uses_imm = true;
                instr.set_imm(8);
                },
        0xA7 => {},
        0xA8 => {},
        0xA9 => {},
        0xAA => {},
        0xAB => {},
        0xAC => {},
        0xAD => {},
        0xAE => {instr.set_name("LD nn,n".to_string());
        instr.set_cputime(8);
        instr.set_regs("".to_string(),"c".to_string());
        instr.uses_imm = true;
        instr.set_imm(8);},
        0xAF => {},
        0xB0 => {},
        0xB1 => {},
        0xB2 => {},
        0xB3 => {},
        0xB4 => {},
        0xB5 => {},
        0xB6 => {instr.set_name("LD nn,n".to_string());
                instr.set_cputime(8);
                instr.set_regs("".to_string(),"b".to_string());
                instr.uses_imm = true;
                instr.set_imm(8);
                },
        0xB7 => {},
        0xB8 => {},
        0xB9 => {},
        0xBA => {},
        0xBB => {},
        0xBC => {},
        0xBD => {},
        0xBE => {instr.set_name("LD nn,n".to_string());
        instr.set_cputime(8);
        instr.set_regs("".to_string(),"c".to_string());
        instr.uses_imm = true;
        instr.set_imm(8);},
        0xBF => {},
        0xC0 => {},
        0xC1 => {},
        0xC2 => {},
        0xC3 => {},
        0xC4 => {},
        0xC5 => {},
        0xC6 => {instr.set_name("LD nn,n".to_string());
                instr.set_cputime(8);
                instr.set_regs("".to_string(),"b".to_string());
                instr.uses_imm = true;
                instr.set_imm(8);
                },
        0xC7 => {},
        0xC8 => {},
        0xC9 => {},
        0xCA => {},
        0xCB => {},
        0xCC => {},
        0xCD => {},
        0xCE => {instr.set_name("LD nn,n".to_string());
        instr.set_cputime(8);
        instr.set_regs("".to_string(),"c".to_string());
        instr.uses_imm = true;
        instr.set_imm(8);},
        0xCF => {},
        0xD0 => {},
        0xD1 => {},
        0xD2 => {},
        0xD3 => {},
        0xD4 => {},
        0xD5 => {},
        0xD6 => {instr.set_name("LD nn,n".to_string());
                instr.set_cputime(8);
                instr.set_regs("".to_string(),"b".to_string());
                instr.uses_imm = true;
                instr.set_imm(8);
                },
        0xD7 => {},
        0xD8 => {},
        0xD9 => {},
        0xDA => {},
        0xDB => {},
        0xDC => {},
        0xDD => {},
        0xDE => {instr.set_name("LD nn,n".to_string());
        instr.set_cputime(8);
        instr.set_regs("".to_string(),"c".to_string());
        instr.uses_imm = true;
        instr.set_imm(8);},
        0xDF => {},
        0xE0 => {},
        0xE1 => {},
        0xE2 => {},
        0xE3 => {},
        0xE4 => {},
        0xE5 => {},
        0xE6 => {instr.set_name("LD nn,n".to_string());
                instr.set_cputime(8);
                instr.set_regs("".to_string(),"b".to_string());
                instr.uses_imm = true;
                instr.set_imm(8);
                },
        0xE7 => {},
        0xE8 => {},
        0xE9 => {},
        0xEA => {},
        0xEB => {},
        0xEC => {},
        0xED => {},
        0xEE => {instr.set_name("LD nn,n".to_string());
        instr.set_cputime(8);
        instr.set_regs("".to_string(),"c".to_string());
        instr.uses_imm = true;
        instr.set_imm(8);},
        0xEF => {},
        0xF0 => {},
        0xF1 => {},
        0xF2 => {},
        0xF3 => {},
        0xF4 => {},
        0xF5 => {},
        0xF6 => {instr.set_name("LD nn,n".to_string());
                instr.set_cputime(8);
                instr.set_regs("".to_string(),"b".to_string());
                instr.uses_imm = true;
                instr.set_imm(8);
                },
        0xF7 => {},
        0xF8 => {},
        0xF9 => {},
        0xFA => {},
        0xFB => {},
        0xFC => {},
        0xFD => {},
        0xFE => {instr.set_name("LD nn,n".to_string());
        instr.set_cputime(8);
        instr.set_regs("".to_string(),"c".to_string());
        instr.uses_imm = true;
        instr.set_imm(8);},
        0xFF => {},



    }
}





