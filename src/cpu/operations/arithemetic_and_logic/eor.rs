use crate::cpu::model::{State, CPU};

impl CPU {
    /**
     * EOR - "Exclusive OR" Memory with Accumulator
        Operation: A ⊻ M → A

        The EOR instruction transfers the memory and the accumulator to the adder which performs a binary "EXCLUSIVE OR" on a bit-by-bit basis and stores the result in the accumulator.

        This instruction affects the accumulator; sets the zero flag if the result in the accumulator is 0, otherwise resets the zero flag sets the negative flag if the result in the accumulator has bit 7 on, otherwise resets the negative flag.

        Addressing Mode	        Assembly Language Form	Opcode	No. Bytes	No. Cycles
        Immediate	                    EOR #$nn	      $49	    2	        2
        Absolute	                    EOR $nnnn	      $4D	    3	        4
        X-Indexed Absolute	            EOR $nnnn,X	      $5D	    3	        4+p
        Y-Indexed Absolute	            EOR $nnnn,Y	      $59	    3	        4+p
        Zero Page	                    EOR $nn	          $45	    2	        3
        X-Indexed Zero Page	            EOR $nn,X	      $55	    2	        4
        X-Indexed Zero Page Indirect	EOR ($nn,X)	      $41	    2	        6
        Zero Page Indirect Y-Indexed	EOR ($nn),Y	      $51	    2	        5+p
        p: =1 if page is crossed.

        Processor Status register changes
        Flag	Effect
        Zero flag	Set if the result is zero, otherwise cleared.
        Negative flag	Updated to the value of bit #7 of the result.
     */
    pub fn eor(&mut self,code : &u8) {
        match *code {
            /* Immediate */
            0x49 => {
                self.eor_immediate();
                self.eor_run();
            },
            /* Absolute */
            0x4D => {
                self.eor_absolute();
                self.eor_run();
            },
            /* X-Indexed Absolute */
            0x5D => {
                let page_cross = self.eor_absolute_x();
                self.eor_run();

                if page_cross {
                    // TODO :: Tick
                }
            },
            /* Y-Indexed Absolute */
            0x59 => {
                let page_cross = self.eor_absolute_y();
                self.eor_run();

                if page_cross {
                    // TODO :: Tick
                }
            },
            /* Zero Page */
            0x45 => {
                self.eor_zero_page();
                self.eor_run();
            },
            /* X-Indexed Zero Page	 */
            0x55 => {
                self.eor_zero_page_x();
                self.eor_run();
            },
            /* X-Indexed Zero Page Indirect */
            0x41 => {
                self.eor_indirect_x();
                self.eor_run();
            },
            /* Zero Page Indirect Y-Indexed */
            0x51 => {
                let page_cross = self.eor_indirect_y();
                self.eor_run();

                if page_cross {
                    // TODO :: Tick
                }
            },
            _ => {
                self.state = State::Fetch;
            }
        }
    }

    fn eor_immediate(&mut self) {
        // PC + 1
        self.pc +=1;
        self.address = self.pc.clone();
        // Fetch Data
        self.data = self.read(&self.address);
        // PC + 2 : Next Instruction
        self.pc +=1;
    }

    fn eor_absolute(&mut self) {
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

    fn eor_absolute_x(&mut self) -> bool {
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

    fn eor_absolute_y(&mut self) -> bool {
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

    fn eor_zero_page(&mut self) { 
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

    fn eor_zero_page_x(&mut self) {
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

    fn eor_indirect_x(&mut self) {
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
        self.address_register = ((adh as u16) << 8) & 0xFF00 | (adl as u16) & 0x00FF;
        self.address = self.address_register;
        self.data = self.read(&self.address);

        // PC + 2
        self.pc += 1;
    }

    fn eor_indirect_y(&mut self) -> bool {
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

        let current_addr = ((bah as u16) << 8) & 0xFF00 | bal as u16 & 0x00FF;
        let new_addr = self.y_register.clone() as u16 & 0x00FF;

        // Fetch Data
        self.address_register = current_addr + new_addr;
        self.address = self.address_register;
        self.data = self.read(&self.address);

        // PC + 2
        self.pc += 1;

        return self.page_cross(current_addr, current_addr + new_addr);
    }

    fn eor_run(&mut self) {
        self.accumulator ^= self.data;

        // Set if the result is zero, otherwise cleared.
        if self.accumulator & 0x00FF == 0 {
            self.status.set_zero();
        } else {
            self.status.unset_zero();
        }

        // Updated to the value of bit #7 of the result.
        if self.accumulator & 0x80 > 0 {
            self.status.set_negative();
        } else {
            self.status.unset_negative();
        }
    }
}