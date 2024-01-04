#[cfg(feature = "serialize")]
use serde::{Serialize, Serializer};
use std;


#[derive(Copy, Clone, Debug, PartialEq)]
struct FlagsRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool
 }

 impl FlagsRegister {
 pub fn new() -> FlagsRegister {
    FlagsRegister {
        zero: false,
        subtract: false,
        half_carry: false,
        carry: false,
    }
}
}

 impl std::convert::From<FlagsRegister> for u8  {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero       { 1 } else { 0 }) << 7 |
        (if flag.subtract   { 1 } else { 0 }) << 6 |
        (if flag.half_carry { 1 } else { 0 }) << 5 |
        (if flag.carry      { 1 } else { 0 }) << 4
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> 7) & 0b1) != 0;
        let subtract = ((byte >> 6) & 0b1) != 0;
        let half_carry = ((byte >>5) & 0b1) != 0;
        let carry = ((byte >> 4) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}


#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
  }

  impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister::new(),
            h: 0,
            l: 0,
        }
    }

  fn get_bc(&self) -> u16 {
    (self.b as u16) << 8
    | self.c as u16
  }

  fn set_bc(&mut self, value: u16) {
    self.b = ((value & 0xFF00) >> 8) as u8;
    self.c = (value & 0xFF) as u8;
  }

  fn get_af(&self) -> u16 {
    (self.a as u16) << 8 | (self.f as u16)
  }

  fn set_af(&mut self, value: u16) {
    self.a = ((value & 0xFF00) >> 8) as u8;
    self.f = (value & 0xFF) as u8;
  }

  fn get_de(&self) -> u16 {
    (self.d as u16) << 8 | (self.e as u16)
  }

  fn set_de(&mut self, value: u16) {
    self.d = ((value & 0xFF00) >> 8) as u8;
    self.e = (value & 0xFF) as u8;
  }


  fn get_hl(&self) -> u16 {
    (self.h as u16) << 8 | (self.l as u16)
  }

  fn set_hl(&mut self, value: u16) {
    self.h = ((value & 0xFF00) >> 8) as u8;
    self.l = (value & 0xFF) as u8;
  }

}

struct Flags {
    zero : bool,
    subtract : bool,
    half_carry : bool,
    carry : bool,
}




 


 