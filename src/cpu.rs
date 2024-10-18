use std::time::{Duration, SystemTime};

use crate::constants::START_STACK_POINTER;
use crate::instruction;
use crate::instruction::Instruction;
use crate::instruction::CPU_6502_OPERATION_CODES_MAP;
use crate::interface::{IBus, ICPU};
use crate::memory::Memory;
use crate::processor_status::ProcessorStatus;
use crate::rom::Rom;
use crate::stack::Stack;

pub enum State {
    Fetch,
    Decode,
    Execute,
    Exit,
}

pub struct CPU {
    /*
       Program Counter (PC) - holds the address for
       the next machine language instruction to be executed.
    */
    pc: u16,
    address: u16,
    /**
     * The ROM load datas
     */
    rom: Rom,
    /**
     * The CPU6502 memory stack for R/W
     */
    memory: Memory,
    /**
     * The word 8-bit
     */
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
    /**
     * CPU6502 instruction decoded
     */
    instruction_reg: Option<&'static Instruction>,
    /*
       Processor status (P) - 8-bit register represents 7 status flags
       that can be set or unset depending on the result of
       the last executed instruction (for example Z flag is set (1)
       if the result of an operation is 0, and is unset/erased (0) otherwise)
    */
    status: ProcessorStatus,

    address_register: u16,
    cycles: u16,
    state: State,
    prev_cycle_count: u16,
}

impl CPU {
    pub fn new(rom: Rom) -> Self {
        CPU {
            pc: 0x0000,
            address: 0x0000,
            memory: Memory::new(),
            rom,
            data: 0x00,
            stack_pointer: 0x00,
            x_reg: 0x00,
            y_reg: 0x00,
            accumulator: 0x00,
            instruction_reg: None,
            status: ProcessorStatus::new(),
            address_register: 0x0000,
            cycles: 0x0000,
            state: State::Fetch,
            prev_cycle_count: 0x0000,
        }
    }

    fn fetch(&mut self) {
        // Load 16-bit from program counter (PC) and set to address
        // PC -> Address
        // Example :
        //           PC = 0x0000
        //           Address = 0x0000
        self.address = self.pc.clone();

        // Add one program counter (PC)
        // PC = PC + 1
        // Example :
        //           PC = 0x0000 + 0x0001
        //           PC = 0x0001
        self.pc += 1;

        // Load data from cpu6502 instruction memory 8-bit.And store data
        // Instrunction Memory 8-bit -> Data
        // Example :
        //           Memory = 0x0000 : 4C
        //           Data   = 0x4C
        self.data = self.read_rom(&self.address);
    }

    fn decode(&mut self) {
        // Decoder operation instuction
        // Example :
        //           Data = 0x4C
        //           Instruction Register =
        //                                   code  = 0x4C
        //                                   name  = JMP
        //                                   len   = 3 (Byte)
        //                                   Cycle = 3  (tim)
        //
        let address_instruction: u8 = self.read_rom(&self.address);
        self.instruction_reg = CPU_6502_OPERATION_CODES_MAP
            .get(&address_instruction)
            .copied();
    }

    // TODO :: Maybe not implement in global
    fn execute(&mut self) {
        // Load 16-bit from program counter (PC) and set to address
        // PC -> Address
        // Example :
        //           PC = 0x0001
        //           Address = 0x0001
        self.address = self.pc.clone();

        // Add one program counter (PC)
        // PC = PC + 1
        // Example :
        //           PC = 0x0001 + 0x0001
        //           PC = 0x0002
        self.pc += 1;

        // Load data from ROM
        self.data = self.read_rom(&self.address);
    }

    fn update_clock(&mut self) {
        match self.instruction_reg {
            Some(instruction) => {
                self.cycles += instruction.cycle as u16 & 0x00FF;
            }
            None => {}
        }
    }
}

impl CPU {
    pub fn display_rom(&self) {
        self.rom.display();
    }

    fn debug(&self, state: &str) {
        println!("======================================");
        println!("State         = {:?}", state);
        println!("PC            = {:#04x?}", self.pc);
        println!("Address       = {:#04x?}", self.address);
        println!("Address Reg   = {:#04x?}", self.address_register);
        println!("Data          = {:#02x?}", self.data);
        match self.instruction_reg {
            Some(instruction) => {
                println!(
                    "Decoded       = {:?} ${:#02x?}",
                    instruction.name, instruction.code
                );
            }
            None => {}
        }
        println!("X Reg         = {:#02x?}", self.x_reg);
        println!("Y Reg         = {:#02x?}", self.y_reg);
        println!("Accumulator   = {:#02x?}", self.accumulator);
        println!("Status        = {:#010b}", self.status.get_status());
        println!("Cycle         = {:?}", self.cycles);
    }
}

impl ICPU for CPU {
    fn reset(&mut self) {
        // Reset program counter
        // Ref : https://www.c64-wiki.com/wiki/Reset_(Process)
        //self.pc = 0xFFFC;
        self.pc = 0x0600;

        // Initial Clock
        let data = self.read_rom(&self.pc);
        match CPU_6502_OPERATION_CODES_MAP.get(&data).cloned() {
            Some(instruction) => {
                self.cycles = instruction.cycle as u16;
            }
            None => {}
        }

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
        self.instruction_reg = None;

        // Reset stack pointer
        self.stack_pointer = 0x00FD;
    }

