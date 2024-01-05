use std;
pub struct Instr {
    pub CPU_cycles : u8,
    pub op_type : String,
    pub target_addr : u8,
    pub src_addr : u8,
    pub opcode : u8,
    pub immm : u16,
    pub pc_incr : u8,
}



pub trait ExecInstr {
    fn execute(&self);
    fn set_src_dst(&self, src : u8, dst : u8);
    fn set_imm(&self, r1 : u8, r2: u8);
    fn set_cputime(&self, num_cycles : u8);
}



impl ExecInstr for Instr {
    fn execute(&self) {
        
    }
    fn set_cputime(&self, num_cycles : u8) {
        
    }
    fn set_imm(&self, r1 : u8, r2: u8) {
        
    }
    fn set_src_dst(&self, src : u8, dst : u8) {
        
    }

}





