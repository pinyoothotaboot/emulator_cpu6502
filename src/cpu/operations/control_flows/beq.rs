use crate::cpu::model::{State, CPU};

impl CPU {
    /**
    * BEQ - Branch on Result Zero
       Operation: Branch on Z = 1

       This instruction could also be called "Branch on Equal."

       It takes a conditional branch whenever the Z flag is on or the previ­ ous result is equal to 0.

       BEQ does not affect any of the flags or registers other than the program counter and only then when the Z flag is set.

       Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
       Relative	        BEQ $nnnn	         $F0	    2	        2+t+p
       p: =1 if page is crossed.
       t: =1 if branch is taken.
    */
    pub fn beq(&mut self) {
        self.beq_relative();
        self.beq_run();
    }

    fn beq_relative(&mut self) {
        self.pc += 1;
        self.address = self.pc.clone();
        // Fetch Branch Offset
        self.data = self.read(&self.address);
    }

    fn beq_run(&mut self) {
        if self.status.get_zero() == 1 {
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
