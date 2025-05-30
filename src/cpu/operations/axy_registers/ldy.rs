use crate::cpu::model::{State, CPU};

impl CPU {
    /**
    * LDY - Load Index Register Y From Memory
      Operation: M → Y

      Load the index register Y from memory.

      LDY does not affect the C or V flags, sets the N flag if the value loaded in bit 7 is a 1,
      otherwise resets N, sets Z flag if the loaded value is zero otherwise resets Z and only affects the Y register.

      Addressing Mode	        Assembly Language Form	Opcode	No. Bytes	No. Cycles
      Immediate	                    LDY #$nn	     $A0	    2	        2
      Absolute	                        LDY $nnnn	     $AC	    3	        4
      X-Indexed Absolute	            LDY $nnnn,X	     $BC	    3	        4+p
      Zero Page	                    LDY $nn 	     $A4	    2	        3
      X-Indexed Zero Page	            LDY $nn,X	     $B4	    2	        4
      p: =1 if page is crossed.

      Processor Status register changes
      Flag	Effect
      Zero flag	    Set if the specified byte is zero, otherwise cleared.
      Negative flag	Updated to the value of bit #7 of the specified byte.
    */
    pub fn ldy(&mut self, code: &u8) {
        match *code {
            /* Immediate */
            0xA0 => {
                self.ldy_immediate();
                self.ldy_run();
            }
            /* Absolute */
            0xAC => {
                self.ldy_absolute();
                self.ldy_run();
            }
            /* X-Indexed Absolute */
            0xBC => {
                let page_cross = self.ldy_absolute_x();
                self.ldy_run();

                if page_cross {
                    // TODO :: Tick
                }
            }
            /* Zero Page */
            0xA4 => {
                self.ldy_zero_page();
                self.ldy_run();
            }
            /* X-Indexed Zero Page */
            0xB4 => {
                self.ldy_zero_page_x();
                self.ldy_run();
            }
            _ => {
                self.state = State::Fetch;
            }
        }
    }

    fn ldy_immediate(&mut self) {
        // PC + 1
        self.pc += 1;
        self.address = self.pc.clone();
        // Fetch Data
        self.data = self.read(&self.address);
        // PC + 2 : Next Instruction
        self.pc += 1;
    }

    fn ldy_zero_page(&mut self) {
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

    fn ldy_zero_page_x(&mut self) {
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

    fn ldy_absolute(&mut self) {
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

    fn ldy_absolute_x(&mut self) -> bool {
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

    fn ldy_run(&mut self) {
        self.y_register = self.data;

        // Set if the specified byte is zero, otherwise cleared.
        if self.y_register & 0xFF == 0x00 {
            self.status.set_zero();
        } else {
            self.status.unset_zero();
        }

        // Updated to the value of bit #7 of the specified byte.
        if self.y_register & 0x80 > 0 {
            self.status.set_negative();
        } else {
            self.status.unset_negative();
        }
    }
}
