use crate::cpu::model::{State, CPU};

impl CPU {
    /**
    * ROL - Rotate Left
       Operation: C ← /M7...M0/ ← C

       The rotate left instruction shifts either the accumulator or addressed memory left 1 bit, with the input carry being stored in bit 0 and with the input bit 7 being stored in the carry flags.

       The ROL instruction either shifts the accumulator left 1 bit and stores the carry in accumulator bit 0 or does not affect the internal reg­isters at all. The ROL instruction sets carry equal to the input bit 7, sets N equal to the input bit 6 , sets the Z flag if the result of the ro­ tate is 0, otherwise it resets Z and does not affect the overflow flag at all.

       Addressing Mode	        Assembly Language Form	Opcode	No. Bytes	No. Cycles
       Accumulator	                ROL A	              $2A	    1	        2
       Absolute	                ROL $nnnn	          $2E	    3	        6
       X-Indexed Absolute	        ROL $nnnn,X	          $3E	    3	        7
       Zero Page	                ROL $nn	              $26	    2	        5
       X-Indexed Zero Page     	ROL $nn,X	          $36	    2	        6

       Processor Status register changes
       Flag	Effect
       Zero flag	Set if the rotated byte is zero, otherwise cleared.
       Negative flag	Set to the value of the new bit #7 (which was bit #6 in the original byte).
       Carry flag	The old value of bit #7 is stored here.
    */
    pub fn rol(&mut self, code: &u8) {
        match *code {
            /* Accumulator */
            0x2A => {
                self.rol_accumulator();
                self.rol_accumulator_run();
            }
            /* Absolute */
            0x2E => {
                self.rol_absolute();
                self.rol_run();
            }
            /* X-Indexed Absolute */
            0x3E => {
                let page_cross = self.rol_absolute_x();
                self.rol_run();

                if page_cross {
                    // TODO :: Tick
                }
            }
            /* Zero Page */
            0x26 => {
                self.rol_zero_page();
                self.rol_run();
            }
            /* X-Indexed Zero Page */
            0x36 => {
                self.rol_zero_page_x();
                self.rol_run();
            }
            _ => {
                self.state = State::Fetch;
            }
        }
    }

    fn rol_absolute(&mut self) {
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

    fn rol_absolute_x(&mut self) -> bool {
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

    fn rol_zero_page(&mut self) {
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

    fn rol_zero_page_x(&mut self) {
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

    fn rol_accumulator(&mut self) {
        self.data = self.accumulator.clone();
        self.pc += 1;
    }

    fn rol_accumulator_run(&mut self) {
        let temp = (self.data << 1) as u16 | self.status.get_carry() as u16;

        // The old value of bit #7 is stored here.
        if temp & 0xFF00 > 0 {
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

        // Set to the value of the new bit #7 (which was bit #6 in the original byte).
        if temp & 0x0080 > 0 {
            self.status.set_negative();
        } else {
            self.status.unset_negative();
        }

        self.accumulator = (temp & 0x00FF) as u8;
    }

    fn rol_run(&mut self) {
        let temp = (self.data << 1) as u16 | self.status.get_carry() as u16;

        // The old value of bit #7 is stored here.
        if temp & 0xFF00 > 0 {
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

        // Set to the value of the new bit #7 (which was bit #6 in the original byte).
        if temp & 0x0080 > 0 {
            self.status.set_negative();
        } else {
            self.status.unset_negative();
        }

        let addr = self.address.clone();
        self.write(&addr, (temp & 0x00FF) as u8);
    }
}
