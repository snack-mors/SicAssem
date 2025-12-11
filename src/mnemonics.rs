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

// src/directives.rs

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Directive {
    Start,
    End,
    Byte,
    Word,
    Resb,
    Resw,
}

impl Directive {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "START" => Some(Directive::Start),
            "END"   => Some(Directive::End),
            "BYTE"  => Some(Directive::Byte),
            "WORD"  => Some(Directive::Word),
            "RESB"  => Some(Directive::Resb),
            "RESW"  => Some(Directive::Resw),
            _       => None,
        }
    }
    // We pass the operand because RESW/BYTE need it to calculate size.
    pub fn get_size(&self, operand: Option<&str>) -> Result<i32, String> {
        match self {
            Directive::Word => Ok(3),
            Directive::Start | Directive::End => Ok(0),

            Directive::Resw => {
                let val = operand.ok_or("Missing operand for RESW")?
                    .parse::<i32>()
                    .map_err(|_| "Invalid integer for RESW")?;
                Ok(val * 3)
            },

            Directive::Resb => {
                let val = operand.ok_or("Missing operand for RESB")?
                    .parse::<i32>()
                    .map_err(|_| "Invalid integer for RESB")?;
                Ok(val)
            },

            Directive::Byte => {
                let op = operand.ok_or("Missing operand for BYTE")?;
                if op.starts_with("C'") && op.ends_with('\'') {
                    // C'EOF' -> 3 bytes
                    Ok((op.len() - 3) as i32)
                } else if op.starts_with("X'") && op.ends_with('\'') {
                    // X'F1' -> 1 byte
                    let hex_len = op.len() - 3;
                    if hex_len % 2 != 0 {
                        return Err("Hex literal must have even number of digits".to_string());
                    }
                    Ok((hex_len / 2) as i32)
                } else {
                    Err("Invalid BYTE format".to_string())
                }
            }
        }
    }
}