    fn run(&mut self) {
        loop {
            let duration_since_epoch = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();
            let cuurrent_time: u128 = duration_since_epoch.as_nanos();

            // Process Instruction With State
            match self.state {
                State::Fetch => {
                    // State fetch
                    self.fetch();
                    self.state = State::Decode;
                    //self.debug("Fetch");
                }
                State::Decode => {
                    // State Decode
                    self.decode();
                    // Update clock cycle
                    self.update_clock();
                    //self.debug("Decode");
                    self.state = State::Execute;
                }
                State::Execute => {
                    //self.debug("Execute");
                    // Handle Instruction
                    self.execute_instruction();
                    //self.debug("Execute");
                }
                State::Exit => {
                    break;
                }
                _ => {
                    panic!("Program has problem!.");
                }
            }
            /**
             * Ref : https://www.cs.cornell.edu/~kt/post/6502-7/
             */
            let cycle_count = self.cycles.clone();
            let new_cycle = cycle_count - self.prev_cycle_count;
            let elaped_time = (480 * new_cycle) as u128;
            while SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
                - cuurrent_time
                < elaped_time
            {
                continue;
            }

            // Update prev count
            self.prev_cycle_count = cycle_count;
        }
    }
}

/**
 * Execute Instructions
 */
impl CPU {
    fn execute_instruction(&mut self) {
        match self.instruction_reg {
            Some(instruction) => {
                match instruction.code {
                    /* LDA - Load accumulator */
                    0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                        self.LDA();
                    }
                    /* LDX Load X Register */
                    0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => {
                        self.LDX();
                    }
                    /* LDY Load Y Register */
                    0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => {
                        self.LDY();
                    }
                    /* CPX Compare X Register */
                    0xE0 | 0xE4 | 0xEC => {
                        self.CPX();
                    }
                    /* CPY Compare Y Register */
                    0xC0 | 0xC4 | 0xCC => {
                        self.CPY();
                    }
                    /* DEX Decrement X Register */
                    0xCA => {
                        self.DEX();
                    }
                    /* DEY Decrement Y Register */
                    0x88 => {
                        self.DEY();
                    }
                    /* INC  Increment Memory */
                    0xE6 | 0xF6 | 0xEE | 0xFE => {
                        self.INC();
                    }
                    /* INX - Increment X Register */
                    0xE8 => {
                        self.INX();
                    }
                    /* INY - Increment Y Register */
                    0xC8 => {
                        self.INY();
                    }
                    /* STA - Store Accumulator */
                    0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                        self.STA();
                    }
                    /* STX - Store X Register */
                    0x86 | 0x96 | 0x8E => {
                        self.STX();
                    }
                    /* STY Store Y Register */
                    0x84 | 0x94 | 0x8C => {
                        self.STY();
                    }
                    /* TAX - Transfer Accumulator to X */
                    0xAA => {
                        self.TAX();
                    }
                    /* TAY - Transfer Accumulator to Y */
                    0xA8 => {
                        self.TAY();
                    }
                    /* TSX - Transfer Stack Pointer to X */
                    0xBA => {
                        self.TSX();
                    }
                    /* TXA - Transfer X to Accumulator */
                    0x8A => {
                        self.TXA();
                    }
                    /* TXS - Transfer X to Stack Pointer */
                    0x9A => {
                        self.TXS();
                    }
                    /* TYA - Transfer Y to Accumulator */
                    0x98 => {
                        self.TYA();
                    }
                    /* JSR (Jump to SubRoutine) */
                    0x20 => {
                        self.JSR();
                    },
                    /* JMP - Jump */
                    0x4C | 0x6C => {
                        self.JMP(&instruction.code);
                    },
                    /** CMP (CoMPare accumulator) */
                    0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
                        self.CMP(&instruction.code);
                    },
                    _ => {
                        self.state = State::Exit;
                    }
                }
            }
            None => {
                panic!("Not found instruction");
            }
        }
    }
}

/** R/W Memory */
impl IBus for CPU {
    fn read(&self, address: &u16) -> u8 {
        return self.memory.read(address);
    }

    fn write(&mut self, address: &u16, data: u8) {
        self.memory.write(address, data);
    }
}

/** R Rom */
impl CPU {
    fn read_rom(&self, address: &u16) -> u8 {
        return self.rom.read(address);
    }
}

/**
 * References instructions
 * - https://www.nesdev.org/obelisk-6502-guide/reference.html
 * - http://www.6502.org/tutorials/6502opcodes.html
 */
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

/**
 * AXY Register Instructions
 */
