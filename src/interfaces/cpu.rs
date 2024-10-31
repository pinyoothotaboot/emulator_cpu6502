pub trait ICPU {
    fn reset(&mut self);

    fn run(&mut self);
}
