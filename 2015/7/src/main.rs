use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Clone, Copy, Debug)]
enum NumOrVar<'a> {
    Num(i16),
    Var(&'a str),
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
        value: NumOrVar<'a>,
        var: &'a str,
    },
}

fn evaluate<'a>(
    search: &str,
    instructions: &'a [Instruction],
    cache: &mut HashMap<&'a str, Option<i16>>,
) -> i16 {
    let mut ending_value = 0;
    for instruction in instructions {
        match instruction {
            Instruction::Assignment { var, value } => match value {
                NumOrVar::Var(s) => {
                    cache.entry(var).or_insert(None);
                    cache.entry(s).or_insert(None);
                    if var == &search {
                        let v1;
                        if let Some(i) = cache[s] {
                            v1 = i;
                        } else {
                            v1 = evaluate(var, instructions, cache)
                        }
                        ending_value = v1
                    }
                }
                NumOrVar::Num(i) => {
                    ending_value = *i;
                    cache.insert(var, Some(*i));
                }
            },
            Instruction::And { opnd0, opnd1, ans } => {
                if ans == &search {
                    let v1;
                    let v2;
                    if let Some(i) = cache[opnd0] {
                        v1 = i;
                    } else {
                        v1 = evaluate(opnd0, instructions, cache)
                    }

                    if let Some(i) = cache[opnd1] {
                        v2 = i;
                    } else {
                        v2 = evaluate(opnd1, instructions, cache)
                    }
                    ending_value = v1 & v2;
                }
            }
            Instruction::Or { opnd0, opnd1, ans } => {
                if ans == &search {
                    let v1;
                    let v2;
                    if let Some(i) = cache[opnd0] {
                        v1 = i;
                    } else {
                        v1 = evaluate(opnd0, instructions, cache)
                    }

                    if let Some(i) = cache[opnd1] {
                        v2 = i;
                    } else {
                        v2 = evaluate(opnd1, instructions, cache)
                    }
                    ending_value = v1 | v2;
                }
            }
            Instruction::Not { opnd0, ans } => {
                if ans == &search {
                    let v1;
                    if let Some(i) = cache[opnd0] {
                        v1 = i;
                    } else {
                        v1 = evaluate(opnd0, instructions, cache)
                    }

                    ending_value = !v1;
                }
            }
            Instruction::RShift { opnd0, shift, ans } => {
                if ans == &search {
                    let v1;
                    if let Some(i) = cache[opnd0] {
                        v1 = i;
                    } else {
                        v1 = evaluate(opnd0, instructions, cache)
                    }

                    ending_value = v1 >> shift;
                }
            }
            Instruction::LShift { opnd0, shift, ans } => {
                if ans == &search {
                    let v1;
                    if let Some(i) = cache[opnd0] {
                        v1 = i;
                    } else {
                        v1 = evaluate(opnd0, instructions, cache)
                    }

                    ending_value = v1 << shift;
                }
            }
        }
    }
    println!("{}", ending_value);
    ending_value
}

fn main() {
    let file = read_to_string("input.txt");
    let mut cache: Box<HashMap<&str, Option<i16>>> = Box::default();

    if let Ok(file) = file {
        let mut instructions: Box<Vec<Instruction>> = Box::default();

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
                        value: NumOrVar::Num(i),
                        var: split[2],
                    }
                } else {
                    instruction = Instruction::Assignment {
                        value: NumOrVar::Var(split[0]),
                        var: split[2],
                    }
                }
            }

            instructions.push(instruction);
        }

        let answer = Box::new(evaluate("a", &instructions, &mut cache));
        println!("{}", answer);
    } else {
        println!("input.txt not found")
    }
}
