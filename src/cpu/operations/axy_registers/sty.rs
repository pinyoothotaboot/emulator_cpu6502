use crate::cpu::model::{State, CPU};

impl CPU {
    /**
     * STY - Store Index Register Y In Memory
       Operation: Y → M
       
       Transfer the value of the Y register to the addressed memory location.
       
       STY does not affect any flags or registers in the microprocessor.
       
       Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
       Absolute	            STY $nnnn	          $8C	    3	        4
       Zero Page	        STY $nn	              $84	    2	        3
       X-Indexed Zero Page	STY $nn,X	          $94	    2	        4

       Processor Status register changes
        Does not update any flags.
     */
    pub fn sty(&mut self,code : &u8) {
        match *code {
            /* Absolute */
            0x8C => {
                self.sty_absolute();
                self.sty_run();
            },
            /* Zero Page */
            0x84 => {
                self.sty_zero_page();
                self.sty_run();
            },
            /* X-Indexed Zero Page */
            0x94 => {
                self.sty_zero_page_x();
                self.sty_run();
            },
            _ => {
                self.state = State::Fetch;
            }
        }
    }

    fn sty_zero_page(&mut self) {
        // PC + 1
        self.pc += 1;
        self.address = self.pc.clone();
        // Fetch Effective Address
        self.data = self.read(&self.address);
        let adl = self.data;
        self.address_register = 0x0000 + adl as u16 & 0x00FF;
        self.address = self.address_register;
        // Fetch Data
        self.data = self.read(&self.address);
        // PC + 2
        self.pc += 1;
    }

    fn sty_zero_page_x(&mut self) {
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

    fn sty_absolute(&mut self) {
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

    fn sty_run(&mut self) {
        let addr = self.address.clone();
        self.write(&addr,self.y_register.clone());
    }
}