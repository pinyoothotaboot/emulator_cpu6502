use crate::cpu::model::{State, CPU};

impl CPU {
    /**
     * BIT - Test Bits in Memory with Accumulator
        Operation: A ∧ M, M7 → N, M6 → V

        This instruction performs an AND between a memory location and the accumulator but does not store the result of the AND into the accumulator.

        The bit instruction affects the N flag with N being set to the value of bit 7 of the memory being tested, the V flag with V being set equal to bit 6 of the memory being tested and Z being set by the result of the AND operation between the accumulator and the memory if the result is Zero, Z is reset otherwise. It does not affect the accumulator.

        Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
        Absolute	        BIT $nnnn	         $2C	    3	        4
        Zero Page	        BIT $nn	             $24	    2	        3

        Processor Status register changes
        Flag	Effect
        Zero flag	Set if the result of the AND operation is zero (none of the bits tested were set in both bytes), otherwise cleared.
        Overflow flag	Updated to equal bit #6 of the specified memory byte.
        Negative flag	Updated to equal bit #7 of the specified memory byte.
     */
    pub fn bit(&mut self,code : &u8) {
        match *code {
            /* Absolute */
            0x2C => {
                self.bit_absolute();
                self.bit_run();
            },
            /* Zero Page */
            0x24 => {
                self.bit_zero_page();
                self.bit_run();
            },
            _ => {
                self.state = State::Fetch;
            }
        }
    }

    fn bit_zero_page(&mut self) { 
        // PC + 1
        self.pc += 1;
        self.address = self.pc.clone();
        // Fetch Effective Address
        self.data = self.read(&self.address);
        let adl = self.data;
        self.address_register = 0x0000 | adl as u16 & 0x00FF;
        self.address = self.address_register;
        // Fetch Data
        self.data = self.read(&self.address);
        // PC + 2
        self.pc += 1;
    }

    fn bit_absolute(&mut self) {
        // PC + 1
        self.pc += 1;
        self.address = self.pc.clone();
        // Fetch low order Effective Address byte
        // Store : ADL
        self.data = self.read(&self.address);
        let adl = self.data;

        // PC + 1
        self.pc += 1;
        self.address = self.pc.clone();
        // Fetch high order Effective Address byte
        // Store : ADH
        self.data = self.read(&self.address);
        let adh = self.data;

        self.address_register = ((adh as u16) << 8) & 0xFF00 | adl as u16 & 0x00FF;
        self.address = self.address_register;
        // Fetch Data
        self.data = self.read(&self.address);

        // PC + 3
        self.pc += 1;
    }

    fn bit_run(&mut self) {
        let temp = self.data & self.accumulator;

        // Set if the result of the AND operation is zero (none of the bits tested were set in both bytes), otherwise cleared.
        if temp & 0xFF == 0 {
            self.status.set_zero();
        } else {
            self.status.unset_zero();
        }

        // Updated to equal bit #6 of the specified memory byte.
        if temp & 0b0100_0000 > 0 {
            self.status.set_overflow();
        } else {
            self.status.unset_overflow();
        }

        // Updated to equal bit #7 of the specified memory byte.
        if temp & 0x80 > 0 {
            self.status.set_negative();
        } else {
            self.status.unset_negative();
        }
    }
}