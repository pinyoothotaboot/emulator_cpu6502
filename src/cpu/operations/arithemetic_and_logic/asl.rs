use crate::cpu::model::{State, CPU};

impl CPU {
    /**
     * ASL - Arithmetic Shift Left
        Operation: C ← /M7...M0/ ← 0

        The shift left instruction shifts either the accumulator or the address memory location 1 bit to the left, with the bit 0 always being set to 0 and the input bit 7 being stored in the carry flag. ASL either shifts the accumulator left 1 bit or is a read/modify/write instruction that affects only memory.

        The instruction does not affect the overflow bit, sets N equal to the result bit 7 (bit 6 in the input), sets Z flag if the result is equal to 0, otherwise resets Z and stores the input bit 7 in the carry flag.

        Addressing Mode	        Assembly Language Form	Opcode	No. Bytes	No. Cycles
        Accumulator	                ASL A	              $0A	    1	        2
        Absolute	                ASL $nnnn	          $0E	    3	        6
        X-Indexed Absolute	        ASL $nnnn,X	          $1E	    3	        7
        Zero Page	                ASL $nn 	          $06	    2	        5
        X-Indexed Zero Page	        ASL $nn,X	          $16	    2	        6

        Processor Status register changes
        Flag	Effect
        Zero flag	Set if the shifted byte is zero, otherwise cleared.
        Negative flag	Set to the value of bit #7 in the shifted byte.
        Carry flag	The old value of bit #7 is stored here.
     */

    pub fn asl(&mut self,code : &u8) {
        match *code {
            /* Accumulator */
            0x0A => {
                self.asl_accumulator();
                self.asl_accumulator_run();
            },
            /* Absolute */
            0x0E => {
                self.asl_absolute();
                self.asl_run();
            },
            /* X-Indexed Absolute */
            0x1E => {
                let page_cross = self.asl_absolute_x();
                self.asl_run();

                if page_cross {
                    // TODO :: Tick
                }
            },
            /* Zero Page */
            0x06 => {
                self.asl_zero_page();
                self.asl_run();
            },
            /* X-Indexed Zero Page */
            0x16 => {
                self.asl_zero_page_x();
                self.asl_run();
            },
            _ => {
                self.state = State::Fetch;
            }
        }
    }

    fn asl_accumulator(&mut self) {
        self.data = self.accumulator.clone();
        self.pc +=1;
    }

    fn asl_accumulator_run(&mut self) {
        let temp = (self.data as u16) << 1;
        // Set if the shifted byte is zero, otherwise cleared.
        if temp & 0x00FF == 0 {
            self.status.set_zero();
        } else {
            self.status.unset_zero();
        }

        // Set to the value of bit #7 in the shifted byte.
        if temp & 0x80 > 0 {
            self.status.set_negative();
        } else {
            self.status.unset_negative();
        }

        // The old value of bit #7 is stored here.
        if temp & 0xFF00 > 0 {
            self.status.set_carry();
        } else {
            self.status.unset_carry();
        }

        self.accumulator = (temp & 0x00FF) as u8;
    }

    fn asl_zero_page(&mut self) { 
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

    fn asl_absolute(&mut self) {
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

    fn asl_absolute_x(&mut self) -> bool {
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

    fn asl_zero_page_x(&mut self) {
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

    fn asl_run(&mut self) {
        let temp = (self.data as u16) << 1;
        // Set if the shifted byte is zero, otherwise cleared.
        if temp & 0x00FF == 0 {
            self.status.set_zero();
        } else {
            self.status.unset_zero();
        }

        // Set to the value of bit #7 in the shifted byte.
        if temp & 0x80 > 0 {
            self.status.set_negative();
        } else {
            self.status.unset_negative();
        }

        // The old value of bit #7 is stored here.
        if temp & 0xFF00 > 0 {
            self.status.set_carry();
        } else {
            self.status.unset_carry();
        }

        let addr = self.address.clone();
        self.write(&addr,(temp & 0x00FF) as u8);
    }
}