use crate::bus::model::Bus;
use crate::cpu::instruction::CPU_6502_OPERATION_CODES_MAP;
use crate::cpu::model::{State, CPU};
use crate::cpu::processor_status::ProcessorStatus;
use crate::interfaces::bus::IBus;
use crate::interfaces::cpu::ICPU;

impl CPU {
    pub fn new() -> Self {
        CPU {
            pc: 0x0000,
            address: 0x0000,
            data: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            accumulator: 0x00,
            instruction_reg: None,
            status: ProcessorStatus::new(),
            address_register: 0x0000,
            cycles: 0x0000,
            state: State::Fetch,
            bus: Bus::new(),
            sp: 0x00,
        }
    }

    fn fetch(&mut self) {
        // Load 16-bit from program counter (PC) and set to address
        // PC -> Address
        // Example :
        //           PC = 0x0000
        //           Address = 0x0000
        self.address = self.pc.clone();

        // Load data from cpu6502 instruction memory 8-bit.And store data
        // Instrunction Memory 8-bit -> Data
        // Example :
        //           Memory = 0x0000 : 4C
        //           Data   = 0x4C
        self.data = self.read(&self.address);
    }

    fn decode(&mut self) {
        // Fetch OP CODE
        self.instruction_reg = CPU_6502_OPERATION_CODES_MAP.get(&self.data).copied();
    }
}

/** R/W Memory */
impl CPU {
    pub fn read(&self, address: &u16) -> u8 {
        return self.bus.read(address);
    }

    pub fn write(&mut self, address: &u16, data: u8) {
        self.bus.write(address, data);
    }
}

impl ICPU for CPU {
    fn reset(&mut self) {}

    fn run(&mut self) {
        loop {
            // Process Instruction With State
            match self.state {
                State::Fetch => {
                    // State fetch
                    self.fetch();
                    self.state = State::Decode;
                }
                State::Decode => {
                    // State Decode
                    self.decode();
                    self.state = State::Execute;
                }
                State::Execute => {
                    // State Execute
                    self.execute();
                    self.state = State::Fetch;
                }
                State::Exit => {
                    break;
                }
                _ => {
                    panic!("Program has problem!.");
                }
            }
        }
    }
}

