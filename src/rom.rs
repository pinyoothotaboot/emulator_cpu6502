use crate::interface::IBus;
pub struct Rom {
    pub stack: [u8; 65536],
}

impl Rom {
    pub fn new(stack: [u8; 65536]) -> Self {
        Rom { stack }
    }

    pub fn display(&self) {
        for (idx, data) in self.stack.iter().enumerate() {
            if *data == 0x00 {
                continue;
            }
            println!("{:#04x?}  =  {:#02x?}", idx, *data);
        }
    }
}

impl IBus for Rom {
    fn read(&self, address: &u16) -> u8 {
        return self.stack[*address as usize];
    }

    fn write(&mut self, address: &u16, data: u8) {
        // Not implement
    }
}
