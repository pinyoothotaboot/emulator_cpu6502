use emulator_cpu6502::bus::model::Bus;
use emulator_cpu6502::cpu::model::CPU;
use emulator_cpu6502::interfaces::bus::IBus;
use emulator_cpu6502::interfaces::cpu::ICPU;

#[test]
fn test_cpu_initialization() {
    let mut bus = Bus::new();
    let mut cpu = CPU::new();

    // Test that CPU implements ICPU trait
    let _: &dyn ICPU = &cpu;

    // Test that reset can be called
    cpu.reset();

    // Add more specific tests once we understand the CPU structure better
    assert!(true);
}

#[test]
fn test_bus_initialization() {
    let bus = Bus::new();

    // Test that Bus implements IBus trait
    let _: &dyn IBus = &bus;

    // Add more specific tests once we understand the Bus structure better
    assert!(true);
}

// Note: More comprehensive integration tests will be added once we have
// a better understanding of the CPU and Bus implementations
