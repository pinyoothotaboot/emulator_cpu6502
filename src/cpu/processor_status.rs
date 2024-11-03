/*
    CPU 6502 process status flags : http://wiki.nesdev.com/w/index.php/Status_flags
     processor status (PS) <-- Byte
     7 6 5 4 3 2 1 0 <-- Bit
     N V _ B D I Z C <-- Flag
     | |   | | | | +--- Carry Flag                     0 = FALSE                 1 = TRUE
     | |   | | | +----- Zero Flag                      0 = RESULT NOT ZERO       1 = RESULT ZERO
     | |   | | +------- Interrupt Disable (IRQ)        0 = ENABLE                1 = DISABLE
     | |   | +--------- Decimal Mode (not used on NES) 0 = FALSE                 1 = TRUE
     | |   +----------- Break Command                  0 = NO BREAK              1 = BREAK
     | +--------------- Overflow Flag                  0 = FALSE                 1 = TRUE
     +----------------- Negative Flag                  0 = POSITIVE              1 = NEGATIVE

*/

pub struct ProcessorStatus {
    N: u8,
    V: u8,
    U: u8,
    B: u8,
    D: u8,
    I: u8,
    Z: u8,
    C: u8,
}

impl ProcessorStatus {
    pub fn new() -> Self {
        ProcessorStatus {
            N: 0x00,
            V: 0x00,
            U: 0x01,
            B: 0x01,
            D: 0x00,
            I: 0x00,
            Z: 0x00,
            C: 0x00,
        }
    }

    pub fn reset(&mut self) {
        self.N = 0x00;
        self.V = 0x00;
        self.U = 0x01;
        self.B = 0x01;
        self.D = 0x00;
        self.I = 0x00;
        self.Z = 0x00;
        self.C = 0x00;
    }

    pub fn get_status(&self) -> u8 {
        return self.N << 7
            | self.V << 6
            | self.U << 5
            | self.B << 4
            | self.D << 3
            | self.I << 2
            | self.Z << 1
            | self.C;
    }

    pub fn set_status(&mut self, status: u8) {
        let operator: u8 = 0b0000_0001;
        self.N = status >> 7 & operator;
        self.V = status >> 6 & operator;
        self.U = status >> 5 & operator;
        self.B = status >> 4 & operator;
        self.D = status >> 3 & operator;
        self.I = status >> 2 & operator;
        self.Z = status >> 1 & operator;
        self.C = status & operator;
    }

    pub fn set_overflow(&mut self) {
        self.V = 0x01;
    }

    pub fn unset_overflow(&mut self) {
        self.V = 0x00;
    }

    pub fn set_unused(&mut self) {
        self.U = 0x01;
    }

    pub fn get_unused(&self) -> u8 {
        return self.U.clone();
    }

    pub fn unset_unused(&mut self) {
        self.U = 0x00;
    }

    pub fn set_break_command(&mut self) {
        self.B = 0x01;
    }

    pub fn unset_break_command(&mut self) {
        self.B = 0x00;
    }

    pub fn get_break_command(&self) -> u8 {
        return self.B.clone();
    }

    pub fn set_decimal_mode(&mut self) {
        self.D = 0x01;
    }

    pub fn unset_decimal_mode(&mut self) {
        self.D = 0x00;
    }

    pub fn set_interrupt_disable(&mut self) {
        self.I = 0x01;
    }

    pub fn unset_interrupt_disable(&mut self) {
        self.I = 0x00;
    }

    pub fn set_zero(&mut self) {
        self.Z = 0x01;
    }

    pub fn unset_zero(&mut self) {
        self.Z = 0x00;
    }

    pub fn get_zero(&self) -> u8 {
        return self.Z.clone();
    }

    pub fn set_carry(&mut self) {
        self.C = 0x01;
    }

    pub fn unset_carry(&mut self) {
        self.C = 0x00;
    }

    pub fn get_carry(&self) -> u8 {
        return self.C.clone()
    }

    pub fn set_negative(&mut self) {
        self.N = 0x01;
    }

    pub fn unset_negative(&mut self) {
        self.N = 0x00;
    }

    pub fn get_negative(&self) ->u8 {
        return self.N.clone();
    }
}
