use std::fs::File;
use std::io::{BufWriter, Write};
use crate::ir::Line;
use crate::symbols::SymbolTable;
use crate::mnemonics::{get_opcode};

/// Manages the buffering of the current Text Record (T-record)
struct TextRecord {
    start_addr: Option<u32>,
    buffer: Vec<u8>,
    max_len: usize,
}

impl TextRecord {
    fn new() -> Self {
        TextRecord {
            start_addr: None,
            buffer: Vec::with_capacity(30),
            max_len: 30,
        }
    }

    fn add_bytes(&mut self, addr: i32, data: &[u8]) -> Vec<String> {
        let mut output_lines = Vec::new();
        let mut data_idx = 0;
        let addr_u32 = addr as u32;

        while data_idx < data.len() {
            if self.start_addr.is_none() {
                self.start_addr = Some(addr_u32 + data_idx as u32);
            }
            let current_start = self.start_addr.unwrap();
            let current_loc = current_start + self.buffer.len() as u32;
            let incoming_loc = addr_u32 + data_idx as u32;

            if current_loc != incoming_loc {
                if let Some(line) = self.flush() {
                    output_lines.push(line);
                }
                self.start_addr = Some(incoming_loc);
            }

            let space_left = self.max_len - self.buffer.len();
            let chunk_size = std::cmp::min(space_left, data.len() - data_idx);

            self.buffer.extend_from_slice(&data[data_idx..data_idx + chunk_size]);
            data_idx += chunk_size;

            if self.buffer.len() == self.max_len {
                if let Some(line) = self.flush() {
                    output_lines.push(line);
                }
            }
        }
        output_lines
    }

    fn flush(&mut self) -> Option<String> {
        if self.buffer.is_empty() {
            return None;
        }

        let addr = self.start_addr.unwrap();
        let record_len = self.buffer.len();
        let header = format!("T{:06X}{:02X}", addr, record_len);
        let body: String = self.buffer.iter().map(|b| format!("{:02X}", b)).collect();

        self.start_addr = None;
        self.buffer.clear();

        Some(format!("{}{}", header, body))
    }
}

pub fn pass_two(
    ir: &[Line],
    symtab: &SymbolTable,
    filename: &str
) -> Result<(), String> {

    let start_addr = ir.first().map(|l| l.address).unwrap_or(0);
    // Simple calc for program length (Last Address - First Address)
    let prog_len = if let Some(last) = ir.last() {
        last.address - start_addr
    } else {
        0
    };

    let obj_filename = format!("{}.obj", filename);
    let file = File::create(&obj_filename).map_err(|e| e.to_string())?;
    let mut writer = BufWriter::new(file);

    // 1. Header Record
    let prog_name = ir.first()
        .and_then(|l| l.label.as_deref())
        .unwrap_or("      ");

    writeln!(writer, "H{:<6}{:>06X}{:>06X}", prog_name, start_addr, prog_len)
        .map_err(|e| e.to_string())?;

    // 2. Text Records
    let mut text_rec = TextRecord::new();
    // Vector to store address locations that need Modification Records
    let mut mod_records: Vec<i32> = Vec::new();
    let mut entry_point = start_addr;

    for line in ir {
        if line.mnemonic == "START" { continue; }
        if line.mnemonic == "END" {
            if let Some(ref op) = line.operand {
                entry_point = symtab.get_address(op).unwrap_or(start_addr);
            }
            break;
        }

        // Generate Object Code
        // We pass the mod_records vector to assemble_instruction to append to it
        let object_code = if get_opcode(&line.mnemonic).is_some() {
            Some(assemble_instruction(line, symtab, &mut mod_records)?)
        } else if line.mnemonic == "BYTE" {
            Some(assemble_byte(line)?)
        } else if line.mnemonic == "WORD" {
            Some(assemble_word(line)?)
        } else {
            None
        };

        if let Some(bytes) = object_code {
            let records = text_rec.add_bytes(line.address, &bytes);
            for rec in records {
                writeln!(writer, "{}", rec).map_err(|e| e.to_string())?;
            }
        } else {
            // Gap handling (RESW/RESB)
            if let Some(rec) = text_rec.flush() {
                writeln!(writer, "{}", rec).map_err(|e| e.to_string())?;
            }
        }
    }

    if let Some(rec) = text_rec.flush() {
        writeln!(writer, "{}", rec).map_err(|e| e.to_string())?;
    }

    // 3. Modification Records (New Step)
    // For Standard SIC, we modify the last 4 half-bytes (16 bits) at the specified address.
    // The modification start address is instruction_address + 1 (skipping the opcode byte).
    for addr in mod_records {
        // M<Address+1 (6)><Length (2)>
        // Length 04 represents 4 half-bytes (16 bits)
        writeln!(writer, "M{:06X}04", addr + 1).map_err(|e| e.to_string())?;
    }

    // 4. End Record
    writeln!(writer, "E{:06X}", entry_point).map_err(|e| e.to_string())?;

    println!("Output written to {}", obj_filename);
    Ok(())
}

