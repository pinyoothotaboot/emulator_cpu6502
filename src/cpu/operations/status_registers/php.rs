use crate::cpu::model::{State, CPU};

impl CPU {
    /**
    * PHP - Push Processor Status On Stack
       Operation: P↓

       This instruction transfers the contents of the processor status reg­ ister unchanged to the stack, as governed by the stack pointer.

       The PHP instruction affects no registers or flags in the micropro­cessor.

       Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
       Implied         	PHP	                 $08	    1	        3
    */
    pub fn php(&mut self) {
        self.php_implied();
        self.php_run();
    }

    fn php_implied(&mut self) {
        self.pc += 1;
    }

    fn php_run(&mut self) {
        let addr = 0x0100 + self.sp.clone() as u16 & 0x00FF;
        let status: u8 =
            self.status.get_status() | self.status.get_break_command() | self.status.get_unused();
        self.write(&addr, status);
        self.sp -= 1;
    }
}
