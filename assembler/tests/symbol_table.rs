extern crate hack_assembler;

use hack_assembler::*;

#[test]
fn empty() {
    let table = SymbolTable::new();
    assert!(!table.contains("SCREEN"));
    assert!(!table.contains("KBD"));
}

mod when_added {
    use hack_assembler::*;

    #[test]
    fn it_contains() {
        let mut table = SymbolTable::new();
        table.add_entry("SCREEN", 0x4000);

        assert!(table.contains("SCREEN"));
    }

    #[test]
    fn has_expected_address() {
        let mut table = SymbolTable::new();
        table.add_entry("SCREEN", 0x4000);

        assert_eq!(table.get_address("SCREEN"), 0x4000);
    }

    #[test]
    fn has_no_extra() {
        let mut table = SymbolTable::new();
        table.add_entry("SCREEN", 0x4000);

        assert!(!table.contains("KBD"));
    }
}
