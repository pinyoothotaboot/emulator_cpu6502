use crate::cpu::model::CPU;

impl CPU {
    /**
    *  INX - Increment Index Register X By One
       Operation: X + 1 → X

       Increment X adds 1 to the current value of the X register. This is an 8-bit increment which does
       not affect the carry operation, therefore, if the value of X before the increment was FF,
       the resulting value is 00.

       INX does not affect the carry or overflow flags; it sets the N flag if the result of the increment has a one in bit 7,
       otherwise resets N; sets the Z flag if the result of the increment is 0, otherwise it resets the Z flag.

       INX does not affect any other register other than the X register.

       Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
       Implied	            INX	                  $E8	    1	        2

       Processor Status register changes
       Flag	Effect
       Zero flag	Set if the result is zero, otherwise cleared.
       Negative flag	Updated to the value of bit #7 of the result.
    */
    pub fn inx(&mut self) {
        self.inx_implied();
        self.inx_run();
    }

    fn inx_implied(&mut self) {
        // PC + 1
        self.pc += 1;
    }

    fn inx_run(&mut self) {
        self.x_register += 1;

        // Set if the result is zero, otherwise cleared.
        if self.x_register & 0xFF == 0x00 {
            self.status.set_zero();
        } else {
            self.status.unset_zero();
        }

        // Updated to the value of bit #7 of the result.
        if self.x_register & 0x80 > 0 {
            self.status.set_negative();
        } else {
            self.status.unset_negative();
        }
    }
}
