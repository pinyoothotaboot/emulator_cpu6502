use crate::constants::START_STACK_POINTER;
use crate::instruction::Instruction;
use crate::instruction::CPU_6502_OPERATION_CODES_MAP;
use crate::interface::{IBus, ICPU};
use crate::memory::Memory;
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
    stack_pointer: u8,
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
            stack_pointer: 0x00,
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
        self.address_register = 0x00;

        // Reset decoder
        self.decoder = None;

        // Reset stack pointer
        self.stack_pointer = 0x00FD;
    }

    fn run(&mut self) {}
}

impl IBus for CPU {
    fn read(&self, address: &u16) -> u8 {
        return self.memory.read(address);
    }

    fn write(&mut self, address: &u16, data: u8) {
        self.memory.write(address, data);
    }
}

/**
* Stack Instructions
   These instructions are implied mode, have a length of one byte and require machine cycles as indicated.
   The "PuLl" operations are known as "POP" on most other microprocessors. With the 6502,
   the stack is always on page one ($100-$1FF) and works top down.
*/
impl CPU {
    /**
    *  PHA - Push Accumulator
       Pushes a copy of the accumulator on to the stack.

       Processor Status after use:

       C	Carry Flag	Not affected
       Z	Zero Flag	Not affected
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Not affected

       MODE           SYNTAX       HEX LEN TIM
       Implied         PHA         $48  1   3
    */
    fn PHA(&mut self) {
        // A -> Stack
        let addr_sp: u16 = (START_STACK_POINTER + self.stack_pointer as u16 & 0x00FF).into();

        // Write Accumulator to memory
        // data = accumulator
        self.write(&addr_sp, self.accumulator);

        // S = S - 1
        self.stack_pointer -= 1;
    }

    /**
    *  PHP - Push Processor Status
       Pushes a copy of the status flags on to the stack.

       Processor Status after use:

       C	Carry Flag	Not affected
       Z	Zero Flag	Not affected
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Not affected

       MODE           SYNTAX       HEX LEN TIM
       Implied         PHP         $08  1   3
    */
    fn PHP(&mut self) {
        // N V _ B D I Z C <-- Flag
        let status: u8 = self.status.get_status();

        let addr_sp: u16 = (START_STACK_POINTER + self.stack_pointer as u16 & 0x00FF).into();
        // Write process status to memory
        // data = process status
        self.write(&addr_sp, status);

        // S = S - 1
        self.stack_pointer -= 1;
    }

    /**
    *  PLA - Pull Accumulator
       Pulls an 8 bit value from the stack and into the accumulator. The zero and negative flags are set as appropriate.

       C	Carry Flag	Not affected
       Z	Zero Flag	Set if A = 0
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Set if bit 7 of A is set

       MODE           SYNTAX       HEX LEN TIM
       Implied         PLA         $68  1   4
    */
    fn PLA(&mut self) {
        // S = S + 1
        self.stack_pointer += 1;

        // Pulls an 8 bit value from the stack and into the accumulator
        let addr_sp: u16 = (START_STACK_POINTER + self.stack_pointer as u16 & 0x00FF).into();
        self.accumulator = self.read(&addr_sp);

        // The zero and negative flags are set as appropriate.
        // Z	Zero Flag	Set if A = 0
        if self.accumulator - 0x00 == 0 {
            self.status.set_zero();
        }

        // N	Negative Flag	Set if bit 7 of A is set
        if self.accumulator & 0b1000_0000 > 0 {
            self.status.set_negative();
        }
    }

    /**
    *  PLP - Pull Processor Status
       Pulls an 8 bit value from the stack and into the processor flags.
       The flags will take on new states as determined by the value pulled.

       Processor Status after use:

       C	Carry Flag	Set from stack
       Z	Zero Flag	Set from stack
       I	Interrupt Disable	Set from stack
       D	Decimal Mode Flag	Set from stack
       B	Break Command	Set from stack
       V	Overflow Flag	Set from stack
       N	Negative Flag	Set from stack

       MODE           SYNTAX       HEX LEN TIM
       Implied         PLP         $28  1   4
    */
    fn PLP(&mut self) {
        // S = S + 1
        self.stack_pointer += 1;

        // Pulls an 8 bit value from the stack and into the processor flags.
        let addr_sp: u16 = (START_STACK_POINTER + self.stack_pointer as u16 & 0x00FF).into();
        let data: u8 = self.read(&addr_sp);

        self.status.set_status(data);
    }
}

/**
 * Status Register Instructions
 */
impl CPU {
    /**
    *  CLC - Clear Carry Flag
       C = 0

       Set the carry flag to zero.

       C	Carry Flag	Set to 0
       Z	Zero Flag	Not affected
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Not affected

       MODE           SYNTAX       HEX LEN TIM
       Implied         CLC         $18  1   2
    */
    fn CLC(&mut self) {
        // C = 0
        self.status.unset_carry();
    }

    /**
    *  CLD - Clear Decimal Mode
       D = 0

       Sets the decimal mode flag to zero.

       C	Carry Flag	Not affected
       Z	Zero Flag	Not affected
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Set to 0
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Not affected

       MODE           SYNTAX       HEX LEN TIM
       Implied         CLD         $D8  1   2
    */
    fn CLD(&mut self) {
        // D = 0
        self.status.unset_decimal_mode();
    }

    /**
    *  CLI - Clear Interrupt Disable
       I = 0

       Clears the interrupt disable flag allowing normal interrupt requests to be serviced.

       C	Carry Flag	Not affected
       Z	Zero Flag	Not affected
       I	Interrupt Disable	Set to 0
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Not affected

       MODE           SYNTAX       HEX LEN TIM
       Implied         CLI         $58  1   2
    */
    fn CLI(&mut self) {
        // I = 0
        self.status.unset_interrupt_disable();
    }

    /**
    *  CLV - Clear Overflow Flag
       V = 0

       Clears the overflow flag.

       C	Carry Flag	Not affected
       Z	Zero Flag	Not affected
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Set to 0
       N	Negative Flag	Not affected

       MODE           SYNTAX       HEX LEN TIM
       Implied         CLV         $B8  1   2
    */
    fn CLV(&mut self) {
        // V = 0
        self.status.unset_overflow();
    }

    /**
    *  SEC - Set Carry Flag
       C = 1

       Set the carry flag to one.

       C	Carry Flag	Set to 1
       Z	Zero Flag	Not affected
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Not affected

       MODE           SYNTAX       HEX LEN TIM
       Implied         SEC         $38  1   2
    */
    fn SEC(&mut self) {
        // C = 1
        self.status.set_carry();
    }

    /**
    *  SED - Set Decimal Flag
       D = 1

       Set the decimal mode flag to one.

       C	Carry Flag	Not affected
       Z	Zero Flag	Not affected
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Set to 1
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Not affected

       MODE           SYNTAX       HEX LEN TIM
       Implied         SED         $F8  1   2
    */
    fn SED(&mut self) {
        // D = 1
        self.status.set_decimal_mode();
    }

    /**
    *  SEI - Set Interrupt Disable
       I = 1

       Set the interrupt disable flag to one.

       C	Carry Flag	Not affected
       Z	Zero Flag	Not affected
       I	Interrupt Disable	Set to 1
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Not affected

       MODE           SYNTAX       HEX LEN TIM
       Implied         SEI         $78  1   2
    */
    fn SEI(&mut self) {
        // I = 1
        self.status.set_interrupt_disable();
    }
}
