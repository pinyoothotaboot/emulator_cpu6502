use crate::cpu::model::{State, CPU};

impl CPU {
    /**
    * BCS - Branch on Carry Set
       Operation: Branch on C = 1

       This instruction takes the conditional branch if the carry flag is on.

       BCS does not affect any of the flags or registers except for the program counter and only then if the carry flag is on.

       Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
       Relative	            BCS $nnnn	      $B0	    2	        2+t+p
       p: =1 if page is crossed.
       t: =1 if branch is taken.
    */
    pub fn bcs(&mut self) {
        self.bcs_relative();
        self.bcs_run();
    }

    fn bcs_relative(&mut self) {
        self.pc += 1;
        self.address = self.pc.clone();
        // Fetch Branch Offset
        self.data = self.read(&self.address);
    }

    fn bcs_run(&mut self) {
        if self.status.get_carry() == 1 {
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
