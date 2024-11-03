use crate::cpu::model::{State, CPU};

impl CPU {
    /**
     * JMP - JMP Indirect
        Operation: [PC + 1] → PCL, [PC + 2] → PCH

        This instruction establishes a new valne for the program counter.

        It affects only the program counter in the microprocessor and affects no flags in the status register.

        Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
        Absolute	                JMP $nnnn	 $4C	    3	        3
        Absolute Indirect	        JMP ($nnnn)	 $6C	    3	        5
     */
    pub fn jmp(&mut self,code : &u8) {
        match *code {
            /* Absolute */
            0x4C => {
                self.jmp_absolute();
                self.jmp_run();
            },
            /* Absolute Indirect */
            0x6C => {
                self.jmp_jmp_absolute_indirect();
                self.jmp_run();
            },
            _ => {
                self.state = State::Fetch;
            }
        }
    }

    fn jmp_absolute(&mut self) {
        // Fetch low order byte of Jump Address
        // ADL
        self.pc +=1;
        self.address = self.pc.clone();
        self.data = self.read(&self.address);
        let adl = self.data;

        // Fetch hiht order byte of Jump Address
        // ADH
        self.pc +=1;
        self.address = self.pc.clone();
        self.data = self.read(&self.address);
        let adh = self.data;

        self.address_register = ((adh as u16) << 8 ) & 0xFF00 | adl as u16 & 0x00FF;
    }

    fn jmp_jmp_absolute_indirect(&mut self) {
        // Fetch low order byte of Indirect Address
        // IAL
        self.pc +=1;
        self.address = self.pc.clone();
        self.data = self.read(&self.address);
        let idl = self.data;

        // Fetch high order byte of Indirect Address
        // IAH
        self.pc +=1;
        self.address = self.pc.clone();
        self.data = self.read(&self.address);
        let idh = self.data;

        // Fetch low order byte of Jump Address
        // ADL
        self.address_register = ((idh as u16) << 8 ) & 0xFF00 | idl as u16 & 0x00FF;
        self.address = self.address_register;
        self.data = self.read(&self.address);
        let adl = self.data;

        // Fetch high order byte of Jump Address
        // ADH
        self.address += 1;
        self.data = self.read(&self.address);
        let adh = self.data;

        self.address_register = ((adh as u16) << 8 ) & 0xFF00 | adl as u16 & 0x00FF;
    }

    fn jmp_run(&mut self) {
        self.pc = self.address_register;
    }
}