use crate::cpu::model::{State, CPU};

impl CPU {
    /**
    * STX - Store Index Register X In Memory
      Operation: X → M

      Transfers value of X register to addressed memory location.

      No flags or registers in the microprocessor are affected by the store operation.

      Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
      Absolute	            STX $nnnn	         $8E	    3	        4
      Zero Page	        STX $nn	             $86	    2	        3
      Y-Indexed Zero Page	STX $nn,Y	         $96	    2	        4

      Processor Status register changes
       Does not update any flags.
    */
    pub fn stx(&mut self, code: &u8) {
        match *code {
            /* Absolute */
            0x8E => {
                self.stx_absolute();
                self.stx_run();
            }
            /* Zero Page */
            0x86 => {
                self.stx_zero_page();
                self.stx_run();
            }
            /* Y-Indexed Zero Page */
            0x96 => {
                self.stx_zero_page_y();
                self.stx_run();
            }
            _ => {
                self.state = State::Fetch;
            }
        }
    }

    fn stx_zero_page(&mut self) {
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

    fn stx_zero_page_y(&mut self) {
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

    fn stx_absolute(&mut self) {
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

    fn stx_run(&mut self) {
        let addr = self.address.clone();
        self.write(&addr, self.x_register.clone());
    }
}
