use crate::cpu::model::{State, CPU};

impl CPU {
    /**
     * BMI - Branch on Result Minus
        Operation: Branch on N = 1

        This instruction takes the conditional branch if the N bit is set.

        BMI does not affect any of the flags or any other part of the machine other than the program counter and then only if the N bit is on.

        Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
        Relative	        BMI $nnnn	          $30	    2	        2+t+p
        p: =1 if page is crossed.
        t: =1 if branch is taken.
     */
    pub fn bmi(&mut self) {
        self.bmi_relative();
        self.bmi_run();
    }

    fn bmi_relative(&mut self) {
        self.pc +=1;
        self.address = self.pc.clone();
        // Fetch Branch Offset
        self.data = self.read(&self.address);
    }

    fn bmi_run(&mut self) {
        if self.status.get_negative() == 1 {
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