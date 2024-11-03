use crate::cpu::model::{State, CPU};

impl CPU {
    /**
     * BCC - Branch on Carry Clear
        Operation: Branch on C = 0

        This instruction tests the state of the carry bit and takes a conditional branch if the carry bit is reset.

        It affects no flags or registers other than the program counter and then only if the C flag is not on.

        Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
        Relative	            BCC $nnnn	     $90	    2	        2+t+p
        p: =1 if page is crossed.
        t: =1 if branch is taken.
     */
    pub fn bcc(&mut self) {
        self.bcc_relative();
        self.bcc_run();
    }

    fn bcc_relative(&mut self) {
        self.pc +=1;
        self.address = self.pc.clone();
        // Fetch Branch Offset
        self.data = self.read(&self.address);
    }

    fn bcc_run(&mut self) {
        if self.status.get_carry() == 0 {
            let offset = self.data;
            // TODO :: Tick
            self.address_register = self.pc + offset as u16 & 0x00FF + 0x0001;
            self.address = self.address_register;

            if self.address & 0xFF00 != self.pc & 0xFF00 {
                // TODO :: Tick
            }

            // Offset Added to Program Counter
            self.pc = self.address;
        }
    }
}