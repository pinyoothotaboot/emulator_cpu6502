use crate::cpu::model::{State, CPU};

impl CPU {
    /**
     * BVC - Branch on Overflow Clear
        Operation: Branch on V = 0

        This instruction tests the status of the V flag and takes the conditional branch if the flag is not set.

        BVC does not affect any of the flags and registers other than the program counter and only when the overflow flag is reset.

        Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
        Relative	        BVC $nnnn	         $50	    2	        2+t+p
        p: =1 if page is crossed.
        t: =1 if branch is taken.
     */
    pub fn bvc(&mut self) {
        self.bvc_relative();
        self.bvc_run();
    }

    fn bvc_relative(&mut self) {
        self.pc +=1;
        self.address = self.pc.clone();
        // Fetch Branch Offset
        self.data = self.read(&self.address);
    }

    fn bvc_run(&mut self) {
        if self.status.get_overflow() == 0 {
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