impl CPU {
    /**
    *  LDA - Load Accumulator
       A,Z,N = M

       Loads a byte of memory into the accumulator setting the zero and negative flags as appropriate.

       C	Carry Flag	Not affected
       Z	Zero Flag	Set if A = 0
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Set if bit 7 of A is set

       Affects Flags: N Z

       MODE           SYNTAX       HEX LEN TIM
       Immediate     LDA #$44      $A9  2   2
       Zero Page     LDA $44       $A5  2   3
       Zero Page,X   LDA $44,X     $B5  2   4
       Absolute      LDA $4400     $AD  3   4
       Absolute,X    LDA $4400,X   $BD  3   4+
       Absolute,Y    LDA $4400,Y   $B9  3   4+
       Indirect,X    LDA ($44,X)   $A1  2   6
       Indirect,Y    LDA ($44),Y   $B1  2   5+

       + add 1 cycle if page boundary crossed
    */
    fn LDA(&mut self) {
        // Store PC to Address
        // Example :
        //          PC = 0xDAFC
        //          Address = 0xDAFC
        self.address = self.pc.clone();

        // Next PC = PC + 1
        // Example :
        //          PC = 0xDAFC + 0x0001
        //             = 0xDAFD
        self.pc += 1;

        // Load data from Rom memory
        // Example :
        //          Address = 0xDAFC
        //          Data = 0x0D
        self.data = self.read_rom(&self.address);

        // Loads a byte of memory into the accumulator
        // A = data
        // Example :
        //          Data = 0x0D
        //          Accumulator = 0x0D
        self.accumulator = self.data;

        // setting the zero and negative flags as appropriate.
        // Z	Zero Flag	Set if A = 0
        // Example :
        //          Accumlator = 0x0D  != 0
        //          Z = 0
        if self.accumulator == 0x00 {
            self.status.set_zero();
        } else {
            self.status.unset_zero();
        }

        // N	Negative Flag	Set if bit 7 of A is set
        // Example :
        //          Accumulator = 0b0000_1101 &
        //                        0b1000_0000
        //                      = 0b0000_0000 = 0x00 < 1
        //          N = 0
        if self.accumulator & 0b1000_0000 > 0 {
            self.status.set_negative();
        } else {
            self.status.unset_negative();
        }

        // TODO :: Update Cycles

        // Update state 'Fetch'
        self.state = State::Fetch;
    }

    /**
    *  LDX - Load X Register
       X,Z,N = M

       Loads a byte of memory into the X register setting the zero and negative flags as appropriate.

       C	Carry Flag	Not affected
       Z	Zero Flag	Set if X = 0
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Set if bit 7 of X is set

       LDX (LoaD X register)
       Affects Flags: N Z

       MODE           SYNTAX       HEX LEN TIM
       Immediate     LDX #$44      $A2  2   2
       Zero Page     LDX $44       $A6  2   3
       Zero Page,Y   LDX $44,Y     $B6  2   4
       Absolute      LDX $4400     $AE  3   4
       Absolute,Y    LDX $4400,Y   $BE  3   4+

       + add 1 cycle if page boundary crossed
    */
    fn LDX(&mut self) {
        // Loads a byte of memory into the X register
        // X = data
        self.x_reg = self.data;

        // setting the zero and negative flags as appropriate.
        // Z	Zero Flag	Set if X = 0
        if self.x_reg == 0x00 {
            self.status.set_zero();
        }

        // N	Negative Flag	Set if bit 7 of X is set
        if self.x_reg & 0b1000_0000 > 0 {
            self.status.set_negative();
        }
    }

    /**
    *  LDY - Load Y Register
       Y,Z,N = M

       Loads a byte of memory into the Y register setting the zero and negative flags as appropriate.

       C	Carry Flag	Not affected
       Z	Zero Flag	Set if Y = 0
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Set if bit 7 of Y is set

       Affects Flags: N Z
       MODE           SYNTAX       HEX LEN TIM
       Immediate     LDY #$44      $A0  2   2
       Zero Page     LDY $44       $A4  2   3
       Zero Page,X   LDY $44,X     $B4  2   4
       Absolute      LDY $4400     $AC  3   4
       Absolute,X    LDY $4400,X   $BC  3   4+

       + add 1 cycle if page boundary crossed
    */
    fn LDY(&mut self) {
        // Loads a byte of memory into the Y register
        // Y = data
        self.y_reg = self.data;

        // setting the zero and negative flags as appropriate.
        // Z	Zero Flag	Set if Y = 0
        if self.y_reg == 0x00 {
            self.status.set_zero();
        }

        // N	Negative Flag	Set if bit 7 of Y is set
        if self.y_reg & 0b1000_0000 > 0 {
            self.status.set_negative();
        }
    }

    /**
    *  CPX - Compare X Register
       Z,C,N = X-M

       This instruction compares the contents of the X register with another memory held value
       and sets the zero and carry flags as appropriate.

       Processor Status after use:

       C	Carry Flag	Set if X >= M
       Z	Zero Flag	Set if X = M
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Set if bit 7 of the result is set

       Affects Flags: N Z C
       MODE           SYNTAX       HEX LEN TIM
       Immediate     CPX #$44      $E0  2   2
       Zero Page     CPX $44       $E4  2   3
       Absolute      CPX $4400     $EC  3   4

       Operation and flag results are identical to equivalent mode accumulator CMP ops.
    */
    fn CPX(&mut self) {
        // This instruction compares the contents of the X register with another memory held value
        // X - M
        let temp: u8 = self.x_reg - self.data;

        // C	Carry Flag	Set if X >= M
        if self.x_reg >= self.data {
            self.status.set_carry();
        }

        // Z	Zero Flag	Set if X = M
        if temp == 0 {
            self.status.set_zero();
        }

        // N	Negative Flag	Set if bit 7 of the result is set
        if temp & 0b1000_0000 > 0 {
            self.status.set_negative();
        }
    }

