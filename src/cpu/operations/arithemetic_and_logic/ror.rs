use crate::cpu::model::{State, CPU};

impl CPU {
    /**
     * ROR - Rotate Right
        Operation: C → /M7...M0/ → C

        The rotate right instruction shifts either the accumulator or addressed memory right 1 bit with bit 0 shifted into the carry and carry shifted into bit 7.

        The ROR instruction either shifts the accumulator right 1 bit and stores the carry in accumulator bit 7 or does not affect the internal regis­ ters at all. The ROR instruction sets carry equal to input bit 0, sets N equal to the input carry and sets the Z flag if the result of the rotate is 0; otherwise it resets Z and does not affect the overflow flag at all.

        (Available on Microprocessors after June, 1976)

        Addressing Mode	        Assembly Language Form	Opcode	No. Bytes	No. Cycles
        Accumulator	                    ROR A	          $6A	    1	        2
        Absolute	                    ROR $nnnn	      $6E	    3	        6
        X-Indexed Absolute	            ROR $nnnn,X	      $7E	    3	        7
        Zero Page	                    ROR $nn	          $66	    2	        5
        X-Indexed Zero Page         	ROR $nn,X	      $76	    2	        6

        Processor Status register changes
        Flag	Effect
        Zero flag	Set if the rotated byte is zero, otherwise cleared.
        Negative flag	Set to the previous value of the Carry flag.
        Carry flag	The old value of bit #0 is stored here.
     */
    pub fn ror(&mut self,code : &u8) {
        match *code {
            /* Accumulator */
            0x6A => {
                self.ror_accumulator();
                self.ror_accumulator_run();
            },
            /* Absolute */
            0x6E => {
                self.ror_absolute();
                self.ror_run();
            },
            /* X-Indexed Absolute */
            0x7E => {
                let page_cross = self.ror_absolute_x();
                self.ror_run();

                if page_cross {
                    // TODO :: Tick
                }
            },
            /* Zero Page */
            0x66 => {
                self.ror_zero_page();
                self.ror_run();
            },
            /* X-Indexed Zero Page */
            0x76 => {
                self.ror_zero_page_x();
                self.ror_run();
            },
            _ => {
                self.state = State::Fetch;
            }
        }
    }

    fn ror_absolute(&mut self) {
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

    fn ror_absolute_x(&mut self) -> bool {
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

    fn ror_zero_page(&mut self) { 
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

    fn ror_zero_page_x(&mut self) {
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

    fn ror_accumulator(&mut self) {
        self.data = self.accumulator.clone();
        self.pc +=1;
    }

    fn ror_accumulator_run(&mut self) {
        let temp = ( self.data >> 1 ) as u16 | (self.status.get_carry() << 7) as u16;

        // The old value of bit #0 is stored here.
        if self.data & 0x01 > 0 {
            self.status.set_carry();
        } else {
            self.status.unset_carry();
        }

        // Set if the rotated byte is zero, otherwise cleared.
        if temp & 0x00FF == 0x0000 {
            self.status.set_zero();
        } else {
            self.status.unset_zero();
        }

        // Set to the previous value of the Carry flag.
        if temp & 0x0080 > 0 {
            self.status.set_negative();
        } else {
            self.status.unset_negative();
        }

        self.accumulator = ( temp & 0x00FF ) as u8;
    }

    fn ror_run(&mut self) {
        let temp = ( self.data >> 1 ) as u16 | (self.status.get_carry() << 7) as u16;

        // The old value of bit #0 is stored here.
        if self.data & 0x01 > 0 {
            self.status.set_carry();
        } else {
            self.status.unset_carry();
        }

        // Set if the rotated byte is zero, otherwise cleared.
        if temp & 0x00FF == 0x0000 {
            self.status.set_zero();
        } else {
            self.status.unset_zero();
        }

        // Set to the previous value of the Carry flag.
        if temp & 0x0080 > 0 {
            self.status.set_negative();
        } else {
            self.status.unset_negative();
        }

        let addr = self.address.clone();
        self.write(&addr, ( temp & 0x00FF ) as u8);
    }


}