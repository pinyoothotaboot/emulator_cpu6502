mod constants;
mod cpu;
mod datas;
mod instruction;
mod interface;
mod memory;
mod processor_status;
mod rom;
mod stack;

use interface::ICPU;

use crate::cpu::CPU;
use crate::datas::snakes::SNAKE_MEMORY_MAPS;
use crate::rom::Rom;
fn main() {
    let rom = Rom::new(*SNAKE_MEMORY_MAPS);
    let mut cpu = CPU::new(rom);
    cpu.reset();
    cpu.run();
    //cpu.display_rom();
    println!("End Program..");
}