    /**
    *  CPY - Compare Y Register
       Z,C,N = Y-M

       This instruction compares the contents of the Y register with another memory held value and sets the zero and carry flags as appropriate.

       Processor Status after use:

       C	Carry Flag	Set if Y >= M
       Z	Zero Flag	Set if Y = M
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Set if bit 7 of the result is set

       CPY (ComPare Y register)
       Affects Flags: N Z C

       MODE           SYNTAX       HEX LEN TIM
       Immediate     CPY #$44      $C0  2   2
       Zero Page     CPY $44       $C4  2   3
       Absolute      CPY $4400     $CC  3   4

       Operation and flag results are identical to equivalent mode accumulator CMP ops.
    */
    fn CPY(&mut self) {
        // This instruction compares the contents of the Y register with another memory held value
        // Y - M
        let temp: u8 = self.y_reg - self.data;

        // C	Carry Flag	Set if Y >= M
        if self.y_reg >= self.data {
            self.status.set_carry();
        }

        // Z	Zero Flag	Set if Y = M
        if temp == 0 {
            self.status.set_zero();
        }

        // N	Negative Flag	Set if bit 7 of the result is set
        if temp & 0b1000_0000 > 0 {
            self.status.set_negative();
        }
    }

    /**
    *  DEX - Decrement X Register
       X,Z,N = X-1

       Subtracts one from the X register setting the zero and negative flags as appropriate.

       Processor Status after use:

       C	Carry Flag	Not affected
       Z	Zero Flag	Set if X is zero
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Set if bit 7 of X is set

       MODE           SYNTAX       HEX LEN TIM
       Implied        DEX          $CA  1   2
    */
    fn DEX(&mut self) {
        // Subtracts one from the X register
        self.x_reg -= 1;

        // setting the zero and negative flags as appropriate.
        // Z	Zero Flag	Set if X is zero
        if self.x_reg == 0x00 {
            self.status.set_zero();
        }

        // N	Negative Flag	Set if bit 7 of X is set
        if self.x_reg & 0b1000_0000 > 0 {
            self.status.set_negative();
        }
    }

    /**
    *  DEY - Decrement Y Register
       Y,Z,N = Y-1

       Subtracts one from the Y register setting the zero and negative flags as appropriate.

       Processor Status after use:

       C	Carry Flag	Not affected
       Z	Zero Flag	Set if Y is zero
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Set if bit 7 of Y is set

       MODE           SYNTAX       HEX LEN TIM
       Implied         DEY         $88  1   2
    */
    fn DEY(&mut self) {
        // Subtracts one from the Y register
        // Y = Y - 1
        self.y_reg -= 1;

        // setting the zero and negative flags as appropriate.
        // Z	Zero Flag	Set if Y is zero
        if self.y_reg == 0x00 {
            self.status.set_zero();
        }

        // N	Negative Flag	Set if bit 7 of Y is set
        if self.y_reg & 0b1000_0000 > 0 {
            self.status.set_negative();
        }
    }

    /**
    *  INC - Increment Memory
       M,Z,N = M+1

       Adds one to the value held at a specified memory location setting the zero and negative flags as appropriate.

       Processor Status after use:

       C	Carry Flag	Not affected
       Z	Zero Flag	Set if result is zero
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Set if bit 7 of the result is set

       Affects Flags: N Z
       MODE           SYNTAX       HEX LEN TIM
       Zero Page     INC $44       $E6  2   5
       Zero Page,X   INC $44,X     $F6  2   6
       Absolute      INC $4400     $EE  3   6
       Absolute,X    INC $4400,X   $FE  3   7
    */
    fn INC(&mut self) {
        // Adds one to the value held at a specified memory location
        // M = M + 1
        let temp: u8 = self.data + 1;

        // Write to mem
        let address: u16 = self.address.clone();
        self.write(&address, temp);

        // Z	Zero Flag	Set if result is zero
        if temp == 0x00 {
            self.status.set_zero();
        }

        // N	Negative Flag	Set if bit 7 of the result is set
        if temp & 0b1000_0000 > 0 {
            self.status.set_negative();
        }
    }

    /**
    *  INX - Increment X Register
       X,Z,N = X+1

       Adds one to the X register setting the zero and negative flags as appropriate.

       Processor Status after use:

       C	Carry Flag	Not affected
       Z	Zero Flag	Set if X is zero
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Set if bit 7 of X is set

       MODE           SYNTAX       HEX LEN TIM
       Implied         INX         $E8  1   2
    */
    fn INX(&mut self) {
        // Adds one to the X register
        // X = X + 1
        self.x_reg += 1;

        // Z	Zero Flag	Set if X is zero
        if self.x_reg == 0x00 {
            self.status.set_zero();
        }

        // N	Negative Flag	Set if bit 7 of X is set
        if self.x_reg & 0b1000_0000 > 0 {
            self.status.set_negative();
        }
    }

    /**
    *  INY - Increment Y Register
       Y,Z,N = Y+1

       Adds one to the Y register setting the zero and negative flags as appropriate.

       Processor Status after use:

       C	Carry Flag	Not affected
       Z	Zero Flag	Set if Y is zero
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Set if bit 7 of Y is set

       MODE           SYNTAX       HEX LEN TIM
       Implied         INX         $C8  1   2
    */
    fn INY(&mut self) {
        // Adds one to the Y register
        // Y = Y + 1
        self.y_reg += 1;

        // setting the zero and negative flags as appropriate.
        // Z	Zero Flag	Set if Y is zero
        if self.y_reg == 0x00 {
            self.status.set_zero();
        }

        // N	Negative Flag	Set if bit 7 of Y is set
        if self.y_reg & 0b1000_0000 > 0 {
            self.status.set_negative();
        }
    }

