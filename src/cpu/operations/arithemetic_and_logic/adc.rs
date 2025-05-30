use crate::cpu::model::{State, CPU};

impl CPU {
    /**
    * ADC - Add Memory to Accumulator with Carry
       Operation: A + M + C → A, C

       This instruction adds the value of memory and carry from the previous operation to the value of the accumulator and stores the result in the accumulator.

       This instruction affects the accumulator; sets the carry flag when the sum of a binary add exceeds 255 or when the sum of a decimal add exceeds 99, otherwise carry is reset. The overflow flag is set when the sign or bit 7 is changed due to the result exceeding +127 or -128, otherwise overflow is reset. The negative flag is set if the accumulator result contains bit 7 on, otherwise the negative flag is reset. The zero flag is set if the accumulator result is 0, otherwise the zero flag is reset.

       Note on the MOS 6502:

       In decimal mode, the N, V and Z flags are not consistent with the decimal result.

       Addressing Mode	            Assembly Language Form	Opcode	No. Bytes	No. Cycles
       Immediate	                    ADC #$nn	          $69	    2	        2
       Absolute	                    ADC $nnnn	          $6D	    3	        4
       X-Indexed Absolute	            ADC $nnnn,X	          $7D	    3	        4+p
       Y-Indexed Absolute	            ADC $nnnn,Y	          $79	    3	        4+p
       Zero Page	                    ADC $nn	              $65	    2	        3
       X-Indexed Zero Page	            ADC $nn,X	          $75	    2	        4
       X-Indexed Zero Page Indirect	ADC ($nn,X)	          $61	    2	        6
       Zero Page Indirect Y-Indexed	ADC ($nn),Y	          $71	    2	        5+p
       p: =1 if page is crossed.

       Processor Status register changes
       Flag	Effect
       Carry flag	Set if the result includes a carry bit, otherwise cleared.
       Overflow flag	Set if bit #7 of the result changed in a way that indicates overflow when adding signed byte values, otherwise cleared.
       Zero flag	Set if the result is zero, otherwise cleared.
       Negative flag	Updated to the value of bit #7 of the result.
    */
    pub fn adc(&mut self, code: &u8) {
        match *code {
            /* Immediate */
            0x69 => {
                self.adc_immediate();
                self.adc_run();
            }
            /* Absolute */
            0x6D => {
                self.adc_absolute();
                self.adc_run();
            }
            /* X-Indexed Absolute */
            0x7D => {
                let page_cross = self.adc_absolute_x();
                self.adc_run();

                if page_cross {
                    // TODO :: Tick
                }
            }
            /* Y-Indexed Absolute */
            0x79 => {
                let page_cross = self.adc_absolute_y();
                self.adc_run();

                if page_cross {
                    // TODO :: Tick
                }
            }
            /* Zero Page */
            0x65 => {
                self.adc_zero_page();
                self.adc_run();
            }
            /* X-Indexed Zero Page */
            0x75 => {
                self.adc_zero_page_x();
                self.adc_run();
            }
            /* X-Indexed Zero Page Indirect */
            0x61 => {
                self.adc_indirect_x();
                self.adc_run();
            }
            /* Zero Page Indirect Y-Indexed */
            0x71 => {
                let page_cross = self.adc_indirect_y();
                self.adc_run();

                if page_cross {
                    // TODO :: Tick
                }
            }
            _ => {
                self.state = State::Fetch;
            }
        }
    }

    fn adc_immediate(&mut self) {
        // PC + 1
        self.pc += 1;
        self.address = self.pc.clone();
        // Fetch Data
        self.data = self.read(&self.address);
        // PC + 2 : Next Instruction
        self.pc += 1;
    }

    fn adc_zero_page(&mut self) {
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

    fn adc_absolute(&mut self) {
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

    fn adc_absolute_x(&mut self) -> bool {
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

    fn adc_absolute_y(&mut self) -> bool {
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

    fn adc_zero_page_x(&mut self) {
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

    fn adc_indirect_x(&mut self) {
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

    fn adc_indirect_y(&mut self) -> bool {
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

    fn adc_run(&mut self) {
        let temp: u8 = self.accumulator + self.data + self.status.get_carry();

        // Set if the result includes a carry bit, otherwise cleared.
        if temp as u16 > 255 {
            self.status.set_carry();
        } else {
            self.status.unset_carry();
        }

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

        // Set if bit #7 of the result changed in a way that indicates overflow when adding signed byte values, otherwise cleared.
        // http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
        if (self.data ^ temp) & (self.accumulator ^ temp) & 0x80 != 0 {
            self.status.set_overflow();
        } else {
            self.status.unset_overflow();
        }

        self.accumulator = temp & 0x00FF;
    }
}
