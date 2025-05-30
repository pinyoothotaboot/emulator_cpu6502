use crate::cpu::model::{State, CPU};

impl CPU {
    /**
    * PLP - Pull Processor Status From Stack
       Operation: P↑

       This instruction transfers the next value on the stack to the Proces­ sor Status register, thereby changing all of the flags and setting the mode switches to the values from the stack.

       The PLP instruction affects no registers in the processor other than the status register. This instruction could affect all flags in the status register.

       Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
       Implied	            PLP	                 $28	    1	        4

       Processor Status register changes
       Updates all flags since the entire value for this register is popped from the stack.
    */
    pub fn plp(&mut self) {
        self.plp_implied();
        self.plp_run();
    }

    fn plp_implied(&mut self) {
        self.pc += 1;
    }

    fn plp_run(&mut self) {
        self.sp += 1;
        let addr = 0x0100 + self.sp.clone() as u16 & 0x00FF;
        let data = self.read(&addr);
        self.status.set_status(data);
        self.status.set_unused();
    }
}
