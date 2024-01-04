use std;

struct GameBoyMEM {
    pub memory : [u8; 65536],
} 

impl GameBoyMEM {
    pub fn new() -> GameBoyMEM {
    GameBoyMEM {memory : [u8;0],}
    }

    pub fn write_at(&mut self, value : u8, addr : u16) -> Result{

    }
}