    /**
    *  STA - Store Accumulator
       M = A

       Stores the contents of the accumulator into memory.

       Processor Status after use:

       C	Carry Flag	Not affected
       Z	Zero Flag	Not affected
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Not affected

       Affects Flags: none

       MODE           SYNTAX       HEX LEN TIM
       Zero Page     STA $44       $85  2   3
       Zero Page,X   STA $44,X     $95  2   4
       Absolute      STA $4400     $8D  3   4
       Absolute,X    STA $4400,X   $9D  3   5
       Absolute,Y    STA $4400,Y   $99  3   5
       Indirect,X    STA ($44,X)   $81  2   6
       Indirect,Y    STA ($44),Y   $91  2   6
    */
    fn STA(&mut self) {
        // Stores the contents of the accumulator into memory.
        // A -> M
        let data: u8 = self.accumulator;
        let address = self.address.clone();
        self.write(&address, data);
    }

    /**
    *  STX - Store X Register
       M = X

       Stores the contents of the X register into memory.

       Processor Status after use:

       C	Carry Flag	Not affected
       Z	Zero Flag	Not affected
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Not affected

       Affects Flags: none

       MODE           SYNTAX       HEX LEN TIM
       Zero Page     STX $44       $86  2   3
       Zero Page,Y   STX $44,Y     $96  2   4
       Absolute      STX $4400     $8E  3   4
    */
    fn STX(&mut self) {
        // Stores the contents of the X register into memory.
        // X -> M
        let data: u8 = self.x_reg;
        let address = self.address.clone();
        self.write(&address, data);
    }

    /**
    *  STY - Store Y Register
       M = Y

       Stores the contents of the Y register into memory.

       Processor Status after use:

       C	Carry Flag	Not affected
       Z	Zero Flag	Not affected
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Not affected

       Affects Flags: none

       MODE           SYNTAX       HEX LEN TIM
       Zero Page     STY $44       $84  2   3
       Zero Page,X   STY $44,X     $94  2   4
       Absolute      STY $4400     $8C  3   4
    */
    fn STY(&mut self) {
        // Stores the contents of the Y register into memory.
        // Y -> M
        let data: u8 = self.y_reg;
        let address = self.address.clone();
        self.write(&address, data);
    }

    /**
    *  TAX - Transfer Accumulator to X
       X = A

       Copies the current contents of the accumulator into the X register and sets the zero and negative flags as appropriate.

       Processor Status after use:

       C	Carry Flag	Not affected
       Z	Zero Flag	Set if X = 0
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Set if bit 7 of X is set

       MODE           SYNTAX       HEX LEN TIM
       Implied         TAX         $AA  1   2
    */
    fn TAX(&mut self) {
        //  Copies the current contents of the accumulator into the X register
        // A -> X
        self.x_reg = self.accumulator.clone();

        // sets the zero and negative flags as appropriate.
        // Z	Zero Flag	Set if X = 0
        if self.x_reg == 0x00 {
            self.status.set_zero();
        }

        // N	Negative Flag	Set if bit 7 of X is set
        if self.x_reg & 0b1000_0000 > 0 {
            self.status.set_negative();
        }
    }

    /**
    *  TAY - Transfer Accumulator to Y
       Y = A

       Copies the current contents of the accumulator into the Y register and sets the zero and negative flags as appropriate.

       Processor Status after use:

       C	Carry Flag	Not affected
       Z	Zero Flag	Set if Y = 0
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Set if bit 7 of Y is set

       MODE           SYNTAX       HEX LEN TIM
       Implied         TAY         $A8  1   2
    */
    fn TAY(&mut self) {
        // Copies the current contents of the accumulator into the Y register
        // A -> Y
        self.y_reg = self.accumulator.clone();

        // sets the zero and negative flags as appropriate.
        // Z	Zero Flag	Set if Y = 0
        if self.y_reg == 0x00 {
            self.status.set_zero();
        }

        // N	Negative Flag	Set if bit 7 of Y is set
        if self.y_reg & 0b1000_0000 > 0 {
            self.status.set_negative();
        }
    }

    /**
    *  TSX - Transfer Stack Pointer to X
       X = S

       Copies the current contents of the stack register into the X register and sets the zero and negative flags as appropriate.

       Processor Status after use:

       C	Carry Flag	Not affected
       Z	Zero Flag	Set if X = 0
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Set if bit 7 of X is set

       MODE           SYNTAX       HEX LEN TIM
       Implied         TSX         $BA  1   2
    */
    fn TSX(&mut self) {
        // Copies the current contents of the stack register into the X register
        // X = S
        self.x_reg = self.stack_pointer.clone();

        // sets the zero and negative flags as appropriate.
        // Z	Zero Flag	Set if X = 0
        if self.x_reg == 0x00 {
            self.status.set_zero();
        }

        // N	Negative Flag	Set if bit 7 of X is set
        if self.x_reg & 0b1000_0000 > 0 {
            self.status.set_negative();
        }
    }

