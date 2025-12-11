use std::io::{BufRead, BufReader};
use std::fs::File;
use crate::ir::Line;
use crate::symbols::SymbolTable;
use crate::mnemonics::{get_opcode, Directive};

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

        let (label, mnemonic, operand) = match tokens.len() {
            3 => (Some(tokens[0]), tokens[1], Some(tokens[2])),
            2 => {
                if get_opcode(tokens[0]).is_some() || Directive::from_str(tokens[0]).is_some() {
                    (None, tokens[0], Some(tokens[1]))
                } else {
                    (Some(tokens[0]), tokens[1], None)
                }
            },
            1 => (None, tokens[0], None),
            _ => return Err(format!("Line {}: Too many tokens", source_line_number)),
        };

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
        } else if let Some(dir) = Directive::from_str(mnemonic) {
            instruction_size = dir.get_size(operand)
                .map_err(|e| format!("Line {}: {}", source_line_number, e))?;
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