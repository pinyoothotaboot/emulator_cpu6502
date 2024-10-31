// Although most of the following discussion will consider how one
// operates with a general purpose register called the accumulator, it must
// be understood that data has to transfer between the accumulator and outside sources by means of passing through the microprocessor to 8 lines
// called the data bus. The outside sources include the program which controls the microprocessor, the memory which will be used as interim storage for internal registers when they are to be used in a current operation, and the actual communications to the world through input/output
// ports. Later in this document performance of transfers to and from each
// of these devices will be discussed. However, at present, discussion
// will center on the microprocessor itself.

// The only operation of the data bus is to transfer data between memory and the processor's internal registers such as the accumulator. Figure 2.1 displays the basic communication between the accumulator, A, and
// the memory, M, through the use of 8 bi-directional data lines called the
// data bus.
// _______________________________________________________________
//
//                      8-bit data bus
// _______________________________________________________________
//        |               |               |               |
//        |               |               |               |
//   _____|______      ___|____           |               |
//  |            |    |        |     _____|_____     _____|_____
//  |            |    |  ROM   |    |           |   |           |
//  |    CPU     |    |________|    |           |   |           |
//  |            |     ____|___     |    RAM    |   |    I/O    |
//  |____________|    |        |    |           |   |           |
//     |   |          |  MMC   |    |___________|   |___________|
//     |   |          |________|      |     |           |    |
//     |   |            |   |         |     |           |    |
// ____|___|____________|___|_________|_____|___________|____|___
//     |                | 8-bit control bus |                |
// ____|________________|___________________|________________|___
//     |                |                   |                |
// ____|________________|___________________|________________|___
//                   16-bit address bus
// ______________________________________________________________
//

use crate::ram::model::Memory;

pub struct Bus {
    pub memory: Memory,
}
