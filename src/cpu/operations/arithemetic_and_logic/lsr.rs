use crate::cpu::model::{State, CPU};

impl CPU {
    /**
    * LSR - Logical Shift Right
       Operation: 0 → /M7...M0/ → C

       This instruction shifts either the accumulator or a specified memory location 1 bit to the right, with the higher bit of the result always being set to 0, and the low bit which is shifted out of the field being stored in the carry flag.

       The shift right instruction either affects the accumulator by shift­ing it right 1 or is a read/modify/write instruction which changes a speci­fied memory location but does not affect any internal registers. The shift right does not affect the overflow flag. The N flag is always reset. The Z flag is set if the result of the shift is 0 and reset otherwise. The carry is set equal to bit 0 of the input.

       Addressing Mode	        Assembly Language Form	Opcode	No. Bytes	No. Cycles
       Accumulator	                    LSR A	          $4A	    1	        2
       Absolute	                    LSR $nnnn	      $4E	    3	        6
       X-Indexed Absolute	            LSR $nnnn,X	      $5E	    3	        7
       Zero Page	                    LSR $nn	          $46	    2	        5
       X-Indexed Zero Page	            LSR $nn,X	      $56	    2	        6

       Processor Status register changes
       Flag	Effect
       Zero flag	Set if the shifted byte is zero, otherwise cleared.
       Negative flag	Always cleared (because bit #7 becomes zero).
       Carry flag	The old value of bit #0 is stored here.
    */
    pub fn lsr(&mut self, code: &u8) {
        match *code {
            /* Accumulator */
            0x4A => {
                self.lsr_accumulator();
                self.lsr_accumulator_run();
            }
            /* Absolute */
            0x4E => {
                self.lsr_absolute();
                self.lsr_run();
            }
            /* X-Indexed Absolute */
            0x5E => {
                let page_cross = self.lsr_absolute_x();
                self.lsr_run();

                if page_cross {
                    // TODO :: Tick
                }
            }
            /* Zero Page */
            0x46 => {
                self.lsr_zero_page();
                self.lsr_run();
            }
            /* X-Indexed Zero Page */
            0x56 => {
                self.lsr_zero_page_x();
                self.lsr_run();
            }
            _ => {
                self.state = State::Fetch;
            }
        }
    }

    fn lsr_accumulator(&mut self) {
        self.data = self.accumulator.clone();
        self.pc += 1;
    }

    fn lsr_absolute(&mut self) {
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

    fn lsr_absolute_x(&mut self) -> bool {
        // PC + 1
        self.pc += 1;
        self.address = self.pc.clone();
        // Fetch low order Effective Address byte
        // Store : BAL
        self.data = self.read(&self.address);
        let bal = self.data;

        // PC + 1
        self.pc += 1;
        self.address = self.pc.clone();
        // Fetch high order Effective Address byte
        // Store : BAH
        self.data = self.read(&self.address);
        let bah = self.data;

        let current_addr = ((bah as u16) << 8) & 0xFF00 | bal as u16 & 0x00FF;
        let new_addr = self.x_register.clone() as u16 & 0x00FF;
        self.address_register = current_addr + new_addr;
        self.address = self.address_register;
        // Fetch Data
        self.data = self.read(&self.address);

        // PC + 3
        self.pc += 1;

        // Calculate Page Cross
        return self.page_cross(current_addr, current_addr + new_addr);
    }

    fn lsr_zero_page(&mut self) {
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

    fn lsr_zero_page_x(&mut self) {
        // PC + 1
        self.pc += 1;
        self.address = self.pc.clone();
        // Fetch Page Zero Base Address
        // BAL
        self.data = self.read(&self.address);
        let bal = self.data;
        self.address_register = 0x0000 + bal as u16 & 0x00FF + self.x_register as u16 & 0x00FF;
        self.address = self.address_register;
        // Fetch Data
        self.data = self.read(&self.address);
        // PC + 2
        self.pc += 1;
    }

    fn lsr_accumulator_run(&mut self) {
        let temp = self.data >> 1;

        // The old value of bit #0 is stored here.
        if self.data & 0x0001 > 0 {
            self.status.set_carry();
        } else {
            self.status.unset_carry();
        }

        // Set if the shifted byte is zero, otherwise cleared.
        if temp & 0x00FF == 0 {
            self.status.set_zero();
        } else {
            self.status.unset_zero();
        }

        // Always cleared (because bit #7 becomes zero).
        if temp & 0x0080 > 0 {
            self.status.set_negative();
        } else {
            self.status.unset_negative();
        }

        self.accumulator = (temp & 0x00FF) as u8;
    }

    fn lsr_run(&mut self) {
        let temp = self.data >> 1;

        // The old value of bit #0 is stored here.
        if self.data & 0x0001 > 0 {
            self.status.set_carry();
        } else {
            self.status.unset_carry();
        }

        // Set if the shifted byte is zero, otherwise cleared.
        if temp & 0x00FF == 0 {
            self.status.set_zero();
        } else {
            self.status.unset_zero();
        }

        // Always cleared (because bit #7 becomes zero).
        if temp & 0x0080 > 0 {
            self.status.set_negative();
        } else {
            self.status.unset_negative();
        }

        let addr = self.address.clone();
        self.write(&addr, (temp & 0x00FF) as u8);
    }
}
