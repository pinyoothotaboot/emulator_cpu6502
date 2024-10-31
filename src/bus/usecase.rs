use crate::bus::model::Bus;
use crate::interfaces::bus::IBus;
use crate::ram::model::Memory;

impl Bus {
    pub fn new() -> Self {
        Bus {
            memory: Memory::new(),
        }
    }
}

impl IBus for Bus {
    fn read(&self, address: &u16) -> u8 {
        self.memory.read(address)
    }

    fn write(&mut self, address: &u16, data: u8) {
        self.memory.write(address, data);
    }
}
