use crate::cpu::model::{State, CPU};

impl CPU {
    /**
    * DEC - Decrement Memory By One
       Operation: M - 1 → M

       This instruction subtracts 1, in two's complement, from the contents of the addressed memory location.

       The decrement instruction does not affect any internal register in the microprocessor. It does not affect the carry or overflow flags. If bit 7 is on as a result of the decrement, then the N flag is set, otherwise it is reset. If the result of the decrement is 0, the Z flag is set, other­wise it is reset.

       Addressing Mode	    Assembly Language Form	Opcode	No. Bytes	No. Cycles
       Absolute	                DEC $nnnn	      $CE	    3	        6
       X-Indexed Absolute	        DEC $nnnn,X	      $DE	    3	        7
       Zero Page	                DEC $nn	          $C6	    2	        5
       X-Indexed Zero Page	        DEC $nn,X	      $D6	    2	        6

       Processor Status register changes
       Flag	Effect
       Zero flag	Set if the result is zero, otherwise cleared.
       Negative flag	Updated to the value of bit #7 of the result.
    */
    pub fn dec(&mut self, code: &u8) {
        match *code {
            /* Absolute */
            0xCE => {
                self.dec_absolute();
                self.dec_run();
            }
            /* X-Indexed Absolute */
            0xDE => {
                let page_cross = self.dec_absolute_x();
                self.dec_run();

                if page_cross {
                    // TODO :: Tick
                }
            }
            /* Zero Page */
            0xC6 => {
                self.dec_zero_page();
                self.dec_run();
            }
            /* X-Indexed Zero Page */
            0xD6 => {
                self.dec_zero_page_x();
                self.dec_run();
            }
            _ => {
                self.state = State::Fetch;
            }
        }
    }

    fn dec_absolute(&mut self) {
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

    fn dec_absolute_x(&mut self) -> bool {
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

    fn dec_zero_page(&mut self) {
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

    fn dec_zero_page_x(&mut self) {
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

    fn dec_run(&mut self) {
        let temp = self.data - 1;
        let addr = self.address.clone();
        self.write(&addr, temp & 0x00FF);

        // Set if the result is zero, otherwise cleared.
        if temp & 0x00FF == 0x00 {
            self.status.set_zero();
        } else {
            self.status.unset_zero();
        }

        // Updated to the value of bit #7 of the result.
        if temp & 0x80 > 0 {
            self.status.set_negative();
        } else {
            self.status.unset_negative();
        }
    }
}
