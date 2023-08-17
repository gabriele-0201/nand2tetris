use std::collections::HashMap;

pub fn resolve_symbols(assembly: &mut Vec<String>) {
    // Create the symbol table
    let mut symbols: HashMap<String, u16> = HashMap::from([
        ("R0".to_string(), 0),
        ("R1".to_string(), 1),
        ("R2".to_string(), 2),
        ("R3".to_string(), 3),
        ("R4".to_string(), 4),
        ("R5".to_string(), 5),
        ("R6".to_string(), 6),
        ("R7".to_string(), 7),
        ("R8".to_string(), 8),
        ("R9".to_string(), 9),
        ("R10".to_string(), 10),
        ("R11".to_string(), 11),
        ("R12".to_string(), 12),
        ("R13".to_string(), 13),
        ("R14".to_string(), 14),
        ("R15".to_string(), 15),
        ("SCREEN".to_string(), 16384),
        ("KBD".to_string(), 24576),
        ("SP".to_string(), 0),
        ("LCL".to_string(), 1),
        ("ARG".to_string(), 2),
        ("THIS".to_string(), 3),
        ("THAT".to_string(), 4),
    ]);

    // First Pass
    // Scan all lables
    // TODO: remove this assumption: in the code is NOT
    // present the error of having two lables one after the other
    let mut line_count = 0;
    let mut to_remove: Vec<usize> = vec![];
    for (index, line) in assembly.iter().enumerate() {
        if line.starts_with('(') {
            symbols.insert(line[1..line.len() - 1].to_string(), line_count);
            to_remove.push(index);
        } else {
            line_count += 1;
        }
    }

    to_remove.into_iter().rev().for_each(|r| {
        assembly.remove(r);
    });

    for line in assembly.iter().by_ref() {}

    // Second Pass
    // Substitute every symbol with its
    // defined value or if not present allocate a new varible
    let mut variable_count = 16;
    for line in assembly.iter_mut() {
        if !line.starts_with('@') {
            continue;
        }

        let symbol_range = 1..line.len();
        let possible_symbol = &line[symbol_range.clone()];
        let symbol = match possible_symbol.parse::<u16>() {
            Ok(_) => continue,
            Err(_) => possible_symbol.to_string(),
        };
        // let symbol = line[symbol_range.clone()].to_string();
        let symbol_val = match symbols.get(&symbol) {
            Some(val) => *val,
            None => {
                let val = variable_count;
                variable_count += 1;
                symbols.insert(symbol, val);
                val
            }
        };

        line.replace_range(symbol_range, &symbol_val.to_string());
    }
}
