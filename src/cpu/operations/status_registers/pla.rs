use crate::cpu::model::{State, CPU};

impl CPU {
    /**
     * PLA - Pull Accumulator From Stack
        Operation: A↑

        This instruction adds 1 to the current value of the stack pointer and uses it to address the stack and loads the contents of the stack into the A register.

        The PLA instruction does not affect the carry or overflow flags. It sets N if the bit 7 is on in accumulator A as a result of instructions, otherwise it is reset. If accumulator A is zero as a result of the PLA, then the Z flag is set, otherwise it is reset. The PLA instruction changes content of the accumulator A to the contents of the memory location at stack register plus 1 and also increments the stack register.

        Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
        Implied	                PLA	             $68	    1	        4

        Processor Status register changes
        Flag	Effect
        Zero flag	    Set if the copied value is zero, otherwise cleared.
        Negative flag	Updated to the value of bit #7 of the copied value.
     */
    pub fn pla(&mut self) {
        self.pla_implied();
        self.pla_run();
    }

    fn pla_implied(&mut self) {
        self.pc += 1;
    }

    fn pla_run(&mut self) {
        self.sp += 1;
        let addr = 0x0100 + self.sp.clone() as u16 & 0x00FF;
        self.accumulator = self.read(&addr);

        // Set if the copied value is zero, otherwise cleared.
        if self.accumulator & 0xFF == 0x00 {
            self.status.set_zero();
        } else {
            self.status.unset_zero();
        }

        // Updated to the value of bit #7 of the copied value.
        if self.accumulator & 0x80 > 0 {
            self.status.set_negative();
        } else {
            self.status.unset_negative();
        }
    }
}