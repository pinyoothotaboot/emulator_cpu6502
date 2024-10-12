use crate::interface::IBus;

pub struct Stack {
    stack: [u8; 256],
}

impl Stack {
    pub fn new() -> Self {
        Stack { stack: [0x00; 256] }
    }

    fn in_mem(&self, address: &u16) -> bool {
        if *address >= 0x0100 && *address <= 0x01FFF {
            return true;
        }
        return false;
    }
}

impl IBus for Stack {
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
