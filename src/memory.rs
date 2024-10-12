//  _______________ $10000  _______________
// | PRG-ROM       |       |               |
// | Upper Bank    |       |               |
// |_ _ _ _ _ _ _ _| $C000 | PRG-ROM       |
// | PRG-ROM       |       |               |
// | Lower Bank    |       |               |
// |_______________| $8000 |_______________|
// | SRAM          |       | SRAM          |
// |_______________| $6000 |_______________|
// | Expansion ROM |       | Expansion ROM |
// |_______________| $4020 |_______________|
// | I/O Registers |       |               |
// |_ _ _ _ _ _ _ _| $4000 |               |
// | Mirrors       |       | I/O Registers |
// | $2000-$2007   |       |               |
// |_ _ _ _ _ _ _ _| $2008 |               |
// | I/O Registers |       |               |
// |_______________| $2000 |_______________|
// | Mirrors       |       |               |
// | $0000-$07FF   |       |               |
// |_ _ _ _ _ _ _ _| $0800 |               |
// | RAM           |       | RAM           |
// |_ _ _ _ _ _ _ _| $0200 |               |
// | Stack         |       |               |
// |_ _ _ _ _ _ _ _| $0100 |               |
// | Zero Page     |       |               |
// |_______________| $0000 |_______________|
use crate::interface::IBus;
pub struct Memory {
    pub stack: [u8; 65536],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            stack: [0x00; 65536],
        }
    }

    fn in_mem(&self, address: &u16) -> bool {
        if *address >= 0x0000 && *address <= 0xFFFF {
            return true;
        }
        return false;
    }
}

impl IBus for Memory {
    fn read(&self, address: &u16) -> u8 {
        if !self.in_mem(address) {
            return 0x00;
        }

        return self.stack[*address as usize];
    }

    fn write(&mut self, address: &u16, data: u8) {
        if self.in_mem(address) {
            self.stack[*address as usize] = data;
        }
    }
}