impl CPU {
    fn execute(&mut self) {
        match self.instruction_reg {
            Some(instruction) => {
                match instruction.code {
                    /* CPX Compare X Register */
                    0xE0 | 0xE4 | 0xEC => {
                        self.cpx(&instruction.code);
                    }
                    /* CPY Compare Y Register */
                    0xC0 | 0xC4 | 0xCC => {
                        self.cpy(&instruction.code);
                    }
                    /* DEX Decrement X Register */
                    0xCA => {
                        self.dex();
                    }
                    /* DEY Decrement Y Register */
                    0x88 => {
                        self.dey();
                    }
                    /* INC - Increment Memory By One */
                    0xE6 | 0xF6 | 0xEE | 0xFE => {
                        self.inc(&instruction.code);
                    }
                    /* INX - Increment Index Register X By One */
                    0xE8 => {
                        self.inx();
                    }
                    /* INY - Increment Index Register Y By One */
                    0xC8 => {
                        self.iny();
                    }
                    /* LDA - Load Accumulator with Memory */
                    0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                        self.lda(&instruction.code);
                    }
                    /* LDX - Load Index Register X From Memory */
                    0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => {
                        self.ldx(&instruction.code);
                    }
                    /* LDY - Load Index Register Y From Memory */
                    0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => {
                        self.ldy(&instruction.code);
                    }
                    /* STA - Store Accumulator in Memory */
                    0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                        self.sta(&instruction.code);
                    }
                    /* STX - Store Index Register X In Memory */
                    0x86 | 0x96 | 0x8E => {
                        self.stx(&instruction.code);
                    }
                    /* STY - Store Index Register Y In Memory */
                    0x84 | 0x94 | 0x8C => {
                        self.sty(&instruction.code);
                    }
                    /* TAX - Transfer Accumulator To Index X */
                    0xAA => {
                        self.tax();
                    }
                    /* TAY - Transfer Accumula Tor To Index Y */
                    0xA8 => {
                        self.tay();
                    }
                    /* TSX - Transfer Stack Pointer To Index X */
                    0xBA => {
                        self.tsx();
                    }
                    /* TXA - Transfer Index X To Accumulator */
                    0x8A => {
                        self.txa();
                    }
                    /* TXS - Transfer Index X To Stack Pointer */
                    0x9A => {
                        self.txs();
                    }
                    /* TYA - Transfer Index Y To Accumulator */
                    0x98 => {
                        self.tya();
                    }
                    /* CLC - Clear Carry Flag */
                    0x18 => {
                        self.clc();
                    }
                    /* CLD - Clear Decimal Mode */
                    0xD8 => {
                        self.cld();
                    }
                    /* CLI - Clear Interrupt Disable */
                    0x58 => {
                        self.cli();
                    }
                    /* CLV - Clear Overflow Flag */
                    0xB8 => {
                        self.clv();
                    }
                    /* SEC - Set Carry Flag */
                    0x38 => {
                        self.sec();
                    }
                    /* SED - Set Decimal Mode */
                    0xF8 => {
                        self.sed();
                    }
                    /* SEI - Set Interrupt Disable */
                    0x78 => {
                        self.sei();
                    }
                    /* PHA - Push Accumulator On Stack */
                    0x48 => {
                        self.pha();
                    }
                    /* PHP - Push Processor Status On Stack */
                    0x08 => {
                        self.php();
                    }
                    /* PLA - Pull Accumulator From Stack */
                    0x68 => {
                        self.pla();
                    }
                    /* PLP - Pull Processor Status From Stack */
                    0x28 => {
                        self.plp();
                    }
                    /* BCC - Branch on Carry Clear */
                    0x90 => {
                        self.bcc();
                    }
                    /* BCS - Branch on Carry Set */
                    0xB0 => {
                        self.bcs();
                    }
                    /* BEQ - Branch on Result Zero */
                    0xF0 => {
                        self.beq();
                    }
                    /* BMI - Branch on Result Minus */
                    0x30 => {
                        self.bmi();
                    }
                    /* BNE - Branch on Result Not Zero */
                    0xD0 => {
                        self.bne();
                    }
                    /* BPL - Branch on Result Plus */
                    0x10 => {
                        self.bpl();
                    }
                    /* BVC - Branch on Overflow Clear */
                    0x50 => {
                        self.bvc();
                    }
                    /* JMP - JMP Indirect */
                    0x4C | 0x6C => {
                        self.jmp(&instruction.code);
                    }
                    /* JSR - Jump To Subroutine */
                    0x20 => {
                        self.jsr();
                    }
                    /* RTS - Return From Subroutine */
                    0x60 => {
                        self.rts();
                    }
                    /* BRK - Break Command */
                    0x00 => {
                        self.brk();
                    }
                    /* RTI - Return From Interrupt */
                    0x40 => {
                        self.rti();
                    }
                    /* ADC - Add Memory to Accumulator with Carry */
                    0x69 | 0x6D | 0x7D | 0x79 | 0x65 | 0x75 | 0x61 | 0x71 => {
                        self.adc(&instruction.code);
                    }
                    /* AND - "AND" Memory with Accumulator */
                    0x29 | 0x2D | 0x3D | 0x39 | 0x25 | 0x35 | 0x21 | 0x31 => {
                        self.and(&instruction.code);
                    }
                    /* ASL - Arithmetic Shift Left */
                    0x0A | 0x0E | 0x1E | 0x06 | 0x16 => {
                        self.asl(&instruction.code);
                    }
                    /* BIT - Test Bits in Memory with Accumulator */
                    0x2C | 0x24 => {
                        self.bit(&instruction.code);
                    }
                    /* CMP - Compare Memory and Accumulator */
                    0xC9 | 0xCD | 0xDD | 0xD9 | 0xC5 | 0xD5 | 0xC1 | 0xD1 => {
                        self.cmp(&instruction.code);
                    }
                    /* DEC - Decrement Memory By One */
                    0xCE | 0xDE | 0xC6 | 0xD6 => {
                        self.dec(&instruction.code);
                    }
                    /* EOR - "Exclusive OR" Memory with Accumulator */
                    0x49 | 0x4D | 0x5D | 0x59 | 0x45 | 0x55 | 0x41 | 0x51 => {
                        self.eor(&instruction.code);
                    }
                    /* LSR - Logical Shift Right */
                    0x4A | 0x4E | 0x5E | 0x46 | 0x56 => {
                        self.lsr(&instruction.code);
                    }
                    /* ORA - "OR" Memory with Accumulator */
                    0x09 | 0x0D | 0x1D | 0x19 | 0x05 | 0x15 | 0x01 | 0x11 => {
                        self.ora(&instruction.code);
                    }
                    /* ROL - Rotate Left */
                    0x2A | 0x2E | 0x3E | 0x26 | 0x36 => {
                        self.rol(&instruction.code);
                    }
                    /* ROR - Rotate Right */
                    0x6A | 0x6E | 0x7E | 0x66 | 0x76 => {
                        self.ror(&instruction.code);
                    }
                    /* SBC - Subtract Memory from Accumulator with Borrow */
                    0xE9 | 0xED | 0xFD | 0xF9 | 0xE5 | 0xF5 | 0xE1 | 0xF1 => {
                        self.sbc(&instruction.code);
                    }
                    /* BVS - Branch on Overflow Set */
                    0x70 => {
                        self.bvs();
                    }
                    _ => {
                        self.state = State::Fetch;
                    }
                }
            }
            None => {
                panic!("Not found instruction");
            }
        }
    }
}

/** Calculate */
impl CPU {
    pub fn page_cross(&mut self, old_addr: u16, new_addr: u16) -> bool {
        if old_addr != new_addr {
            return true;
        }
        return false;
    }
}
