use crate::cpu::model::{State, CPU};

impl CPU {
    /**
    * LDA - Load Accumulator with Memory
      Operation: M → A

      When instruction LDA is executed by the microprocessor, data is transferred from memory to the accumulator and stored in the accumulator.

      LDA affects the contents of the accumulator, does not affect the carry or overflow flags;
      sets the zero flag if the accumulator is zero as a result of the LDA, otherwise resets the zero flag;
      sets the negative flag if bit 7 of the accumulator is a 1, other­ wise resets the negative flag.

      Addressing Mode	        Assembly Language Form	Opcode	No. Bytes	No. Cycles
      Immediate	                    LDA #$nn	     $A9	    2	        2
      Absolute	                        LDA $nnnn	     $AD	    3	        4
      X-Indexed Absolute	            LDA $nnnn,X	     $BD	    3	        4+p
      Y-Indexed Absolute	            LDA $nnnn,Y	     $B9	    3	        4+p
      Zero Page	                    LDA $nn	         $A5	    2	        3
      X-Indexed Zero Page	            LDA $nn,X	     $B5	    2	        4
      X-Indexed Zero Page Indirect	    LDA ($nn,X)	     $A1	    2	        6
      Zero Page Indirect Y-Indexed	    LDA ($nn),Y	     $B1	    2	        5+p
      p: =1 if page is crossed.

      Processor Status register changes
      Flag	Effect
      Zero flag	    Set if the specified byte is zero, otherwise cleared.
      Negative flag	Updated to the value of bit #7 of the specified byte.
    */
    pub fn lda(&mut self, code: &u8) {
        match *code {
            /* Immediate */
            0xA9 => {
                self.lda_immediate();
                self.lda_run();
            }
            /* Absolute */
            0xAD => {
                self.lda_absolute();
                self.lda_run();
            }
            /* X-Indexed Absolute */
            0xBD => {
                let page_cross = self.lda_absolute_x();
                self.lda_run();

                if page_cross {
                    // TODO :: Tick
                }
            }
            /* Y-Indexed Absolute */
            0xB9 => {
                let page_cross = self.lda_absolute_y();
                self.lda_run();

                if page_cross {
                    // TODO :: Tick
                }
            }
            /* Zero Page */
            0xA5 => {
                self.lda_zero_page();
                self.lda_run();
            }
            /* X-Indexed Zero Page */
            0xB5 => {
                self.lda_zero_page_x();
                self.lda_run();
            }
            /* X-Indexed Zero Page Indirect */
            0xA1 => {
                self.lda_indirect_x();
                self.lda_run();
            }
            /* Zero Page Indirect Y-Indexed */
            0xB1 => {
                let page_cross = self.lda_indirect_y();
                self.lda_run();

                if page_cross {
                    // TODO :: Tick
                }
            }
            _ => {
                self.state = State::Fetch;
            }
        }
    }

    fn lda_immediate(&mut self) {
        // PC + 1
        self.pc += 1;
        self.address = self.pc.clone();
        // Fetch Data
        self.data = self.read(&self.address);
        // PC + 2
        self.pc += 1;
    }

    fn lda_zero_page(&mut self) {
        // PC + 1
        self.pc += 1;
        self.address = self.pc.clone();
        // Fetch Data and store ADL
        self.data = self.read(&self.address);
        let adl = self.data;
        self.address_register = 0x0000 + adl as u16 & 0x00FF;
        self.address = self.address_register;
        // Fetch Data
        self.data = self.read(&self.address);
        // PC + 2
        self.pc += 1;
    }

    fn lda_zero_page_x(&mut self) {
        // PC + 1
        self.pc += 1;
        self.address = self.pc.clone();
        // Fetch Data and store ADL
        self.data = self.read(&self.address);
        let adl = self.data;
        self.address_register = 0x0000 + adl as u16 & 0x00FF + self.x_register as u16 & 0x00FF;
        self.address = self.address_register;
        // Fetch Data
        self.data = self.read(&self.address);
        // PC + 2
        self.pc += 1;
    }

    fn lda_absolute(&mut self) {
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

        self.address_register = ((adh as u16) << 8) & 0xFF00 + adl as u16 & 0x00FF;
        self.address = self.address_register;
        // Fetch Data
        self.data = self.read(&self.address);

        // PC + 3
        self.pc += 1;
    }

    fn lda_absolute_x(&mut self) -> bool {
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

        let current_addr = ((bah as u16) << 8) & 0xFF00 + bal as u16 & 0x00FF;
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

    fn lda_absolute_y(&mut self) -> bool {
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

        let current_addr = ((bah as u16) << 8) & 0xFF00 + bal as u16 & 0x00FF;
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

    fn lda_indirect_x(&mut self) {
        // PC + 1
        self.pc += 1;
        self.address = self.pc.clone();
        // Fetch Page Zero Base Address
        // Store : BAL
        self.data = self.read(&self.address);
        let bal = self.data;

        // Fetch low order byte of Effective Address
        self.address_register = 0x0000 + bal as u16 & 0x00FF + self.x_register as u16 & 0x00FF;
        self.address = self.address_register;
        self.data = self.read(&self.address);
        let adl = self.data;

        // Fetch hi order byte of Effective Address
        self.address += 1;
        self.data = self.read(&self.address);
        let adh = self.data;

        // Fetch Data
        self.address_register = ((adh as u16) << 8) & 0xFF00 + (adl as u16) & 0x00FF;
        self.address = self.address_register;
        self.data = self.read(&self.address);

        // PC + 2
        self.pc += 1;
    }

    fn lda_indirect_y(&mut self) -> bool {
        // PC + 1
        self.pc += 1;
        self.address = self.pc.clone();
        // Fetch Page Zero Indirect Address
        // Store : IAL
        self.data = self.read(&self.address);
        let ial = self.data;

        // Fetch low order byte of Base Address
        // Store : BAL
        self.address_register = 0x0000 + ial as u16 & 0x00FF;
        self.address = self.address_register;
        self.data = self.read(&self.address);
        let bal = self.data;

        // Fetch hi order byte of Base Address
        // Store : BAH
        self.address += 1;
        self.data = self.read(&self.address);
        let bah = self.data;

        let current_addr = ((bah as u16) << 8) & 0xFF00 + bal as u16 & 0x00FF;
        let new_addr = self.y_register.clone() as u16 & 0x00FF;

        // Fetch Data
        self.address_register = current_addr + new_addr;
        self.address = self.address_register;
        self.data = self.read(&self.address);

        // PC + 2
        self.pc += 1;

        return self.page_cross(current_addr, current_addr + new_addr);
    }

    fn lda_run(&mut self) {
        self.accumulator = self.data;

        // Set if the specified byte is zero, otherwise cleared.
        if self.accumulator & 0xFF == 0 {
            self.status.set_zero();
        } else {
            self.status.unset_zero();
        }

        // Updated to the value of bit #7 of the specified byte.
        if self.accumulator & 0x80 > 0 {
            self.status.set_negative();
        } else {
            self.status.unset_negative();
        }
    }
}
