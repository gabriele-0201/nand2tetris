use std::collections::HashMap;

pub fn resolve_symbols(assembly: &mut Vec<String>) {
    // Create the symbol table
    let mut symbols: HashMap<&str, u16> = HashMap::from([
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
        ("SCREEN", 16384),
        ("KBD", 24576),
        ("SP", 0),
        ("LCL", 1),
        ("ARG", 2),
        ("THIS", 3),
        ("THAT", 4),
    ]);

    // First Pass
    // Scan all lables
    // TODO: remove this assumption: in the code is NOT
    // present the error of having two lables one after the other
    let mut line_count = 0;
    let mut to_remove: Vec<usize> = vec![];
    for (index, line) in assembly.iter().enumerate() {
        //println!("line: {line}");
        //println!("count: {line_count}");
        if line.starts_with('(') {
            symbols.insert(&line[1..line.len() - 1], line_count + 1);
            to_remove.push(index);
        } else {
            line_count += 1;
        }
    }

    to_remove.into_iter().rev().for_each(|r| {
        assembly.remove(r);
    });

    for line in assembly {
        println!("{}", line);
    }

    // println!("{:?}", symbols);

    // Second Pass
    // Substitute every symbol with its
    // defined value or if not present allocate a new varible
    for (index, line) in assembly.iter().enumerate() {
        //println!("line: {line}");
        //println!("count: {line_count}");
        if line.starts_with('(') {
            symbols.insert(&line[1..line.len() - 1], line_count + 1);
            to_remove.push(index);
        } else {
            line_count += 1;
        }
    }
}
