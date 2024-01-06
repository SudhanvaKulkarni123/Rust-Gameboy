use std;

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
        }
    }

    pub fn write_to(&mut self, reg : String, val : u32) {
        match reg {
        String::from("a") => {self.a = val as u8;},
        String::from("b") => {self.b = val as u8;},
        String::from("c") => {self.c = val as u8;},
        String::from("d") => {self.d = val as u8;},
        String::from("e") => {self.e = val as u8;},
         String::from("f") => {self.f = val as u8;},
        String::from("h") => {self.h = val as u8;},
        String::from("h") => {self.h = val as u8;},
        String::from("sp") => {self.sp = val as u32;},
        String::from("pc") => {self.pc = val as u32;},

        }

    }

    pub fn read_from(&mut self, reg : String) -> u32 {
        match reg {
            String::from("a") => self.a as u32,
            String::from("b") => self.b  as u32,
            String::from("c") => self.c  as u32,
            String::from("d") => self.d  as u32,
            String::from("e") => self.e  as u32,
             String::from("f") => self.f as u32,
            String::from("h") => self.h as u32,
            String::from("h") => self.h as u32,
            String::from("sp") => self.sp as u32,
            String::from("pc") => self.pc as u32,
    
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
        self.h << 8 + self.l
    }
    pub fn bc(&mut self) -> u16 {
        self.b << 8 + self.c
    }
    pub fn de(&mut self) -> u16 {
        self.d << 8 + self.e
    }
    


}