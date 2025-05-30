use crate::cpu::model::{State, CPU};

impl CPU {
    /**
    * LDX - Load Index Register X From Memory
      Operation: M → X

      Load the index register X from memory.

      LDX does not affect the C or V flags; sets Z if the value loaded was zero, otherwise resets it;
      sets N if the value loaded in bit 7 is a 1; otherwise N is reset, and affects only the X register.

      Addressing Mode	        Assembly Language Form	Opcode	No. Bytes	No. Cycles
      Immediate	                LDX #$nn	         $A2	    2	        2
      Absolute	                    LDX $nnnn	         $AE	    3	        4
      Y-Indexed Absolute	        LDX $nnnn,Y	         $BE	    3	        4+p
      Zero Page	                LDX $nn	             $A6	    2	        3
      Y-Indexed Zero Page	        LDX $nn,Y	         $B6	    2	        4
      p: =1 if page is crossed.

      Processor Status register changes
      Flag	Effect
      Zero flag	    Set if the specified byte is zero, otherwise cleared.
      Negative flag	Updated to the value of bit #7 of the specified byte.
    */
    pub fn ldx(&mut self, code: &u8) {
        match *code {
            /* Immediate */
            0xA2 => {
                self.ldx_immediate();
                self.ldx_run();
            }
            /* Absolute */
            0xAE => {
                self.ldx_absolute();
                self.ldx_run();
            }
            /* Y-Indexed Absolute */
            0xBE => {
                let page_cross = self.ldx_absolute_y();
                self.ldx_run();

                if page_cross {
                    // TODO :: Tick
                }
            }
            /* Zero Page */
            0xA6 => {
                self.ldx_zero_page();
                self.ldx_run();
            }
            /* Y-Indexed Zero Page */
            0xB6 => {
                self.ldx_zero_page_y();
                self.ldx_run();
            }
            _ => {
                self.state = State::Fetch;
            }
        }
    }

    fn ldx_immediate(&mut self) {
        // PC + 1
        self.pc += 1;
        self.address = self.pc.clone();
        // Fetch Data
        self.data = self.read(&self.address);
        // PC + 2 : Next Instruction
        self.pc += 1;
    }

    fn ldx_zero_page(&mut self) {
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

    fn ldx_zero_page_y(&mut self) {
        // PC + 1
        self.pc += 1;
        self.address = self.pc.clone();
        // Fetch Page Zero Base Address
        // BAL
        self.data = self.read(&self.address);
        let bal = self.data;
        self.address_register = 0x0000 + bal as u16 & 0x00FF + self.y_register as u16 & 0x00FF;
        self.address = self.address_register;
        // Fetch Data
        self.data = self.read(&self.address);
        // PC + 2
        self.pc += 1;
    }

    fn ldx_absolute(&mut self) {
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

    fn ldx_absolute_y(&mut self) -> bool {
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
        let new_addr = self.y_register.clone() as u16 & 0x00FF;
        self.address_register = current_addr + new_addr;
        self.address = self.address_register;
        // Fetch Data
        self.data = self.read(&self.address);

        // PC + 3
        self.pc += 1;

        // Calculate Page Cross
        return self.page_cross(current_addr, current_addr + new_addr);
    }

    fn ldx_run(&mut self) {
        self.x_register = self.data;

        // Set if the specified byte is zero, otherwise cleared.
        if self.x_register & 0xFF == 0x00 {
            self.status.set_zero();
        } else {
            self.status.unset_zero();
        }

        // Updated to the value of bit #7 of the specified byte.
        if self.x_register & 0x80 > 0 {
            self.status.set_negative();
        } else {
            self.status.unset_negative();
        }
    }
}
