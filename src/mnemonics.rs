#[derive(Debug, Clone, Copy)]
pub struct OpInfo{
    pub opcode: u8,
    pub format: u8,
}

pub fn get_opcode(mnemonic: &str) -> Option<OpInfo> {
    match mnemonic {
        "ADD"  => Some(OpInfo { opcode: 0x18, format: 3 }),
        "AND"  => Some(OpInfo { opcode: 0x40, format: 3 }),
        "COMP" => Some(OpInfo { opcode: 0x28, format: 3 }),
        "DIV"  => Some(OpInfo { opcode: 0x24, format: 3 }),
        "J"    => Some(OpInfo { opcode: 0x3C, format: 3 }),
        "JEQ"  => Some(OpInfo { opcode: 0x30, format: 3 }),
        "JGT"  => Some(OpInfo { opcode: 0x34, format: 3 }),
        "JLT"  => Some(OpInfo { opcode: 0x38, format: 3 }),
        "JSUB" => Some(OpInfo { opcode: 0x48, format: 3 }),
        "LDA"  => Some(OpInfo { opcode: 0x00, format: 3 }),
        "LDCH" => Some(OpInfo { opcode: 0x50, format: 3 }),
        "LDL"  => Some(OpInfo { opcode: 0x08, format: 3 }),
        "LDX"  => Some(OpInfo { opcode: 0x04, format: 3 }),
        "MUL"  => Some(OpInfo { opcode: 0x20, format: 3 }),
        "OR"   => Some(OpInfo { opcode: 0x44, format: 3 }),
        "RD"   => Some(OpInfo { opcode: 0xD8, format: 3 }),
        "RSUB" => Some(OpInfo { opcode: 0x4C, format: 3 }),
        "STA"  => Some(OpInfo { opcode: 0x0C, format: 3 }),
        "STCH" => Some(OpInfo { opcode: 0x54, format: 3 }),
        "STL"  => Some(OpInfo { opcode: 0x14, format: 3 }),
        "STSW" => Some(OpInfo { opcode: 0xE8, format: 3 }), 
        "STX"  => Some(OpInfo { opcode: 0x10, format: 3 }),
        "SUB"  => Some(OpInfo { opcode: 0x1C, format: 3 }),
        "TD"   => Some(OpInfo { opcode: 0xE0, format: 3 }),
        "TIX"  => Some(OpInfo { opcode: 0x2C, format: 3 }),
        "WD"   => Some(OpInfo { opcode: 0xDC, format: 3 }),

        _ => None, // Default case or non-matching.
    }
}

pub fn is_directive(mnemonic: &str) -> bool {
    let directives = ["START", "END", "WORD", "BYTE", "RESW", "RESB"];

    directives.contains(&mnemonic)
}