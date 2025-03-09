use std::{collections::HashMap, fs::read_to_string};

#[derive(Clone, Copy, Debug)]
enum NumOrValue<'a> {
    Num(i16),
    Value(&'a str),
}

#[derive(Clone, Copy, Debug)]
enum Instruction<'a> {
    And {
        opnd0: &'a str,
        opnd1: &'a str,
        ans: &'a str,
    },
    Not {
        opnd0: &'a str,
        ans: &'a str,
    },
    Or {
        opnd0: &'a str,
        opnd1: &'a str,
        ans: &'a str,
    },
    LShift {
        opnd0: &'a str,
        shift: i16,
        ans: &'a str,
    },
    RShift {
        opnd0: &'a str,
        shift: i16,
        ans: &'a str,
    },
    Assignment {
        value: NumOrValue<'a>,
        var: &'a str,
    },
}

impl<'a> Instruction<'a> {
    fn get_dependencies(&self) -> Vec<&'a str> {
        let mut dependencies = Vec::new();

        match *self {
            Instruction::And {
                opnd0,
                opnd1,
                ans: _,
            } => {
                let _ = &dependencies.push(opnd0);
                let _ = &dependencies.push(opnd1);
            }
            Instruction::Or {
                opnd0,
                opnd1,
                ans: _,
            } => {
                let _ = &dependencies.push(opnd0);
                let _ = &dependencies.push(opnd1);
            }
            Instruction::Not { opnd0, ans: _ } => {
                let _ = &dependencies.push(opnd0);
            }
            Instruction::LShift {
                opnd0,
                shift: _,
                ans: _,
            } => {
                let _ = &dependencies.push(opnd0);
            }
            Instruction::RShift {
                opnd0,
                shift: _,
                ans: _,
            } => {
                let _ = &dependencies.push(opnd0);
            }
            Instruction::Assignment { var: _, value } => match value {
                NumOrValue::Value(s) => {
                    let _ = &dependencies.push(s);
                }
                NumOrValue::Num(_) => {
                    let _ = &dependencies;
                }
            },
        }

        dependencies
    }

    fn get_declared(&self) -> &'a str {
        match *self {
            Instruction::And {
                opnd0: _,
                opnd1: _,
                ans,
            } => ans,
            Instruction::Or {
                opnd0: _,
                opnd1: _,
                ans,
            } => ans,
            Instruction::Not { opnd0: _, ans } => ans,
            Instruction::LShift {
                opnd0: _,
                shift: _,
                ans,
            } => ans,
            Instruction::RShift {
                opnd0: _,
                shift: _,
                ans,
            } => ans,
            Instruction::Assignment { var, value: _ } => var,
        }
    }
}

fn main() {
    let file = read_to_string("input.txt");

    if let Ok(file) = file {
        let mut data: HashMap<&str, i16> = HashMap::new();
        let mut sorted_instructions: Vec<Instruction> = Vec::new();

        for line in file.lines() {
            let instruction: Instruction;
            let split: Vec<&str> = line.split(" ").collect();

            if line.contains("AND") {
                // x AND y -> z
                // 0 1   2 3  4
                instruction = Instruction::And {
                    opnd0: split[0],
                    opnd1: split[2],
                    ans: split[4],
                }
            } else if line.contains("OR") {
                // x OR y -> z
                // 0 1  2 3  4
                instruction = Instruction::Or {
                    opnd0: split[0],
                    opnd1: split[2],
                    ans: split[4],
                }
            } else if line.contains("NOT") {
                // NOT x -> y
                // 0  1  2  3
                instruction = Instruction::Not {
                    opnd0: split[1],
                    ans: split[3],
                }
            } else if line.contains("LSHIFT") {
                // x LSHIFT 2 -> y
                // 0 1      2 3  4
                instruction = Instruction::LShift {
                    opnd0: split[0],
                    shift: split[2].parse().unwrap(),
                    ans: split[4],
                }
            } else if line.contains("RSHIFT") {
                // x LSHIFT 2 -> y
                // 0 1      2 3  4
                instruction = Instruction::RShift {
                    opnd0: split[0],
                    shift: split[2].parse().unwrap(),
                    ans: split[4],
                }
            } else {
                // 123 -> x
                // 0   1  2
                if let Ok(i) = split[0].parse::<i16>() {
                    instruction = Instruction::Assignment {
                        value: NumOrValue::Num(i),
                        var: split[2],
                    }
                } else {
                    instruction = Instruction::Assignment {
                        value: NumOrValue::Value(split[0]),
                        var: split[2],
                    }
                }
            }

            let mut index = 0;

            for sorted_instruction in &sorted_instructions {
                let dependencies = instruction.get_dependencies();
                let mut declared: Vec<&str> = Vec::new();
                let curr_declared = sorted_instruction.get_declared();

                index += 1;

                if dependencies.contains(&curr_declared) {
                    declared.push(curr_declared);
                }
                if declared.len() == dependencies.len() {
                    break;
                }
            }

            sorted_instructions.insert(index, instruction);
        }
        println!("{:#?}", sorted_instructions);
        for instruction in sorted_instructions {
            println!("{:?}", instruction);

            match instruction {
                Instruction::Assignment { var, value } => match value {
                    NumOrValue::Value(s) => data.insert(var, data[s]),
                    NumOrValue::Num(i) => data.insert(var, i),
                },
                Instruction::And { opnd0, opnd1, ans } => {
                    data.insert(ans, data[opnd0] & data[opnd1])
                }
                Instruction::Or { opnd0, opnd1, ans } => {
                    data.insert(ans, data[opnd0] | data[opnd1])
                }
                Instruction::Not { opnd0, ans } => data.insert(ans, !data[opnd0]),
                Instruction::RShift { opnd0, shift, ans } => data.insert(ans, data[opnd0] >> shift),
                Instruction::LShift { opnd0, shift, ans } => data.insert(ans, data[opnd0] << shift),
            };
        }

        println!("{:?}", data);
    } else {
        println!("input.txt not found")
    }
}
