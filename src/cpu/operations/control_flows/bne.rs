use crate::cpu::model::{State, CPU};

impl CPU {
    /**
     * BNE - Branch on Result Not Zero
        Operation: Branch on Z = 0

        This instruction could also be called "Branch on Not Equal." It tests the Z flag and takes the conditional branch if the Z flag is not on, indicating that the previous result was not zero.

        BNE does not affect any of the flags or registers other than the program counter and only then if the Z flag is reset.

        Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
        Relative	                BNE $nnnn	$D0	        2	        2+t+p
        p: =1 if page is crossed.
        t: =1 if branch is taken.
     */
    pub fn bne(&mut self) {
        self.bne_relative();
        self.bne_run();
    }

    fn bne_relative(&mut self) {
        self.pc +=1;
        self.address = self.pc.clone();
        // Fetch Branch Offset
        self.data = self.read(&self.address);
    }

    fn bne_run(&mut self) {
        if self.status.get_zero() == 0 {
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