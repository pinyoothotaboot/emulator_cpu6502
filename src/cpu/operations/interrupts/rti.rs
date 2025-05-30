use crate::cpu::model::{State, CPU};

impl CPU {
    /**
    * RTI - Return From Interrupt
       Operation: P↑ PC↑

       This instruction transfers from the stack into the microprocessor the processor status and the program counter location for the instruction which was interrupted. By virtue of the interrupt having stored this data before executing the instruction and thei fact that the RTI reinitializes the microprocessor to the same state as when it was interrupted, the combination of interrupt plus RTI allows truly reentrant coding.

       The RTI instruction reinitializes all flags to the position to the point they were at the time the interrupt was taken and sets the program counter back to its pre-interrupt state. It affects no other registers in the microprocessor.

       Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
       Implied	            RTI	                  $40	    1	        6

       Processor Status register changes
       Updates all flags since the entire value for this register is updated.

       Effectively clears the Interrupt Disable flag if the updated value does not have this flag set.
    */
    pub fn rti(&mut self) {
        self.rti_implied();
        self.rti_run();
    }

    fn rti_implied(&mut self) {
        // Pull processor status from stack
        self.sp += 1;
        let addr_stck = 0x0100 + self.sp as u16 & 0x00FF;
        self.data = self.read(&addr_stck);
        let b = self.status.get_break_command();
        let u = self.status.get_unused();

        self.data &= !(b << 4);
        self.data &= !(u << 5);

        self.status.set_status(self.data);

        // Pull PCL from stack
        self.sp += 1;
        let addr_stck = 0x0100 + self.sp as u16 & 0x00FF;
        let pcl = self.read(&addr_stck);

        // Pull PCH from stack
        self.sp += 1;
        let addr_stck = 0x0100 + self.sp as u16 & 0x00FF;
        let pch = self.read(&addr_stck);

        self.address_register = ((pch as u16) << 8) & 0xFF00 | (pcl as u16) & 0x00FF;
    }

    fn rti_run(&mut self) {
        self.pc = self.address_register;
    }
}
