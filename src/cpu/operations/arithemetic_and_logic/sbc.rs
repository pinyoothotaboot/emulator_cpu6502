use crate::cpu::model::{State, CPU};

impl CPU {
    
    /**
     * SBC - Subtract Memory from Accumulator with Borrow
        Operation: A - M - ~C → A

        This instruction subtracts the value of memory and borrow from the value of the accumulator, using two's complement arithmetic, and stores the result in the accumulator. Borrow is defined as the carry flag complemented; therefore, a resultant carry flag indicates that a borrow has not occurred.

        This instruction affects the accumulator. The carry flag is set if the result is greater than or equal to 0. The carry flag is reset when the result is less than 0, indicating a borrow. The over­flow flag is set when the result exceeds +127 or -127, otherwise it is reset. The negative flag is set if the result in the accumulator has bit 7 on, otherwise it is reset. The Z flag is set if the result in the accumulator is 0, otherwise it is reset.

        Note on the MOS 6502:

        In decimal mode, the N, V and Z flags are not consistent with the decimal result.


        Addressing Mode	            Assembly Language Form	Opcode	No. Bytes	No. Cycles
        Immediate	                    SBC #$nn	         $E9	    2	        2
        Absolute	                    SBC $nnnn	         $ED	    3	        4
        X-Indexed Absolute	            SBC $nnnn,X	         $FD	    3	        4+p
        Y-Indexed Absolute	            SBC $nnnn,Y	         $F9	    3	        4+p
        Zero Page	                    SBC $nn	             $E5	    2	        3
        X-Indexed Zero Page	            SBC $nn,X	         $F5	    2	        4
        X-Indexed Zero Page Indirect	SBC ($nn,X)	         $E1	    2	        6
        Zero Page Indirect Y-Indexed	SBC ($nn),Y	         $F1	    2	        5+p
        p: =1 if page is crossed.

        Processor Status register changes
        Flag	Effect
        Carry flag	Set if borrowing did not occur during the calculation, or cleared if borrowing did occur.
        Overflow flag	Set if bit #7 of the result changed in a way that indicates overflow when subtracting signed byte values, otherwise cleared.
        Zero flag	Set if the result is zero, otherwise cleared.
        Negative flag	Updated to the value of bit #7 of the result.
     */
    pub fn sbc(&mut self,code : &u8) {
        match *code {
            /* Immediate */
            0xE9 => {
                self.sbc_immediate();
                self.sbc_run();
            },
            /* Absolute */
            0xED => {
                self.sbc_absolute();
                self.sbc_run();
            },
            /* X-Indexed Absolute */
            0xFD => {
                let page_cross = self.sbc_absolute_x();
                self.sbc_run();

                if page_cross {
                    // TODO :: Tick
                }
            },
            /* Y-Indexed Absolute */
            0xF9 => {
                let page_cross = self.sbc_absolute_y();
                self.sbc_run();

                if page_cross {
                    // TODO :: Tick
                }
            },
            /* Zero Page */
            0xE5 => {
                self.sbc_zero_page();
                self.sbc_run();
            },
            /* X-Indexed Zero Page	 */
            0xF5 => {
                self.sbc_zero_page_x();
                self.sbc_run();
            },
            /* X-Indexed Zero Page Indirect */
            0xE1 => {
                self.sbc_indirect_x();
                self.sbc_run();
            },
            /* Zero Page Indirect Y-Indexed */
            0xF1 => {
                let page_cross = self.sbc_indirect_y();
                self.sbc_run();

                if page_cross {
                    // TODO :: Tick
                }
            },
            _ => {
                self.state = State::Fetch;
            }
        }
    }

    fn sbc_immediate(&mut self) {
        // PC + 1
        self.pc +=1;
        self.address = self.pc.clone();
        // Fetch Data
        self.data = self.read(&self.address);
        // PC + 2 : Next Instruction
        self.pc +=1;
    }

    fn sbc_absolute(&mut self) {
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

    fn sbc_absolute_x(&mut self) -> bool {
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

    fn sbc_absolute_y(&mut self) -> bool {
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

   fn sbc_zero_page(&mut self) { 
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

    fn sbc_zero_page_x(&mut self) {
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

    fn sbc_indirect_x(&mut self) {
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

    fn sbc_indirect_y(&mut self) -> bool {
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

    fn sbc_run(&mut self) {
        let value = (self.data ^ 0x00FF) as u16;
        let temp = (self.accumulator as u16) & 0x00FF + value + (self.status.get_carry() as u16) & 0x00FF;

        // Set if borrowing did not occur during the calculation, or cleared if borrowing did occur.
        if temp & 0xFF00 > 0 {
            self.status.set_carry();
        } else {
            self.status.unset_carry();
        }

        // Set if the result is zero, otherwise cleared.
        if temp & 0x00FF == 0x0000 {
            self.status.set_zero();
        } else {
            self.status.unset_zero();
        }

        // Set if bit #7 of the result changed in a way that indicates overflow when subtracting signed byte values, otherwise cleared.
        if (value ^ temp) & (self.accumulator as u16 ^ temp) & 0x0080 != 0x0000 {
            self.status.set_overflow();
        } else {
            self.status.unset_overflow();
        }

        // Updated to the value of bit #7 of the result.
        if temp & 0x0080 > 0 {
            self.status.set_negative();
        } else {
            self.status.unset_negative();
        }

        self.accumulator = (temp & 0x00FF) as u8;
    }
}