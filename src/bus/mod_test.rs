#[cfg(test)]
mod tests {
    use super::*;
    use crate::bus::model::Bus;

    #[test]
    fn test_bus_read_write() {
        let mut bus = Bus::new();
        
        // Test writing and reading a byte
        bus.write(0x0000, 0x42);
        assert_eq!(bus.read(0x0000), 0x42);
        
        // Test writing to a different address
        bus.write(0x1234, 0xAB);
        assert_eq!(bus.read(0x1234), 0xAB);
        
        // Ensure other memory locations are unchanged
        assert_eq!(bus.read(0x0000), 0x42);
    }

    #[test]
    fn test_bus_read_write_word() {
        let mut bus = Bus::new();
        
        // Test writing and reading a 16-bit word (little-endian)
        bus.write_word(0x1000, 0x1234);
        
        // Check that the bytes are stored in little-endian order
        assert_eq!(bus.read(0x1000), 0x34);  // Low byte
        assert_eq!(bus.read(0x1001), 0x12);  // High byte
        
        // Test reading the word back
        assert_eq!(bus.read_word(0x1000), 0x1234);
    }

    #[test]
    fn test_bus_mirroring() {
        let mut bus = Bus::new();
        
        // Test RAM mirroring (0x0000-0x07FF is mirrored every 0x0800 bytes up to 0x1FFF)
        bus.write(0x0000, 0x12);
        bus.write(0x0800, 0x34);
        bus.write(0x1000, 0x56);
        bus.write(0x1800, 0x78);
        
        // All these should read the same value (last write wins)
        assert_eq!(bus.read(0x0000), 0x78);
        assert_eq!(bus.read(0x0800), 0x78);
        assert_eq!(bus.read(0x1000), 0x78);
        assert_eq!(bus.read(0x1800), 0x78);
        
        // Test that writing to mirrored locations works
        bus.write(0x0001, 0xAB);
        assert_eq!(bus.read(0x0801), 0xAB);
        assert_eq!(bus.read(0x1001), 0xAB);
        assert_eq!(bus.read(0x1801), 0xAB);
    }

    #[test]
    fn test_bus_ppu_registers() {
        let mut bus = Bus::new();
        
        // Test PPU register mirroring (0x2000-0x2007 is mirrored every 8 bytes up to 0x3FFF)
        bus.write(0x2000, 0x12);
        bus.write(0x2008, 0x34);  // Should mirror to 0x2000
        bus.write(0x3FF8, 0x56);  // Should mirror to 0x2000
        
        // All these should read the same value (last write wins)
        assert_eq!(bus.read(0x2000), 0x56);
        assert_eq!(bus.read(0x2008), 0x56);
        assert_eq!(bus.read(0x3FF8), 0x56);
    }
}
