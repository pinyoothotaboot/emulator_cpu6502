use crate::interfaces::bus::IBus;
use crate::ram::model::Memory;

impl Memory {
    pub fn new() -> Self {
        Memory {
            stacks: [0x00; 65536],
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

        return self.stacks[*address as usize];
    }

    fn write(&mut self, address: &u16, data: u8) {
        if self.in_mem(address) {
            self.stacks[*address as usize] = data;
        }
    }
}
