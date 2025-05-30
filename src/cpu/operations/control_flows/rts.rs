use crate::cpu::model::{State, CPU};

impl CPU {
    /**
    * RTS - Return From Subroutine
       Operation: PC↑, PC + 1 → PC

       This instruction loads the program count low and program count high from the stack into the program counter and increments the program counter so that it points to the instruction following the JSR. The stack pointer is adjusted by incrementing it twice.

       The RTS instruction does not affect any flags and affects only PCL and PCH.

       Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
       Implied	            RTS	                 $60	    1	        6
    */
    pub fn rts(&mut self) {
        self.rts_implied();
    }

    fn rts_implied(&mut self) {
        // Pull PCL from Stack
        // PCL
        self.sp += 1;
        let addr_stck = 0x0100 | self.sp as u16 & 0x00FF;
        self.data = self.read(&addr_stck);
        let pcl = self.data as u16 & 0x00FF;

        // Pull PCH from Stack
        // PCH
        self.sp += 1;
        let addr_stck = 0x0100 | self.sp as u16 & 0x00FF;
        self.data = self.read(&addr_stck);
        let pch = self.data as u16 & 0x00FF;

        self.address_register = (pch << 8) & 0xFF00 | pcl;
    }

    fn rts_run(&mut self) {
        // Next Instruction
        self.pc = self.address_register + 0x0001;
    }
}
