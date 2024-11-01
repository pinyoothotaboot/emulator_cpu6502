use crate::cpu::model::{State, CPU};

impl CPU {
    /**
     * TAY - Transfer Accumula Tor To Index Y
       Operation: A → Y
       
       This instruction moves the value of the accumulator into index register Y without affecting the accumulator.
       
       TAY instruction only affects the Y register and does not affect either the carry or overflow flags. If the index register Y has bit 7 on, then N is set, otherwise it is reset. If the content of the index register Y equals 0 as a result of the operation, Z is set on, otherwise it is reset.
       
       Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
       Implied	            TAY	                 $A8	    1	        2

       Processor Status register changes
        Flag	Effect
        Zero flag	    Set if the copied byte is zero, otherwise cleared.
        Negative flag	Updated to the value of bit #7 of the copied byte.
     */
    pub fn tay(&mut self) {
        self.tay_implied();
        self.tay_run();
    }

    fn tay_implied(&mut self) {
        // PC = PC + 1
        self.pc += 1;
    }

    fn tay_run(&mut self) {
        self.y_register = self.accumulator.clone();

        // Set if the copied byte is zero, otherwise cleared.
        if self.y_register & 0xFF == 0x00 {
            self.status.set_zero();
        } else {
            self.status.unset_zero();
        }

        // Updated to the value of bit #7 of the copied byte.
        if self.y_register & 0x80 > 0 {
            self.status.set_negative();
        } else {
            self.status.unset_negative();
        }
    }
}