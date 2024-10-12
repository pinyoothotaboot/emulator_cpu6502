pub trait ICPU {
    fn reset(&mut self);

    fn run(&mut self);
}

pub trait IBus {
    fn read(&self, address: &u16) -> u8;

    fn write(&mut self, address: &u16, data: u8);
}
