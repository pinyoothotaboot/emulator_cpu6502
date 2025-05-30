use crate::cpu::model::{State, CPU};

impl CPU {
    /**
    * CLV - Clear Overflow Flag
       Operation: 0 → V

       This instruction clears the overflow flag to a 0. This com­ mand is used in conjunction with the set overflow pin which can change the state of the overflow flag with an external signal.

       CLV affects no registers in the microprocessor and no flags other than the overflow flag which is set to a 0.

       Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
       Implied	            CLV	                 $B8	    1	        2
    */
    pub fn clv(&mut self) {
        self.clv_implied();
        self.clv_run();
    }

    fn clv_implied(&mut self) {
        self.pc += 1;
    }

    fn clv_run(&mut self) {
        self.status.unset_overflow();
    }
}
