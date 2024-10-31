use crate::cpu::model::CPU;

impl CPU {
    /**
    * DEX - Decrement Index Register X By One
      Operation: X - 1 → X

      This instruction subtracts one from the current value of the index register X and stores the result in
      the index register X.

      DEX does not affect the carry or overflow flag, it sets the N flag if it has bit 7 on as a result of the decrement,
      otherwise it resets the N flag; sets the Z flag if X is a 0 as a result of the decrement, otherwise it resets the Z flag.

      Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
      Implied	            DEX	                 $CA	    1	        2

      Processor Status register changes
      Flag	Effect
      Zero flag	    Set if the result is zero, otherwise cleared.
      Negative flag	Updated to the value of bit #7 of the result.
    */
    pub fn dex(&mut self) {
        self.dex_execute();
        self.dex_run();
    }

    fn dex_execute(&mut self) {
        // PC = PC + 1
        self.pc += 1;
    }

    fn dex_run(&mut self) {
        self.x_register -= 1;

        // Set if the result is zero, otherwise cleared.
        if self.x_register & 0xFF == 0x00 {
            self.status.set_zero();
        } else {
            self.status.unset_zero();
        }

        // Updated to the value of bit #7 of the result.
        if self.x_register & 0x80 > 0x00 {
            self.status.set_negative();
        } else {
            self.status.unset_negative();
        }
    }
}