fn assemble_instruction(
    line: &Line,
    symtab: &SymbolTable,
    mod_records: &mut Vec<i32>
) -> Result<Vec<u8>, String> {
    let op_info = get_opcode(&line.mnemonic)
        .ok_or(format!("Unknown mnemonic {}", line.mnemonic))?;

    let opcode = op_info.opcode;
    let mut address = 0;

    if let Some(ref operand) = line.operand {
        let (label, is_indexed) = if operand.ends_with(",X") {
            (&operand[..operand.len()-2], true)
        } else {
            (operand.as_str(), false)
        };

        // If the operand is a Symbol, we need a Modification Record
        // (Unless it's an absolute reference, but simplified SIC usually assumes symbols are relocatable)
        if let Some(addr) = symtab.get_address(label) {
            address = addr;
            // Record that this instruction location needs modification
            mod_records.push(line.address);
        } else {
            return Err(format!("Undefined Symbol: {}", label));
        }

        if is_indexed {
            address |= 0x8000;
        }
    } else if line.mnemonic != "RSUB" {
        return Err(format!("Missing operand for {}", line.mnemonic));
    }

    let b1 = opcode;
    let b2 = (address >> 8) as u8;
    let b3 = (address & 0xFF) as u8;

    Ok(vec![b1, b2, b3])
}

fn assemble_byte(line: &Line) -> Result<Vec<u8>, String> {
    let operand = line.operand.as_ref()
        .ok_or(format!("BYTE requires operand at line {}", line.source_line))?;

    if operand.starts_with("C'") && operand.ends_with('\'') {
        let content = &operand[2..operand.len()-1];
        Ok(content.as_bytes().to_vec())
    } else if operand.starts_with("X'") && operand.ends_with('\'') {
        let hex_str = &operand[2..operand.len()-1];
        if hex_str.len() % 2 != 0 {
            return Err("X constant must have even number of digits".to_string());
        }
        let mut bytes = Vec::new();
        for i in (0..hex_str.len()).step_by(2) {
            let byte_val = u8::from_str_radix(&hex_str[i..i+2], 16)
                .map_err(|_| "Invalid hex in X constant")?;
            bytes.push(byte_val);
        }
        Ok(bytes)
    } else {
        Err(format!("Invalid BYTE format: {}", operand))
    }
}

fn assemble_word(line: &Line) -> Result<Vec<u8>, String> {
    let operand = line.operand.as_ref()
        .ok_or(format!("WORD requires operand at line {}", line.source_line))?;

    let value = operand.parse::<i32>()
        .map_err(|_| format!("Invalid WORD constant: {}", operand))?;

    let min = -(1 << 23);
    let max = (1 << 23) - 1;
    if value < min || value > max {
        return Err(format!("WORD value too large for 24-bits: {}", value));
    }

    let bytes = value.to_be_bytes();
    Ok(vec![bytes[1], bytes[2], bytes[3]])
}