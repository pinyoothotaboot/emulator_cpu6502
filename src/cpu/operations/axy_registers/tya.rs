use crate::cpu::model::{State, CPU};

impl CPU {
    /**
     * TYA - Transfer Index Y To Accumulator
        Operation: Y → A

        This instruction moves the value that is in the index register Y to accumulator A without disturbing the content of the register Y.

        TYA does not affect any other register other than the accumula­ tor and does not affect the carry or overflow flag. If the result in the accumulator A has bit 7 on, the N flag is set, otherwise it is reset. If the resultant value in the accumulator A is 0, then the Z flag is set, otherwise it is reset.

        Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
        Implied	            TYA	                 $98	    1	        2

        Processor Status register changes
        Flag	Effect
        Zero flag	Set if the copied byte is zero, otherwise cleared.
        Negative flag	Updated to the value of bit #7 of the copied byte.
     */
    pub fn tya(&mut self) {
        self.tya_implied();
        self.tya_run();
    }

    fn tya_implied(&mut self) {
        // PC = PC + 1
        self.pc += 1;
    }

    fn tya_run(&mut self) {
        self.accumulator = self.y_register.clone();

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