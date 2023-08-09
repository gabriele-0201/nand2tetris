use std::collections::HashMap;

#[derive(Debug)]
pub enum Instruction {
    A(u16),
    // dest, comp, jump
    C(String, String, String),
}

lazy_static::lazy_static! {
    static ref DEST_MAP: HashMap<&'static str, &'static str> = HashMap::from([
        ("", "000"),
        ("M", "001"),
        ("D", "010"),
        ("MD", "011"),
        ("A", "100"),
        ("AM", "101"),
        ("AD", "110"),
        ("AMD", "111"),
    ]);

    static ref COMP_MAP: HashMap<&'static str, &'static str> = HashMap::from([
        ("0", "0101010"),
        ("1", "0111111"),
        ("-1", "0111010"),
        ("D", "0001100"),
        ("A", "0110000"),
        ("!D", "0001101"),
        ("!A", "0110001"),
        ("-D", "0001111"),
        ("-A", "0110011"),
        ("D+1", "0011111"),
        ("A+1", "0110111"),
        ("D-1", "0001110"),
        ("A-1", "0110010"),
        ("D+A", "0000010"),
        ("D-A", "0010011"),
        ("A-D", "0000111"),
        ("D&A", "0000000"),
        ("D|A", "0010101"),
        ("M", "1110000"),
        ("!M", "1110001"),
        ("-M", "1110011"),
        ("M+1", "1110111"),
        ("M-1", "1110010"),
        ("D+M", "1000010"),
        ("D-M", "1010011"),
        ("M-D", "1000111"),
        ("D&M", "1000000"),
        ("D|M", "1010101"),
    ]);

    static ref JUMP_MAP: HashMap<&'static str, &'static str> = HashMap::from([
        ("", "000"),
        ("JGT", "001"),
        ("JEQ", "010"),
        ("JGE", "011"),
        ("JLT", "100"),
        ("JNE", "101"),
        ("JLE", "110"),
        ("JMP", "111"),
    ]);
}

impl Instruction {
    pub fn parse(instruction: String) -> Result<Instruction, ()> {
        // A-instruction
        if instruction.starts_with("@") {
            return Ok(Instruction::A(
                instruction.as_str()[1..].parse().map_err(|_| ())?,
            ));
        }

        // C-instruction
        let dest_ub = instruction.find('=').unwrap_or(0);
        let jump_lb = instruction.find(';').unwrap_or(instruction.len());

        macro_rules! check {
            ($name: ident, $map: ident) => {
                if let None = $map.get(&$name.as_str()) {
                    return Err(());
                }
            };
        }
        // TODO: check that every substring is a valid instruction
        let dest = instruction[0..dest_ub].to_string();
        println!("dest: {dest}");
        check!(dest, DEST_MAP);
        let comp = instruction[if dest_ub == 0 { 0 } else { dest_ub + 1 }..jump_lb].to_string();
        println!("comp: {comp}");
        check!(comp, COMP_MAP);
        let jump = instruction[if jump_lb == instruction.len() {
            instruction.len()
        } else {
            jump_lb + 1
        }..instruction.len()]
            .to_string();
        println!("jump: {jump}");
        check!(jump, JUMP_MAP);
        Ok(Instruction::C(dest, comp, jump))
    }

    pub fn encode(&self) -> String {
        match self {
            Instruction::A(val) => {
                format!("0{}", Self::value_to_bin_string(*val))
            }
            Instruction::C(dest, comp, jump) => {
                format!(
                    "111{}{}{}",
                    COMP_MAP.get(&comp.as_str()).unwrap(),
                    DEST_MAP.get(&dest.as_str()).unwrap(),
                    JUMP_MAP.get(&jump.as_str()).unwrap(),
                )
            }
        }
    }

    fn value_to_bin_string(val: u16) -> String {
        let mut encoded = String::new();
        for i in (0..15).rev() {
            encoded.push(if val & (1 << i) == (1 << i) { '1' } else { '0' });
        }
        encoded
    }
}
