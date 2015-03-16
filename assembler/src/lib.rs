#![feature(collections)]

pub use code::Code;
pub use parser::{Command, Parser};
pub use symbol_table::SymbolTable;

mod code;
mod parser;
mod symbol_table;

static HARDCODED_ADDRESSES: &'static [(&'static str, u16)] = &[
    ("SP", 0),
    ("LCL", 1),
    ("ARG", 2),
    ("THIS", 3),
    ("THAT", 4),
    ("R0", 0),
    ("R1", 1),
    ("R2", 2),
    ("R3", 3),
    ("R4", 4),
    ("R5", 5),
    ("R6", 6),
    ("R7", 7),
    ("R8", 8),
    ("R9", 9),
    ("R10", 10),
    ("R11", 11),
    ("R12", 12),
    ("R13", 13),
    ("R14", 14),
    ("R15", 15),
    ("SCREEN", 0x4000),
    ("KBD", 0x6000)
];

pub fn assemble(input: &str) -> Result<String, String> {
    let table = try!(parse_labels(input));
    generate_code(input, table)
}

fn parse_labels<'a>(input: &'a str) -> Result<SymbolTable<'a>, String> {
    let mut parser = Parser::new(input);
    let mut table = SymbolTable::new();

    for &(label, addr) in HARDCODED_ADDRESSES {
        table.add_entry(label, addr)
    }

    let mut icount = 0;
    let _ : Result<(), ()> = parser.each_advance(|parser| {
        if parser.command_type() == Command::L {
            table.add_entry(parser.symbol(), icount);
        } else {
            icount += 1;
        }
        None
    });

    Ok(table)
}

fn generate_code<'a>(input: &'a str, mut table: SymbolTable<'a>) -> Result<String, String> {
    let mut parser = Parser::new(input);
    let mut output = String::new();

    // Start placing variables from address 16 onwards
    let mut next_address = 16;

    let mut get_address = |sym: &'a str| -> u16 {
        if sym.chars().all(|c| c.is_digit(10)) {
            sym.parse().unwrap()
        } else if table.contains(sym) {
            table.get_address(sym)
        } else {
            table.add_entry(sym, next_address);
            let address = next_address;
            next_address += 1;
            address
        }
    };

    try!(parser.each_advance(|parser| {
        let bits = match parser.command_type() {
            Command::A => format!("0{:015b}\n", get_address(parser.symbol())),
            Command::C => {
                let comp = match Code::comp(parser.comp()) {
                    Ok(comp) => comp,
                    Err(err) => return Some(err)
                };
                let dest = Code::dest(parser.dest());
                let jump = Code::jump(parser.jump());

                // puts "111#{comp}#{dest}#{jump}"
                format!("111{}{}{}\n", comp, dest, jump)
            }
            _ => return None // Ignore labels
        };

        output.push_str(&bits);
        None
    }));

    Ok(output)
}
