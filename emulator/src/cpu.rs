use std;
use crate::enums::Reg;
pub struct CPU {
    //add all registers
    pub a : u8,
    pub b : u8,
    pub c : u8,
    pub d : u8,
    pub e : u8,
    pub f : u8,     //flag register
    pub h : u8,
    pub l : u8,
    pub sp : u16,   //stack pointer
    pub pc : u16,   //program counter
    pub num_cycles: u32,

}
//NOTE : Gameboy processes all values in Big endian so everything must be reversed when we are decoding in software
impl CPU {
    pub fn new()-> CPU {
        CPU {
            a : 0,
            b : 0,
            c : 0,
            d : 0,
            e : 0,
            f : 0,
            h : 0,
            l : 0,
            sp : 0xFFFE,
            pc : 0x100,         //look at http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf for ocumentation
            num_cycles : 0,
            }
    }

    pub fn write_to(&mut self, reg : Reg, val : u32) {
        match reg {
        Reg::A => {self.a = val as u8;},
        Reg::B => {self.b = val as u8;},
        Reg::C => {self.c = val as u8;},
        Reg::D => {self.d = val as u8;},
        Reg::E => {self.e = val as u8;},
        Reg::F => {self.f = val as u8;},
        Reg::H => {self.h = val as u8;},
        Reg::L => {self.l = val as u8;},
        Reg::SP => {self.sp = val as u16;},
        Reg::PC => {self.pc = val as u16;},
        _ => {},

        }

    }

    pub fn read_from(&mut self, reg : Reg) -> u32 {
        match reg {
            Reg::A => self.a as u32,
            Reg::B => self.b  as u32,
            Reg::C => self.c  as u32,
            Reg::D => self.d  as u32,
            Reg::E => self.e  as u32,
            Reg::F => self.f as u32,
            Reg::H => self.h as u32,
            Reg::L => self.l as u32,
            Reg::SP => self.sp as u32,
            Reg::PC => self.pc as u32,
            _ => 0,
    
            }
    }

    pub fn set_zero_flag(&mut self)  {
        self.f = self.f | 0b10000000;
    }
    pub fn rst_zero_flag(&mut self)  {
        self.f = self.f & 0b01111111;
    }

    pub fn set_sub_flag(&mut self) {
        self.f = self.f | 0b01000000;
    }
    pub fn rst_sub_flag(&mut self)  {
        self.f = self.f & 0b10111111;
    }
    pub fn set_half_carry_flag(&mut self)  {
        self.f = self.f | 0b00100000;
    }
    pub fn rst_half_carry_flag(&mut self)  {
        self.f = self.f & 0b11011111;
    }
    pub fn set_carry_flag(&mut self)  {
        self.f = self.f | 0b00010000;
    }
    pub fn rst_carry_flag(&mut self)  {
        self.f = self.f & 0b11101111;
    }

    pub fn hl(&mut self) -> u16 {
        (self.h as u16) << 8 + self.l as u16
    }
    pub fn bc(&mut self) -> u16 {
        (self.b as u16) << 8 + self.c as u16
    }
    pub fn de(&mut self) -> u16 {
        (self.d as u16) << 8 + self.e as u16
    }
    pub fn sp(&self) -> u16 {
        self.sp
    }

    pub fn set_hl(&mut self, val : u16) {
        self.h = (val >> 8) as u8;
        self.l = (val & 0xFF) as u8; 
    }
    pub fn set_bc(&mut self, val : u16) {
        self.b = (val >> 8) as u8;
        self.c = (val & 0xFF) as u8;
    }
    pub fn set_de(&mut self, val : u16) {
        self.d = (val >> 8) as u8;
        self.c = (val & 0xFF) as u8;
    }

    pub fn set_sp(&mut self, val : u16) {
        self.sp = val;
    }

    pub fn set16reg(&mut self, reg : Reg, val : u16) {
        match reg {
            "hl" => {self.set_hl(val);},
            "bc" => {self.set_bc(val);},
            "de" => {self.set_de(val);},
            "sp" => {self.set_sp(val);},
        }
    }

    pub fn dec_hl(&mut self) {
        let mut hl = self.hl();
        hl = hl - 1;
        self.set_hl(hl);
    }
    pub fn inc_hl(&mut self) {
        let mut hl = self.hl();
        hl = hl + 1;
        self.set_hl(hl);
    }
    


}