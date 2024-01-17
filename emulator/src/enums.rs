use std;
pub enum Reg{
    A,B,C,D,E,F,H,L,SP,PC,HL, DE, BC, NULL
}

pub enum Loads {
    LD1, LD2 , LD3, LD4, LD5, NULL
}

pub enum Ariths {
P, M, DEC, INC, NULL
}