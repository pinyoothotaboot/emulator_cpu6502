use crate::cpu::model::CPU;

impl CPU {
    /**
    * DEY - Decrement Index Register Y By One
      Operation: Y - 1 → Y

      This instruction subtracts one from the current value in the in­ dex register Y and stores
      the result into the index register Y. The result does not affect or consider carry so that the value in
      the index register Y is decremented to 0 and then through 0 to FF.

      Decrement Y does not affect the carry or overflow flags; if the Y register
      contains bit 7 on as a result of the decrement the N flag is set, otherwise the N flag is reset.
      If the Y register is 0 as a result of the decrement, the Z flag is set otherwise the Z flag is reset.
      This instruction only affects the index register Y.

      Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
      Implied	            DEY	                 $88	    1	        2

      Processor Status register changes
      Flag	Effect
      Zero flag	Set if the result is zero, otherwise cleared.
      Negative flag	Updated to the value of bit #7 of the result.
    */
    pub fn dey(&mut self) {
        self.dey_implied();
        self.dey_run();
    }

    fn dey_implied(&mut self) {
        // PC = PC + 1
        self.pc += 1;
    }

    fn dey_run(&mut self) {
        self.y_register -= 1;

        // Set if the result is zero, otherwise cleared.
        if self.y_register & 0xFF == 0x00 {
            self.status.set_zero();
        } else {
            self.status.unset_zero();
        }

        // Updated to the value of bit #7 of the result.
        if self.y_register & 0x80 > 0x00 {
            self.status.set_negative();
        } else {
            self.status.unset_negative();
        }
    }
}
