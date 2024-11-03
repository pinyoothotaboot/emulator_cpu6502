use crate::cpu::model::{State, CPU};

impl CPU {
    /**
     * JSR - Jump To Subroutine
        Operation: PC + 2↓, [PC + 1] → PCL, [PC + 2] → PCH

        This instruction transfers control of the program counter to a subroutine location but leaves a return pointer on the stack to allow the user to return to perform the next instruction in the main program after the subroutine is complete. To accomplish this, JSR instruction stores the program counter address which points to the last byte of the jump instruc­ tion onto the stack using the stack pointer. The stack byte contains the program count high first, followed by program count low. The JSR then transfers the addresses following the jump instruction to the program counter low and the program counter high, thereby directing the program to begin at that new address.

        The JSR instruction affects no flags, causes the stack pointer to be decremented by 2 and substitutes new values into the program counter low and the program counter high.

        Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
        Absolute	        JSR $nnnn	          $20	    3	        6
     */
    pub fn jsr(&mut self) {
        self.jsr_absolute();
        self.jsr_run();
    }

    fn jsr_absolute(&mut self) {
        // Fetch low order byte of Subroutine Address
        self.pc +=1;
        self.address = self.pc.clone();
        self.data = self.read(&self.address);
    }

    fn jsr_run(&mut self) {
        // ADL
        let adl = self.data;
        
        // Push high order byte of program counter to Stack
        // PCH
        let pch = ((self.pc >> 8) & 0x00FF) as u8;
        let addr_stck = 0x0100 + self.sp as u16;
        self.write(&addr_stck,pch);

        // Push low order byte of program counter to Stack
        // PCL
        self.sp -=1;
        let pcl = ( self.pc & 0x00FF ) as u8;
        let addr_stck = 0x0100 + self.sp as u16;
        self.write(&addr_stck,pcl);

        // Fetch high order byte of Subroutine Address
        // ADH
        self.pc +=1;
        self.address = self.pc.clone();
        self.data = self.read(&self.address);
        let adh = self.data;

        // Next Instruction
        self.address_register = ((adh as u16 ) << 8) & 0xFF00 + adl as u16 & 0x00FF;
        self.pc = self.address_register;
    }
}