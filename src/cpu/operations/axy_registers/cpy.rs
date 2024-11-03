use crate::cpu::model::{State, CPU};

impl CPU {
    /**
    *  CPY - Compare Index Register Y To Memory
       Operation: Y - M

       This instruction performs a two's complement subtraction between the index register Y and
       the specified memory location. The results of the subtraction are not stored anywhere.
       The instruction is strict­ly used to set the flags.

       CPY affects no registers in the microprocessor and also does not affect the overflow flag.
       If the value in the index register Y is equal to or greater than the value in the memory,
       the carry flag will be set, otherwise it will be cleared. If the results of the subtract- tion
       contain bit 7 on the N bit will be set, otherwise it will be cleared. If the value in the index register Y and
       the value in the memory are equal, the zero flag will be set, otherwise it will be cleared.

       Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
       Immediate	            CPY #$nn	     $C0	    2	        2
       Absolute	            CPY $nnnn	     $CC	    3	        4
       Zero Page	            CPY $nn	         $C4	    2	        3

       Processor Status register changes
       Flag	Effect
       Carry flag	Set if the value in the Y register is greater than or equal to the operand byte, otherwise cleared.
       Zero flag	Set if the value in the Y register is equal to the operand byte, otherwise cleared.
       Negative flag	Updated to the value of bit #7 of the result.
    */
    pub fn cpy(&mut self, code: &u8) {
        match *code {
            /* Immediate */
            0xC0 => {
                self.cpy_immediate();
                self.cpy_run();
            }
            /* Absolute */
            0xCC => {
                self.cpy_absolute();
                self.cpy_run();
            }
            /* Zero Page */
            0xC4 => {
                self.cpy_zero_page();
                self.cpy_run();
            }
            _ => {
                self.state = State::Fetch;
            }
        }
    }

    fn cpy_immediate(&mut self) {
        // PC = PC + 1
        self.pc += 1;

        // Load PC to Address 2-Byte
        self.address = self.pc.clone();

        // Fetch Data
        self.data = self.read(&self.address);

        // Next PC for next instruction
        // PC = PC + 1
        self.pc += 1;
    }

    fn cpy_absolute(&mut self) {
        // PC = PC + 1
        self.pc += 1;
        // Load PC to Address 2-Byte
        self.address = self.pc.clone();
        // Fetch low order Effective Address byte
        // And store : ADL
        self.data = self.read(&self.address);
        let adl = self.data; // Lo-Byte

        // PC = PC + 1
        self.pc += 1;
        // Load PC to Address 2-Byte
        self.address = self.pc.clone();
        // Fetch hight order Effective Address byte
        // And store : ADH
        self.data = self.read(&self.address);
        let adh = self.data; // Hi-Byte

        // Fetch Data
        self.address_register = ((adh as u16) << 8) & 0xFF00 | (adl as u16) & 0x00FF;
        self.address = self.address_register;
        self.data = self.read(&self.address);

        // Next PC for next instruction
        // PC = PC + 1
        self.pc += 1;
    }

    fn cpy_zero_page(&mut self) {
        // PC = PC + 1
        self.pc += 1;

        // Load PC to Address 2-Byte
        self.address = self.pc.clone();

        // Fetch effective address
        // And store : ADL
        self.data = self.read(&self.address);
        let adl = self.data; // Lo-Byte
        self.address_register = 0x0000 | (adl as u16) & 0x00FF;

        self.address = self.address_register;

        // Fetch Data
        self.data = self.read(&self.address);

        // Next PC for next instruction
        // PC = PC + 1
        self.pc += 1;
    }

    fn cpy_run(&mut self) {
        let temp: u8 = self.y_register - self.data;

        // Set if the value in the y register is greater than or equal to the operand byte, otherwise cleared.
        if self.y_register >= self.data {
            self.status.set_carry();
        } else {
            self.status.unset_carry();
        }

        // Set if the value in the y register is equal to the operand byte, otherwise cleared.
        if temp & 0xFF == 0x00 {
            self.status.set_zero();
        } else {
            self.status.unset_zero();
        }

        // Updated to the value of bit #7 of the result.
        // 0x80 -> 0b1000_0000
        if temp & 0x80 > 0 {
            self.status.set_negative();
        } else {
            self.status.unset_negative();
        }
    }
}
