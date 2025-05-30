use crate::cpu::model::{State, CPU};

impl CPU {
    /**
    * TXS - Transfer Index X To Stack Pointer
       Operation: X → S

       This instruction transfers the value in the index register X to the stack pointer.

       TXS changes only the stack pointer, making it equal to the content of the index register X. It does not affect any of the flags.

       Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
       Implied	            TXS	                 $9A	    1	        2

       Processor Status register changes
       Does not update any flags.
    */
    pub fn txs(&mut self) {
        self.txs_implied();
        self.txs_run();
    }

    fn txs_implied(&mut self) {
        // PC = PC + 1
        self.pc += 1;
    }

    fn txs_run(&mut self) {
        self.sp = self.x_register.clone();
    }
}
