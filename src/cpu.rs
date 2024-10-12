use crate::instruction::Instruction;
use crate::instruction::CPU_6502_OPERATION_CODES_MAP;
use crate::interface::ICPU;
use crate::memory::{IMemory, Memory};
use crate::processor_status::ProcessorStatus;
use crate::stack::Stack;
pub struct CPU {
    /*
       Program Counter (PC) - holds the address for
       the next machine language instruction to be executed.
    */
    pc: u16,
    address: u16,
    memory: Memory,
    data: u8,
    /*
       Stack Pointer - Memory space [0x0100 .. 0x1FF] is used for stack.
       The stack pointer holds the address of the top of that space.
       NES Stack (as all stacks) grows from top to bottom: when a byte gets pushed to the stack,
       SP register decrements. When a byte is retrieved from the stack, SP register increments.
    */
    stack: Stack,
    /*
       Index Register X (X) - used as an offset in specific memory addressing modes (more on this later).
       Can be used for auxiliary storage needs (holding temp values, being used as a counter, etc.)
    */
    x_reg: u8,
    /*
       Index Register Y (Y) - similar use cases as register X.
    */
    y_reg: u8,
    /*
       Accumulator (A) - stores the results of arithmetic, logic,
       and memory access operations. It used as an input parameter for some operations.
    */
    accumulator: u8,
    decoder: Option<&'static Instruction>,
    /*
       Processor status (P) - 8-bit register represents 7 status flags
       that can be set or unset depending on the result of
       the last executed instruction (for example Z flag is set (1)
       if the result of an operation is 0, and is unset/erased (0) otherwise)
    */
    status: ProcessorStatus,
    address_register: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            pc: 0x0000,
            address: 0x0000,
            memory: Memory::new(),
            data: 0x00,
            stack: Stack::new(),
            x_reg: 0x00,
            y_reg: 0x00,
            accumulator: 0x00,
            decoder: None,
            status: ProcessorStatus::new(),
            address_register: 0x0000,
        }
    }

    fn fetch(&mut self) {
        // Load 16-bit from program counter (PC) and set to address
        self.address = self.pc.clone();

        // Add one program counter (PC)
        // PC = PC + 1
        self.pc += 1;

        // Load data from memory 8-bit.And store data
        self.data = self.memory.read(&self.address);

        // Decoder operation instuction
        self.decoder = CPU_6502_OPERATION_CODES_MAP.get(&self.data).cloned();
    }

    fn execute(&mut self) {}
}

impl ICPU for CPU {
    fn reset(&mut self) {
        // Reset program counter
        // Ref : https://www.c64-wiki.com/wiki/Reset_(Process)
        self.pc = 0xFFFC;

        // Reset processor status
        self.status.reset();

        // Reset A,X,Y Register
        self.accumulator = 0x00;
        self.x_reg = 0x00;
        self.y_reg = 0x00;

        // Reset Address 16-bit
        self.address = 0x0000;

        // Reset Address Register [Hi,Lo] 16-bit
        self.address_register = 0x0000;

        // Reset decoder
        self.decoder = None;
    }

    fn run(&mut self) {}
}
