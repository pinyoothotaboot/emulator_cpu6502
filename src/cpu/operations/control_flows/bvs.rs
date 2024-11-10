use crate::cpu::model::{State, CPU};

impl CPU {
    /**
     * BVS - Branch on Overflow Set
        Operation: Branch on V = 1

        This instruction tests the V flag and takes the conditional branch if V is on.

        BVS does not affect any flags or registers other than the program, counter and only when the overflow flag is set.

        Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
        Relative	        BVS $nnnn	          $70	    2	        2+t+p
        p: =1 if page is crossed.
        t: =1 if branch is taken.

        Processor Status register changes
        Does not update any flags.
     */
    pub fn bvs(&mut self) {
        self.bvs_relative();
        self.bvs_run();
    }

    fn bvs_relative(&mut self) {
        self.pc +=1;
        self.address = self.pc.clone();
        // Fetch Branch Offset
        self.data = self.read(&self.address);
    }

    fn bvs_run(&mut self) {
        if self.status.get_overflow() > 0 {
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