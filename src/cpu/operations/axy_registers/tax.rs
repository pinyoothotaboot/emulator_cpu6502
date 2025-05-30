use crate::cpu::model::{State, CPU};

impl CPU {
    /**
    * TAX - Transfer Accumulator To Index X
      Operation: A → X

      This instruction takes the value from accumulator A and trans­ fers or loads it into the index register X without disturbing the content of the accumulator A.

      TAX only affects the index register X, does not affect the carry or overflow flags. The N flag is set if the resultant value in the index register X has bit 7 on, otherwise N is reset. The Z bit is set if the content of the register X is 0 as aresult of theopera­ tion, otherwise it is reset.

      Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
      Implied	            TAX	                  $AA	    1	        2

      Processor Status register changes
       Flag	Effect
       Zero flag	    Set if the copied byte is zero, otherwise cleared.
       Negative flag	Updated to the value of bit #7 of the copied byte.
    */
    pub fn tax(&mut self) {
        self.tax_implied();
        self.tax_run();
    }

    fn tax_implied(&mut self) {
        // PC = PC + 1
        self.pc += 1;
    }

    fn tax_run(&mut self) {
        self.x_register = self.accumulator.clone();

        // Set if the copied byte is zero, otherwise cleared.
        if self.x_register & 0xFF == 0x00 {
            self.status.set_zero();
        } else {
            self.status.unset_zero();
        }

        // Updated to the value of bit #7 of the copied byte.
        if self.x_register & 0x80 > 0 {
            self.status.set_negative();
        } else {
            self.status.unset_negative();
        }
    }
}
