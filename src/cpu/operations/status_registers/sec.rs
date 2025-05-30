use crate::cpu::model::{State, CPU};

impl CPU {
    /**
    * SEC - Set Carry Flag
       Operation: 1 → C

       This instruction initializes the carry flag to a 1. This op eration should normally precede a SBC loop. It is also useful when used with a ROL instruction to initialize a bit in memory to a 1.

       This instruction affects no registers in the microprocessor and no flags other than the carry flag which is set.

       Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
       Implied	            SEC	                  $38	    1	        2
    */
    pub fn sec(&mut self) {
        self.sec_implied();
        self.sec_run();
    }

    fn sec_implied(&mut self) {
        self.pc += 1;
    }

    fn sec_run(&mut self) {
        self.status.set_carry();
    }
}