    /**
    *  TXA - Transfer X to Accumulator
       A = X

       Copies the current contents of the X register into the accumulator and sets the zero and negative flags as appropriate.

       Processor Status after use:

       C	Carry Flag	Not affected
       Z	Zero Flag	Set if A = 0
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Set if bit 7 of A is set

       MODE           SYNTAX       HEX LEN TIM
       Implied         TXA         $8A  1   2
    */
    fn TXA(&mut self) {
        // Copies the current contents of the X register into the accumulator
        // A = X
        self.accumulator = self.x_reg.clone();

        // sets the zero and negative flags as appropriate.
        // Z	Zero Flag	Set if A = 0
        if self.accumulator == 0x00 {
            self.status.set_zero();
        }

        // N	Negative Flag	Set if bit 7 of A is set
        if self.accumulator & 0b1000_0000 > 0 {
            self.status.set_negative();
        }
    }

    /**
    *  TXS - Transfer X to Stack Pointer
       S = X

       Copies the current contents of the X register into the stack register.

       Processor Status after use:

       C	Carry Flag	Not affected
       Z	Zero Flag	Not affected
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Not affected

       MODE           SYNTAX       HEX LEN TIM
       Implied         TXS         $9A  1   2
    */
    fn TXS(&mut self) {
        // Copies the current contents of the X register into the stack register.
        self.x_reg = self.stack_pointer.clone();
    }

    /**
    *  TYA - Transfer Y to Accumulator
       A = Y

       Copies the current contents of the Y register into the accumulator and sets the zero and negative flags as appropriate.

       Processor Status after use:

       C	Carry Flag	Not affected
       Z	Zero Flag	Set if A = 0
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Set if bit 7 of A is set

       MODE           SYNTAX       HEX LEN TIM
       Implied         TYA         $98  1   2
    */
    fn TYA(&mut self) {
        // Copies the current contents of the Y register into the accumulator
        // A = Y
        self.accumulator = self.y_reg.clone();

        // sets the zero and negative flags as appropriate.
        // Z	Zero Flag	Set if A = 0
        if self.accumulator == 0x00 {
            self.status.set_zero();
        }

        // N	Negative Flag	Set if bit 7 of A is set
        if self.accumulator & 0b1000_0000 > 0 {
            self.status.set_negative();
        }
    }
}

/**
* Constrol Flow Instructions
*  Branch Instructions
   Affect Flags: none

   All branches are relative mode and have a length of two bytes.
   Syntax is "Bxx Displacement" or (better) "Bxx Label".
   See the notes on the Program Counter for more on displacements.

   Branches are dependant on the status of the flag bits when the op code is encountered.
   A branch not taken requires two machine cycles.
   Add one if the branch is taken and add one more if the branch crosses a page boundary.
*/
impl CPU {
    /**
    *  JSR (Jump to SubRoutine)
       The JSR instruction pushes the address (minus one) of the return point on to the stack 
       and then sets the program counter to the target memory address.

       Processor Status after use:
       
       C	Carry Flag	Not affected
       Z	Zero Flag	Not affected
       I	Interrupt Disable	Not affected
       D	Decimal Mode Flag	Not affected
       B	Break Command	Not affected
       V	Overflow Flag	Not affected
       N	Negative Flag	Not affected

       Affects Flags: none

       MODE           SYNTAX       HEX LEN TIM
       Absolute      JSR $5597     $20  3   6

       JSR pushes the address-1 of the next operation on to
       the stack before transferring program control to the following address.
       Subroutines are normally terminated by a RTS op code.
    */
    fn JSR(&mut self) {
        // Fetch PC for access OP Code and store at address
        // Example :
        //          PC = 0xD43D
        //          Address = 0xD43D
        self.address = self.pc.clone();

        // Next PC
        // PC = PC + 1
        // Example :
        //          PC = 0xD43D + 0x0001
        //          PC = 0xD43E
        self.pc += 1;

        // Fetch PC for write to stack Hi/Lo
        // Example :
        //          PC = 0xD43E
        //          Address Register = 0xD43E
        self.address_register = self.pc.clone();

        // Push PC Hi byte (8-bit) to stack by stack pointer (SP)
        // Example :
        //          PC(Hi) = 0xD4
        //          STCK_ADDR_HI = 0x01B7
        let pch: u8 = (self.address_register >> 8 & 0x00FF) as u8;
        let stck_addr_hi: u16 = (*START_STACK_POINTER + self.stack_pointer as u16 & 0x00FF).into();
        self.write(&stck_addr_hi, pch);

        // Push PC Lo byte (8-bit) to stack by stack pointer - 1 (SP)
        // Eample :
        //          PC(Lo) = 0x3E
        //          STCK_ADDR_LO = 0x01B6
        self.stack_pointer -= 1;
        let pcl: u8 = (self.address_register & 0x00FF) as u8;
        let stck_addr_lo: u16 = (*START_STACK_POINTER + self.stack_pointer as u16 & 0x00FF).into();
        self.write(&stck_addr_lo, pcl);

        // Clear address register
        // Address Register = 0x0000
        self.address_register = 0x0000;

        // Read Rom memory by address
        // Example :
        //          Address = 0xD43D
        //          Data = 0xFB
        self.data = self.read_rom(&self.address);

        // Add data to Lo byte of address register
        // Example :
        //          Address Register = 0x0000 + 0xFB
        //                           = 0x00FB
        self.address_register += self.data as u16 & 0x00FF;

        // Store PC to address
        // Example :
        //          PC = 0xD43E
        //          Address = 0xD43E
        self.address = self.pc.clone();

        // Read Rom memory by address
        // Example :
        //          Address = 0xD43E
        //          Data = 0xDA
        self.data = self.read_rom(&self.address);

        // Add data to Hi byte (8-bit) of address register
        // Example :
        //          Data  = 0xDA
        //          Address Register = 0x00FB + 0xDA << 8
        //                           = 0xDAFB
        let data: u16 = (((self.data as u16) << 8) as u16 & 0xFF00).into();
        self.address_register += data;

        // Store back to PC
        // Example :
        //          PC = 0xD43E
        //          Address Register = 0xDAFB
        //          PC = 0xDAFB
        self.pc = self.address_register;

        // Update Cycles (6)
        // Cycle = Cycle - 6
        self.cycles -= 6;

        // Switch state to 'Fetch'
        self.state = State::Fetch;
    }


