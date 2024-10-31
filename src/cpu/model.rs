//                                            __________________
//                                            PPU               |
//                                            __________________|
//    NV-B DIZC                               ALU               |      ______
//    ___________                                               |     |      |
//   |_0011_0000_|--|                                           |-----| 03   | X Reg.
//     P Stastus    |    __________________                     |     |______|
//                  |    ___________________                    |
//                  |---|___________________|-------------------|       ______
//                  |   |___| Accumulator                       |      |      |
//                  |                                           |------| D8   | Y Reg.
//                  |                                           |      |______|
//                  |___________________________________________|
//                                                              |
//         _________                                            |
//     PC |__0804___|                                           |     _____________
//             |              Address Register                  |----| LDA $$,X    |
//             |                 ________                       |    |_____________|
//             |----------------|________|----------------------|      Decoder
//             |                                                |
//             |             _____________________              |       ____________
//             |            |                     |             |      |            |
//             |            |     0801 :    03    |             |______| 01B9 : FF  |
//         ____|____        |     0802 :    BD    |          ___|__    | 01B8 : FF  |
//        |__0803___|-------|     0803 :    00    |---------|__00__|   | 01B7 : FF  |
//          Address         |     0804 :    08    |           Data     | 01B6 : FF  |
//                          |     0805 :    8D    |                    |____________|
//                          |     0806 :    00    |                       Mem Stack
//                          |     0807 :    10    |       Fetch
//                          |     0808 :    CA    |      _________
//                          |                     |     |_Execute_|
//                          |                     |
//                          |_____________________|
//                                   Memory
//
use crate::bus::model::Bus;
use crate::cpu::instruction::Instruction;
use crate::cpu::processor_status::ProcessorStatus;

pub enum State {
    Fetch,
    Decode,
    Execute,
    Exit,
}

pub struct CPU {
    /*
       Program Counter (PC) - holds the address for
       the next machine language instruction to be executed.
    */
    pub pc: u16,
    pub address: u16,
    pub address_register: u16,
    /**
     * The word 8-bit
     */
    pub data: u8,
    /*
       Index Register X (X) - used as an offset in specific memory addressing modes (more on this later).
       Can be used for auxiliary storage needs (holding temp values, being used as a counter, etc.)
    */
    pub x_register: u8,
    /*
       Index Register Y (Y) - similar use cases as register X.
    */
    pub y_register: u8,
    /*
       Accumulator (A) - stores the results of arithmetic, logic,
       and memory access operations. It used as an input parameter for some operations.
    */
    pub accumulator: u8,
    /**
     * CPU6502 instruction decoded
     */
    pub instruction_reg: Option<&'static Instruction>,
    /*
       Processor status (P) - 8-bit register represents 7 status flags
       that can be set or unset depending on the result of
       the last executed instruction (for example Z flag is set (1)
       if the result of an operation is 0, and is unset/erased (0) otherwise)
    */
    pub status: ProcessorStatus,
    pub cycles: u16,
    pub state: State,
    pub bus: Bus,
}
