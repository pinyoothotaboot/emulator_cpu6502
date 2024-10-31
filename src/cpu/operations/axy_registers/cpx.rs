use crate::cpu::model::{State, CPU};

impl CPU {
    /**
    *  CPX - Compare Index Register X To Memory
       Operation: X - M

       This instruction subtracts the value of the addressed memory location
       from the content of index register X using the adder but does not store the result; therefore,
       its only use is to set the N, Z and C flags to allow for comparison between the index register X and
       the value in memory.

       The CPX instruction does not affect any register in the machine; it also does not affect the overflow flag.
       It causes the carry to be set on if the absolute value of the index register X is equal
       to or greater than the data from memory. If the value of the memory is greater than the content of
       the index register X, carry is reset. If the results of the subtraction contain a bit 7,
       then the N flag is set, if not, it is reset. If the value in memory is equal to the value in index register X,
       the Z flag is set, otherwise it is reset.

       Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
       Immediate	        CPX #$nn	          $E0	    2	        2
       Absolute	            CPX $nnnn	          $EC	    3	        4
       Zero Page	        CPX $nn	              $E4	    2	        3

       Processor Status register changes
       Flag	Effect
       Carry flag	Set if the value in the X register is greater than or equal to the operand byte, otherwise cleared.
       Zero flag	Set if the value in the X register is equal to the operand byte, otherwise cleared.
       Negative flag Updated to the value of bit #7 of the result.
    */
    pub fn cpx(&mut self, code: &u8) {
        match *code {
            /* Immediate */
            0xE0 => {
                self.cpx_immediate();
                self.cpx_run();
            }
            /* Zero Page */
            0xE4 => {
                self.cpx_zero_page();
                self.cpx_run();
            }
            /* Absolute */
            0xEC => {
                self.cpx_absolute();
                self.cpx_run();
            }
            _ => {
                self.state = State::Fetch;
            }
        }
    }

    fn cpx_immediate(&mut self) {
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

    fn cpx_zero_page(&mut self) {
        // PC = PC + 1
        self.pc += 1;

        // Load PC to Address 2-Byte
        self.address = self.pc.clone();

        // Fetch effective address
        // And store : ADL
        self.data = self.read(&self.address);
        let adl = self.data; // Lo-Byte
        self.address_register = 0x0000 + (adl as u16) & 0x00FF;

        self.address = self.address_register;

        // Fetch Data
        self.data = self.read(&self.address);

        // Next PC for next instruction
        // PC = PC + 1
        self.pc += 1;
    }

    fn cpx_absolute(&mut self) {
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
        self.address_register = (adh << 8) as u16 & 0xFF00 + (adl as u16) & 0x00FF;
        self.address = self.address_register;
        self.data = self.read(&self.address);

        // Next PC for next instruction
        // PC = PC + 1
        self.pc += 1;
    }

    fn cpx_run(&mut self) {
        let temp: u8 = self.x_register - self.data;

        // Set if the value in the X register is greater than or equal to the operand byte, otherwise cleared.
        if self.x_register >= self.data {
            self.status.set_carry();
        } else {
            self.status.unset_carry();
        }

        // Set if the value in the X register is equal to the operand byte, otherwise cleared.
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
