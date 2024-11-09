use crate::cpu::model::{State, CPU};

impl CPU {
    /**
     * CMP - Compare Memory and Accumulator
        Operation: A - M

        This instruction subtracts the contents of memory from the contents of the accumulator.

        The use of the CMP affects the following flags: Z flag is set on an equal comparison, reset otherwise; the N flag is set or reset by the result bit 7, the carry flag is set when the value in memory is less than or equal to the accumulator, reset when it is greater than the accumulator. The accumulator is not affected.

        Addressing Mode	        Assembly Language Form	Opcode	No. Bytes	No. Cycles
        Immediate	                    CMP #$nn	      $C9	    2	        2
        Absolute	                    CMP $nnnn	      $CD	    3	        4
        X-Indexed Absolute	            CMP $nnnn,X	      $DD	    3	        4+p
        Y-Indexed Absolute	            CMP $nnnn,Y	      $D9	    3	        4+p
        Zero Page	                    CMP $nn	          $C5	    2	        3
        X-Indexed Zero Page	            CMP $nn,X	      $D5	    2	        4
        X-Indexed Zero Page Indirect	CMP ($nn,X)	      $C1	    2	        6
        Zero Page Indirect Y-Indexed	CMP ($nn),Y	      $D1	    2	        5+p
        p: =1 if page is crossed.

        Processor Status register changes
        Flag	Effect
        Carry flag	Set if the value in the Accumulator is greater than or equal to the operand byte, otherwise cleared.
        Zero flag	Set if the value in the Accumulator is equal to the operand byte, otherwise cleared.
        Negative flag	Updated to the value of bit #7 of the result.
     */
    pub fn cmp(&mut self,code : &u8) {
        match *code {
            /* Immediate */
            0xC9 => {
                self.cmp_immediate();
                self.cmp_run();
            },
            /* Absolute */
            0xCD => {
                self.cmp_absolute();
                self.cmp_run();
            },
            /* X-Indexed Absolute */
            0xDD => {
                let page_cross = self.cmp_absolute_x();
                self.cmp_run();

                if page_cross {
                    // TODO :: Tick
                }
            },
            /* Y-Indexed Absolute */
            0xD9 => {
                let page_cross = self.cmp_absolute_y();
                self.cmp_run();

                if page_cross {
                    // TODO :: Tick
                }
            },
            /* Zero Page */
            0xC5 => {
                self.cmp_zero_page();
                self.cmp_run();
            },
            /* X-Indexed Zero Page */
            0xD5 => {
                self.cmp_zero_page_x();
                self.cmp_run();
            },
            /* X-Indexed Zero Page Indirect */
            0xC1 => {
                self.cmp_indirect_x();
                self.cmp_run();
            },
            /* Zero Page Indirect Y-Indexed */
            0xD1 => {
                let page_cross = self.cmp_indirect_y();
                self.cmp_run();

                if page_cross {
                    // TODO :: Tick
                }
            },
            _ => {
                self.state = State::Fetch;
            }
        }
    }

    fn cmp_immediate(&mut self) {
        // PC + 1
        self.pc +=1;
        self.address = self.pc.clone();
        // Fetch Data
        self.data = self.read(&self.address);
        // PC + 2 : Next Instruction
        self.pc +=1;
    }

    fn cmp_absolute(&mut self) {
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

    fn cmp_absolute_x(&mut self) -> bool {
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

    fn cmp_absolute_y(&mut self) -> bool {
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

    fn cmp_zero_page(&mut self) { 
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

    fn cmp_zero_page_x(&mut self) {
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

    fn cmp_indirect_x(&mut self) {
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

    fn cmp_indirect_y(&mut self) -> bool {
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

    fn cmp_run(&mut self) {
        let temp : u8 = self.accumulator - self.data;

        // Set if the value in the Accumulator is greater than or equal to the operand byte, otherwise cleared.
        if self.accumulator >= self.data {
            self.status.set_carry();
        } else {
            self.status.unset_carry();
        }

        // Set if the value in the Accumulator is equal to the operand byte, otherwise cleared.
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
    }
}