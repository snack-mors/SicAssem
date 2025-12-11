use std::io::{BufRead, BufReader};
use std::fs::File;
use crate::ir::Line;

use crate::symbols::SymbolTable;
use crate::mnemonics::{get_opcode, is_directive};




pub fn pass_one(filename: &str) -> Result<(SymbolTable, Vec<Line>), String> {
    let file = File::open(filename).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    let mut symtab = SymbolTable::new();
    let mut intermediate_code = Vec::new();
    let mut locctr = 0;
    let mut start_seen = false;

    for (index, line_result) in reader.lines().enumerate() {
        let source_line_number = index + 1;
        let line = line_result.map_err(|e| e.to_string())?;

        let tokens: Vec<&str> = line.split_whitespace().collect();

        // Checks for comments.
        if tokens.is_empty() || tokens[0].starts_with('#') || tokens[0].starts_with('.') {
            continue;
        }

        let mut label: Option<&str> = None;
        let mut mnemonic: &str;
        let mut operand: Option<&str> = None;

        match tokens.len() {
            3 => {
                // LABEL MNEMONIC OPERAND
                label = Some(tokens[0]);
                mnemonic = tokens[1];
                operand = Some(tokens[2]);
            }
            2 => {
                // Either "LABEL MNEMONIC" or "MNEMONIC OPERAND"
                // If the first token is an instruction, it's MNEMONIC OPERAND
                if get_opcode(tokens[0]).is_some() || is_directive(tokens[0]) {
                    mnemonic = tokens[0];
                    operand = Some(tokens[1]);
                }
                else {
                    label = Some(tokens[0]);
                    mnemonic = tokens[1];
                }
            }
            1 => {
                // MNEMONIC ONLY
                mnemonic = tokens[0];
            }
            _ => return Err(format!("Line: {}: Too many tokens", source_line_number)),
        }

        if mnemonic == "START" {
            if let Some(op) = operand {
                // Parse hex: strtol(op, NULL, 16)
                locctr = i32::from_str_radix(op, 16).unwrap_or(0);
            }
            start_seen = true;

            intermediate_code.push(Line::new(
                locctr,
                label,
                mnemonic,
                operand,
                source_line_number
            ));
            continue;
        }

        let current_address = locctr;

        if let Some(lbl) = label {
            if symtab.insert(lbl.to_string(), current_address, source_line_number as i32).is_err(){
                return Err(format!("Line: {}: Duplicate Symbol '{}'", source_line_number, lbl));

            }
        }

        let mut instruction_size = 0;

        if get_opcode(mnemonic).is_some() {
            instruction_size = 3;
        } else if is_directive(mnemonic) {
            match mnemonic {
                "WORD" => instruction_size = 3,
                "RESW" => {
                    let count = operand.unwrap().parse::<i32>().map_err(|_| "Invalid RESW")?;
                    instruction_size = 3 * count;
                },
                "RESB" => {
                    let count = operand.unwrap().parse::<i32>().map_err(|_| "Invalid RESB")?;
                    instruction_size = count;
                },
                "BYTE" => {
                    let op = operand.unwrap();
                    if op.starts_with("C'") && op.ends_with('\'') {
                        instruction_size = (op.len() - 3) as i32;
                    } else if op.starts_with("X'") && op.ends_with('\'') {
                        let hex_len = op.len() - 3;
                        instruction_size = (hex_len / 2) as i32;
                    } else {
                        return Err(format!("Line {}: Invalid BYTE literal", source_line_number));
                    }
                },
                "END" => {}
                _ => return Err(format!("Line {}: Unknown directive '{}'", source_line_number, mnemonic)),

            }
        }
        else {
            return Err(format!("Line {}: Unknown Opcode '{}'", source_line_number, mnemonic));
        }

        intermediate_code.push(Line::new(
            current_address,
            label,
            mnemonic,
            operand,
            source_line_number
        ));

        locctr += instruction_size;

        if mnemonic == "END" {
            break;
        }

    }

    if !start_seen {
        return Err("Error: Missing START directive".to_string());
    }

    Ok((symtab, intermediate_code))
}