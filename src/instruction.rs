use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Instruction {
    pub code: u8,
    pub name: &'static str,
    pub length: u8,
    pub cycle: u8,
}

impl Instruction {
    pub fn new(code: u8, name: &'static str, length: u8, cycle: u8) -> Self {
        Instruction {
            code,
            name,
            length,
            cycle,
        }
    }
}

lazy_static! {
    pub static ref CPU_6502_OPERATION_CODES: Vec<Instruction> =
        vec![Instruction::new(0xA9, "LDA", 2, 2)];
    pub static ref CPU_6502_OPERATION_CODES_MAP: HashMap<u8, &'static Instruction> = {
        let mut map = HashMap::new();
        for cpu_6502_operation_code in &*CPU_6502_OPERATION_CODES {
            map.insert(cpu_6502_operation_code.code, cpu_6502_operation_code);
        }
        map
    };
}
