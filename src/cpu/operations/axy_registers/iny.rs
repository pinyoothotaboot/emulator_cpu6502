use crate::cpu::model::CPU;

impl CPU {
    /**
    * INY - Increment Index Register Y By One
      Operation: Y + 1 → Y

      Increment Y increments or adds one to the current value in the Y register, storing the result in the Y register.
      As in the case of INX the primary application is to step thru a set of values using the Y register.

      The INY does not affect the carry or overflow flags, sets the N flag if the result of the increment has a one in bit 7,
      otherwise resets N, sets Z if as a result of the increment the Y register is zero otherwise resets the Z flag.

      Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
       Implied	                INY	             $C8	    1	        2

      Processor Status register changes
      Flag	Effect
      Zero flag	    Set if the result is zero, otherwise cleared.
      Negative flag	Updated to the value of bit #7 of the result.
    */
    pub fn iny(&mut self) {
        self.iny_implied();
        self.iny_run();
    }

    fn iny_implied(&mut self) {
        // PC + 1
        self.pc += 1;
    }

    fn iny_run(&mut self) {
        self.y_register += 1;

        // Set if the result is zero, otherwise cleared.
        if self.y_register & 0xFF == 0x00 {
            self.status.set_zero();
        } else {
            self.status.unset_zero();
        }

        // Updated to the value of bit #7 of the result.
        if self.y_register & 0x80 > 0 {
            self.status.set_negative();
        } else {
            self.status.unset_negative();
        }
    }
}
