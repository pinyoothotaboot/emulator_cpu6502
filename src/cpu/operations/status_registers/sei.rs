use crate::cpu::model::{State, CPU};

impl CPU {
    /**
    * SEI - Set Interrupt Disable
       Operation: 1 → I

       This instruction initializes the interrupt disable to a 1. It is used to mask interrupt requests during system reset operations and during interrupt commands.

       It affects no registers in the microprocessor and no flags other than the interrupt disable which is set.

       Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
       Implied	            SEI	                 $78	    1	        2
    */
    pub fn sei(&mut self) {
        self.sei_implied();
        self.sei_run();
    }

    fn sei_implied(&mut self) {
        self.pc += 1;
    }

    fn sei_run(&mut self) {
        self.status.set_interrupt_disable();
    }
}
