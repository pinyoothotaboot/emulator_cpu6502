use crate::cpu::model::{State, CPU};

impl CPU {
    /**
     * TXA - Transfer Index X To Accumulator
        Operation: X → A

        This instruction moves the value that is in the index register X to the accumulator A without disturbing the content of the index register X.

        TXA does not affect any register other than the accumula­tor and does not affect the carry or overflow flag. If the result in A has bit 7 on, then the N flag is set, otherwise it is reset. If the resultant value in the accumulator is 0, then the Z flag is set, other­ wise it is reset.

        Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
        Implied	            TXA	                 $8A	    1	        2

        Processor Status register changes
        Flag	Effect
        Zero flag	    Set if the copied byte is zero, otherwise cleared.
        Negative flag	Updated to the value of bit #7 of the copied byte.
     */
    pub fn txa(&mut self) {
        self.txa_implied();
        self.txa_run();
    }

    fn txa_implied(&mut self) {
        // PC = PC + 1
        self.pc += 1;
    }

    fn txa_run(&mut self) {
        self.accumulator = self.x_register.clone();

        // Set if the copied byte is zero, otherwise cleared.
        if self.accumulator & 0xFF == 0x00 {
            self.status.set_zero();
        } else {
            self.status.unset_zero();
        }

        // Updated to the value of bit #7 of the copied byte.
        if self.accumulator & 0x80 > 0 {
            self.status.set_negative();
        } else {
            self.status.unset_negative();
        }
    }
}