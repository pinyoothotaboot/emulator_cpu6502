use crate::cpu::model::{State, CPU};

impl CPU {
    /**
    * INC - Increment Memory By One
      Operation: M + 1 → M

      This instruction adds 1 to the contents of the addressed memory loca­tion.

      The increment memory instruction does not affect any internal registers and does not affect the carry or overflow flags.
      If bit 7 is on as the result of the increment,N is set, otherwise it is reset; if the increment causes the result to become 0,
      the Z flag is set on, otherwise it is reset.

      Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
      Absolute	            INC $nnnn	         $EE	    3	        6
      X-Indexed Absolute	INC $nnnn,X	         $FE	    3	        7
      Zero Page	        INC $nn 	         $E6	    2	        5
      X-Indexed Zero Page	INC $nn,X	         $F6	    2	        6

      Processor Status register changes
      Flag	Effect
      Zero flag	    Set if the result is zero, otherwise cleared.
      Negative flag	Updated to the value of bit #7 of the result.
    */
    pub fn inc(&mut self, code: &u8) {
        match *code {
            /* Absolute */
            0xEE => {
                self.inc_absolute();
                self.inc_run();
            }
            /* X-Indexed Absolute */
            0xFE => {
                let page_cross = self.inc_absolute_x();
                self.inc_run();

                if page_cross {
                    // TODO :: Tick
                }
            }
            /* Zero Page */
            0xE6 => {
                self.inc_zero_page();
                self.inc_run();
            }
            /* X-Indexed Zero Page */
            0xF6 => {
                self.inc_zero_page_x();
                self.inc_run();
            }
            _ => {
                self.state = State::Fetch;
            }
        }
    }

    fn inc_zero_page(&mut self) {
        // PC + 1
        // ADL : Fetch page zero Effective Address
        self.pc += 1;
        self.address = self.pc.clone();
        self.data = self.read(&self.address);
        let adl = self.data;

        self.address_register = 0x0000 + (adl & 0x00FF) as u16;
        self.address = self.address_register;

        // Fetch Data
        self.data = self.read(&self.address);

        // Next Instruction
        self.pc += 1;
    }

    fn inc_zero_page_x(&mut self) {
        // PC + 1
        // BAL : Fetch page zero Base Address
        self.pc += 1;
        self.address = self.pc.clone();
        self.data = self.read(&self.address);
        let bal = self.data;

        self.address_register = 0x0000 + (bal & 0x00FF) as u16 + (self.x_register & 0x00FF) as u16;
        self.address = self.address_register;

        // Fetch Data
        self.data = self.read(&self.address);

        // Next Instruction
        self.pc += 1;
    }

    fn inc_absolute(&mut self) {
        // PC + 1
        // ADL : Fetch low order byte of Effective Address
        self.pc += 1;
        self.address = self.pc.clone();
        self.data = self.read(&self.address);
        let adl = self.data;

        // PC + 2
        // ADH : Fetch hi order byte of Effective Address
        self.pc += 1;
        self.address = self.pc.clone();
        self.data = self.read(&self.address);
        let adh = self.data;

        self.address_register = ((adh as u16) << 8) & 0xFF00 + (adl as u16) & 0x00FF;
        self.address = self.address_register;

        // Fetch Data
        self.data = self.read(&self.address);

        // PC + 3
        self.pc += 1;
    }

    fn inc_absolute_x(&mut self) -> bool {
        // PC + 1
        // BAL : Fetch low order byte of base address
        self.pc += 1;
        self.address = self.pc.clone();
        self.data = self.read(&self.address);
        let bal = self.data;

        // PC + 2
        // BAH : Fetch hi order byte of base address
        self.pc += 1;
        self.address = self.pc.clone();
        self.data = self.read(&self.address);
        let bah = self.data;

        let current_addr = ((bah as u16) << 8) & 0xFF00 + (bal as u16) & 0x00FF;
        let x_addr = (self.x_register.clone() & 0x00FF) as u16;

        self.address_register = current_addr + x_addr;
        self.address = self.address_register;

        // Fetch Data
        self.data = self.read(&self.address);

        // PC + 3
        self.pc += 1;

        // Calculate Page cross
        return self.page_cross(current_addr, current_addr + x_addr);
    }

    fn inc_run(&mut self) {
        let temp = self.data + 1;

        // Set if the result is zero, otherwise cleared.
        if temp & 0xFF == 0x00 {
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

        // Write modified Data back to memory
        let addr = self.address;
        self.write(&addr, temp & 0x00FF);
    }
}