    /**
     *  JMP - Jump
        Sets the program counter to the address specified by the operand.

        Processor Status after use:

        C	Carry Flag	Not affected
        Z	Zero Flag	Not affected
        I	Interrupt Disable	Not affected
        D	Decimal Mode Flag	Not affected
        B	Break Command	Not affected
        V	Overflow Flag	Not affected
        N	Negative Flag	Not affected

        Affects Flags: none

        MODE           SYNTAX       HEX LEN TIM
        Absolute      JMP $5597     $4C  3   3
        Indirect      JMP ($5597)   $6C  3   5

        JMP transfers program execution to the following address (absolute) or to the location contained in 
        the following address (indirect). Note that there is no carry associated with the indirect jump so:
        AN INDIRECT JUMP MUST NEVER USE A
        VECTOR BEGINNING ON THE LAST BYTE
        OF A PAGE
        For example if address $3000 contains $40, $30FF contains $80, and $3100 contains $50, 
        the result of JMP ($30FF) will be a transfer of control to $4080 rather than $5080 as you intended i.e. 
        the 6502 took the low byte of the address from $30FF and the high byte from $3000.
     */
    fn JMP(&mut self,code : &u8) {
        match *code {
            0x4C => {
                self.JMP_ABS();
            },
            0x6C => {
                self.JMP_IND();
            },
            _ => {
                self.state = State::Fetch;
            }
        }
    }

    fn JMP_ABS(&mut self) {
        // Fetch PC for access OP Code and store at address
        // Example :
        //          PC = 0x0001
        //          Address = 0x0001
        self.address = self.pc.clone();

        // Next PC
        // PC = PC + 1
        // Example :
        //          PC = 0x0001 + 0x0001
        //          PC = 0x0002
        self.pc += 1;

        // Read Rom memory by address
        // Example :
        //          Address = 0x0001
        //          Data = 0x3C
        self.data = self.read_rom(&self.address);

        // Clear address register
        // Address Register = 0x0000
        self.address_register = 0x0000;


        // Add data to Lo byte of address register
        // Example :
        //          Address Register = 0x0000 + 0x3C
        //                           = 0x003C
        self.address_register = self.data as u16 & 0x00FF;

        // Store PC to address
        // Example :
        //          PC = 0x0002
        //          Address = 0x0002
        self.address = self.pc.clone();

        // Next PC
        // PC = PC + 1
        // Example :
        //          PC = 0x0002 + 0x0001
        //          PC = 0x0003
        self.pc += 1;

        // Read Rom memory by address
        // Example :
        //          Address = 0x0002
        //          Data = 0xD4
        self.data = self.read_rom(&self.address);

        // Add data to Hi byte (8-bit) of address register
        // Example :
        //          Data  = 0xD4
        //          Address Register = 0x003C + 0xD4 << 8
        //                           = 0xD43C
        let data: u16 = (((self.data as u16) << 8) as u16 & 0xFF00).into();
        self.address_register += data;

        // Store back to PC
        // Example :
        //          PC = 0x0003
        //          Address Register = 0xD43C
        //          PC = 0xD43C
        self.pc = self.address_register;

        // Update Cycles (3)
        // Cycle = Cycle - 3
        self.cycles -= 3;

        // Switch state to 'Fetch'
        self.state = State::Fetch;

    }

