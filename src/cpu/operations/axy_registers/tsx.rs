use crate::cpu::model::{State, CPU};

impl CPU {
    /**
    * TSX - Transfer Stack Pointer To Index X
      Operation: S → X

      This instruction transfers the value in the stack pointer to the index register X.

      TSX does not affect the carry or overflow flags. It sets N if bit 7 is on in index X as a result of the instruction, otherwise it is reset. If index X is zero as a result of the TSX, the Z flag is set, other­ wise it is reset. TSX changes the value of index X, making it equal to the content of the stack pointer.

      Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
      Implied	            TSX	                  $BA	    1	        2

      Processor Status register changes
      Flag	Effect
      Zero flag	    Set if the copied byte is zero, otherwise cleared.
      Negative flag	Updated to the value of bit #7 of the copied byte.
    */
    pub fn tsx(&mut self) {
        self.tsx_implied();
        self.tsx_run();
    }

    fn tsx_implied(&mut self) {
        // PC = PC + 1
        self.pc += 1;
    }

    fn tsx_run(&mut self) {
        self.x_register = self.sp.clone();

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
