use std::fs::File;
use std::vec::Vec;

use std::os::unix::fs::FileExt;





fn main() {
    let mut f : File = File::open("DMG_ROM.bin").unwrap();  //loads bootrom
    let mut file: [u8; 256] = [0;256];
    f.read_at(&mut file, 0);
    // for byte in file {
    //     println!("{}", byte);
    // }



}