    fn JMP_IND(&mut self) {
        // Fetch PC for access OP Code and store at address
        // Example :
        //          PC = 0xFDEE
        //          Address = 0xFDEE
        self.address = self.pc.clone();

        // Next PC
        // PC = PC + 1
        // Example :
        //          PC = 0xFDEE + 0x0001
        //          PC = 0xFDEF
        self.pc += 1;

        // Read Rom memory by address
        // Example :
        //          Address = 0xFDEE
        //          Data = 0x36
        self.data = self.read_rom(&self.address);

        // Clear address register
        // Address Register = 0x0000
        self.address_register = 0x0000;


        // Add data to Lo byte of address register
        // Example :
        //          Address Register = 0x0000 + 0x36
        //                           = 0x0036
        self.address_register += self.data as u16 & 0x00FF;

        // Store PC to address
        // Example :
        //          PC = 0xFDEF
        //          Address = 0xFDEF
        self.address = self.pc.clone();

        // Next PC
        // PC = PC + 1
        // Example :
        //          PC = 0xFDEF + 0x0001
        //          PC = 0xFDF0
        self.pc += 1;

        // Read Rom memory by address
        // Example :
        //          Address = 0xFDEF
        //          Data = 0x00
        self.data = self.read_rom(&self.address);

        // Add data to Hi byte (8-bit) of address register
        // Example :
        //          Data  = 0x00
        //          Address Register = 0x0036 + 0x00 << 8
        //                           = 0x0036
        let data: u16 = (((self.data as u16) << 8) as u16 & 0xFF00).into();
        self.address_register += data;

        // Store data to address
        // Example :
        //          Data = 0x0036
        //          Address = 0x0036
        self.address = self.address_register.clone();
        // Clear address register
        // Address Register = 0x0000
        self.address_register = 0x0000;


        // Read Rom memory by address
        // Example :
        //          Address = 0x0036
        //          Data = 0xF0
        self.data = self.read_rom(&self.address);

        // Add data to Lo byte of address register
        // Example :
        //          Address Register = 0x0000 + 0xF0
        //                           = 0x00F0
        self.address_register += self.data as u16 & 0x00FF;


        // Next address + 1
        // Address = Address + 1
        // Example :
        //          Address = 0x0036 + 0x0001
        //                  = 0x0037
        self.address += 1;

        // Read Rom memory by address
        // Example :
        //          Address = 0x0037
        //          Data = 0xFD
        self.data = self.read_rom(&self.address);

        // Add data to Hi byte (8-bit) of address register
        // Example :
        //          Data  = 0xFD
        //          Address Register = 0x00F0 + 0xFD << 8
        //                           = 0xFDF0
        let data: u16 = (((self.data as u16) << 8) as u16 & 0xFF00).into();
        self.address_register += data;

        // Store back to PC
        // Example :
        //          PC = 0xFDF0
        //          Address Register = 0xFDF0
        //          PC = 0xFDF0
        self.pc = self.address_register;
        // Clear address register
        // Address Register = 0x0000
        self.address_register = 0x0000;

        // Update Cycles (5)
        // Cycle = Cycle - 5
        self.cycles -= 5;

        // Switch state to 'Fetch'
        self.state = State::Fetch;
    }
}


/**
 * Arithmetrix & Logic Instructions 
 */
impl CPU {
    /**
     *  CMP - Compare
        Z,C,N = A-M

        This instruction compares the contents of the accumulator with 
        another memory held value and sets the zero and carry flags as appropriate.

        Processor Status after use:

        C	Carry Flag	Set if A >= M
        Z	Zero Flag	Set if A = M
        I	Interrupt Disable	Not affected
        D	Decimal Mode Flag	Not affected
        B	Break Command	Not affected
        V	Overflow Flag	Not affected
        N	Negative Flag	Set if bit 7 of the result is set

        Affects Flags: N Z C

        MODE           SYNTAX       HEX LEN TIM
        Immediate     CMP #$44      $C9  2   2
        Zero Page     CMP $44       $C5  2   3
        Zero Page,X   CMP $44,X     $D5  2   4
        Absolute      CMP $4400     $CD  3   4
        Absolute,X    CMP $4400,X   $DD  3   4+
        Absolute,Y    CMP $4400,Y   $D9  3   4+
        Indirect,X    CMP ($44,X)   $C1  2   6
        Indirect,Y    CMP ($44),Y   $D1  2   5+

        + add 1 cycle if page boundary crossed

        Compare sets flags as if a subtraction had been carried out. 
        If the value in the accumulator is equal or greater than the compared value, 
        the Carry will be set. The equal (Z) and negative (N) flags will be set based on equality or 
        lack thereof and the sign (i.e. A>=$80) of the accumulator.
     */
    fn CMP(&mut self,code : &u8) {
        match *code {
            0xC9 => {
                self.CMP_IMM();
            },
            0xC5 => {},
            0xD5 => {},
            0xCD => {},
            0xDD => {},
            0xD9 => {},
            0xC1 => {},
            0xD1 => {},
            _ => {}
        }
    }

    fn CMP_IMM(&mut self) {
        // Load PC to store add address
        // Example :
        //          PC = 0xDB5F
        //          Address = 0xDB5F
        self.address = self.pc.clone();

        // Next PC 
        // Example :
        //          PC = 0xDB5F + 0x0001
        //             = 0xDB60
        self.pc += 1;

        // Copy load accumulator to temp
        // Example :
        //          Accumulator = 0b1000_1000
        //          temp = 0b11000_1000
        let temp = self.accumulator.clone();

        // Load data from Rom memory
        // Example :
        //          Address = 0xDB5F
        //          Data  = 0xA0
        self.data = self.read_rom(&self.address);

        // A - Data
        // Example :
        //        diff = T - Data
        //             = 0b11000_1000
        //                          -
        //               0b1010_0000
        //             = 0b1110_1000
        let diff = temp - self.data;

        // sets the zero and carry flags as appropriate.
        // C	Carry Flag	Set if A >= M
        // Example :
        //          Accumulator = 0b1000_1000
        //          Data        = 0b1010_0000
        //          Status      = 0b1011_0000
        //          C = 0
        if self.accumulator >= diff {
            self.status.set_carry();
        } else {
            self.status.unset_carry();
        }

        // Z	Zero Flag	Set if A = M
        // Example :
        //           Z = 0
        if self.accumulator == diff {
            self.status.set_zero();
        } else {
            self.status.unset_zero();
        }

        // N	Negative Flag	Set if bit 7 of the result is set
        // Example :
        //          N = 1
        if diff & 0b1000_0000 > 0 {
            self.status.set_negative();
        } else {
            self.status.unset_negative();
        }

        // Update Cycle = Cycle - 2
        self.cycles -= 2;

        // Update state 'Fetch'
        self.state = State::Fetch;
    }
}