mod instructions;
mod symbols;

use instructions::Instruction;

fn main() {
    let mut args = std::env::args().skip(1).take(2); // extract the second line arguments, the others will be discarded

    let (in_file_name, out_file_name) = (
        args.next().expect("Assembly in file name required"),
        args.next().expect("Assembly out file name required"),
    );

    let raw_assembly = std::fs::read_to_string(in_file_name).expect("File not exists");
    let mut assembly = vec![String::new()];

    // Remove whitespace, empty lines and comments

    let mut raw_chars_iter = raw_assembly.chars();

    while let Some(c) = raw_chars_iter.next() {
        match c {
            // white spaces are ignored
            ' ' => {}
            '\n' | '\r' => {
                if let Some(last_line) = assembly.last() {
                    // completely empty lines are ignored
                    if !last_line.is_empty() {
                        // every time a new line is pushed the previous
                        // one could be already parsed
                        assembly.push(String::new());
                    }
                }
            }
            '/' => {
                // if also the next character is '/' then
                // skip until '/r' or '/n'
                if let Some('/') = raw_chars_iter.next() {
                    loop {
                        let c = raw_chars_iter.next();
                        if (c == Some('\n')) || (c == Some('\r')) || (c == None) {
                            break;
                        }
                    }
                }
            }
            _ => {
                assembly
                    .last_mut()
                    .expect("Assembly not initialized")
                    .push(c);
            }
        }
    }

    // remove last line if empty
    if assembly.last().expect("Empty file?").is_empty() {
        assembly.pop();
    }

    // Resolve all symbols
    symbols::resolve_symbols(&mut assembly);
    // Parse the code to a sequence of Instructions
    let instructions: Vec<Instruction> = assembly
        .into_iter()
        .enumerate()
        .map(|(i, x)| Instruction::parse(x).expect(format!("Error in instruciton {i}").as_str()))
        .collect();

    // Encode the instructions into their binary representation
    let hack_code = instructions
        .iter()
        .map(|i| i.encode())
        .collect::<Vec<String>>()
        .join("\n");

    std::fs::write(out_file_name, hack_code).expect("Impossible write into file");
}
