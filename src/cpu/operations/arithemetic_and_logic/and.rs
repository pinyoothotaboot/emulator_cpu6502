use crate::cpu::model::{State, CPU};

impl CPU {
    /**
    * AND - "AND" Memory with Accumulator
       Operation: A ∧ M → A

       The AND instruction transfer the accumulator and memory to the adder which performs a bit-by-bit AND operation and stores the result back in the accumulator.

       This instruction affects the accumulator; sets the zero flag if the result in the accumulator is 0, otherwise resets the zero flag; sets the negative flag if the result in the accumulator has bit 7 on, otherwise resets the negative flag.

       Addressing Mode	            Assembly Language Form	Opcode	No. Bytes	No. Cycles
       Immediate	                    AND #$nn	          $29	    2	        2
       Absolute	                    AND $nnnn	          $2D	    3	        4
       X-Indexed Absolute	            AND $nnnn,X	          $3D	    3	        4+p
       Y-Indexed Absolute	            AND $nnnn,Y	          $39	    3	        4+p
       Zero Page	                    AND $nn	              $25	    2	        3
       X-Indexed Zero Page            	AND $nn,X	          $35	    2	        4
       X-Indexed Zero Page Indirect	AND ($nn,X)	          $21	    2	        6
       Zero Page Indirect Y-Indexed	AND ($nn),Y	          $31	    2	        5+p
       p: =1 if page is crossed.

       Processor Status register changes
       Flag	Effect
       Zero flag	Set if the result is zero, otherwise cleared.
       Negative flag	Updated to the value of bit #7 of the result.
    */
    pub fn and(&mut self, code: &u8) {
        match *code {
            /* Immediate */
            0x29 => {
                self.and_immediate();
                self.and_run();
            }
            /* Absolute */
            0x2D => {
                self.and_absolute();
                self.and_run();
            }
            /* X-Indexed Absolute */
            0x3D => {
                let page_cross = self.and_absolute_x();
                self.and_run();

                if page_cross {
                    // TODO :: Tick
                }
            }
            /* Y-Indexed Absolute */
            0x39 => {
                let page_cross = self.and_absolute_y();
                self.and_run();

                if page_cross {
                    // TODO :: Tick
                }
            }
            /* Zero Page */
            0x25 => {
                self.and_zero_page();
                self.and_run();
            }
            /* X-Indexed Zero Page */
            0x35 => {
                self.and_zero_page_x();
                self.and_run();
            }
            /* X-Indexed Zero Page Indirect */
            0x21 => {
                self.and_indirect_x();
                self.and_run();
            }
            /* Zero Page Indirect Y-Indexed */
            0x31 => {
                let page_cross = self.and_indirect_y();
                self.and_run();

                if page_cross {
                    // TODO :: Tick
                }
            }
            _ => {
                self.state = State::Fetch;
            }
        }
    }

    fn and_immediate(&mut self) {
        // PC + 1
        self.pc += 1;
        self.address = self.pc.clone();
        // Fetch Data
        self.data = self.read(&self.address);
        // PC + 2 : Next Instruction
        self.pc += 1;
    }

    fn and_absolute(&mut self) {
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

    fn and_absolute_x(&mut self) -> bool {
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

    fn and_absolute_y(&mut self) -> bool {
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

    fn and_zero_page(&mut self) {
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

    fn and_zero_page_x(&mut self) {
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

    fn and_indirect_x(&mut self) {
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

    fn and_indirect_y(&mut self) -> bool {
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

    fn and_run(&mut self) {
        self.accumulator &= self.data;

        // Set if the result is zero, otherwise cleared.
        if self.accumulator & 0xFF == 0 {
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
