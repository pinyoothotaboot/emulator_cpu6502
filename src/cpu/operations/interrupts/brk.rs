use crate::cpu::model::{State, CPU};

impl CPU {
    /**
    * BRK - Break Command
       Operation: PC + 2↓, [FFFE] → PCL, [FFFF] → PCH

       The break command causes the microprocessor to go through an inter­ rupt sequence under program control. This means that the program counter of the second byte after the BRK. is automatically stored on the stack along with the processor status at the beginning of the break instruction. The microprocessor then transfers control to the interrupt vector.

       Other than changing the program counter, the break instruction changes no values in either the registers or the flags.

       Note on the MOS 6502:

       If an IRQ happens at the same time as a BRK instruction, the BRK instruction is ignored.


       Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
       Implied	            BRK	                  $00	    1	        7

       Processor Status register changes
       Sets the Interrupt Disable flag.
    */
    pub fn brk(&mut self) {
        self.brk_implied();
        self.brk_run();
    }

    fn brk_implied(&mut self) {
        // PC + 1
        // PC on hardware interrupt
        self.pc += 1;

        // Push high order byte of program counter to stack
        let pch = ((self.pc >> 8) & 0x00FF) as u8;
        let addr_stck_pch = 0x0100 + self.sp as u16 & 0x00FF;
        self.write(&addr_stck_pch, pch);

        // Push low order byte of program counter to stack
        self.sp -= 1;
        let pcl = (self.pc & 0x00FF) as u8;
        let addr_stck_pcl = 0x0100 + self.sp as u16 & 0x00FF;
        self.write(&addr_stck_pcl, pcl);

        // Push status register to stack
        self.sp -= 1;
        self.status.set_interrupt_disable();
        self.status.set_break_command();
        let status = self.status.get_status();
        let addr_stck_sta = 0x0100 + self.sp as u16 & 0x00FF;
        self.write(&addr_stck_sta, status & 0xFF);

        self.status.unset_break_command();
        self.sp -= 1;

        // Fetch low order byte of interrupt vector
        // NMI -> 0xFFFA
        // RES -> 0xFFFD
        let adl = self.read(&0xFFFE);

        // Fetch high order byte of interrupt vector
        // NMI -> 0xFFFB
        // RES -> 0xFFFD
        let adh = self.read(&0xFFFF);

        // Interrupt vector
        self.address_register = ((adh as u16) << 8) & 0xFF00 | (adl as u16) & 0x00FF;
    }

    fn brk_run(&mut self) {
        self.pc = self.address_register;
    }
}
