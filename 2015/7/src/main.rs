use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Clone, Copy, Debug)]
enum NumOrVar<'a> {
    Num(u16),
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
        shift: u16,
        ans: &'a str,
    },
    RShift {
        opnd0: &'a str,
        shift: u16,
        ans: &'a str,
    },
    Assignment {
        value: NumOrVar<'a>,
        var: &'a str,
    },
}

fn evaluate<'a>(
    search: &'a str,
    instructions: &'a [Instruction],
    cache: &mut HashMap<&'a str, Option<u16>>,
) -> u16 {
    let mut ending_value = 0;
    for instruction in instructions {
        match instruction {
            Instruction::Assignment { var, value } => {
                if var == &search {
                    match value {
                        NumOrVar::Var(s) => {
                            let entry = cache.entry(s);
                            entry.or_insert(None);

                            let s = cache[s];

                            if let Some(s) = s {
                                ending_value = s;
                            } else {
                                ending_value = evaluate(search, instructions, cache);
                            }
                        }
                        NumOrVar::Num(i) => {
                            ending_value = *i;
                        }
                    }
                }
            }

            Instruction::And { opnd0, opnd1, ans } => {
                if ans == &search {
                    let entry1 = cache.entry(opnd0);
                    entry1.or_insert(None);

                    let entry2 = cache.entry(opnd1);
                    entry2.or_insert(None);

                    let v1: u16;
                    let v2: u16;

                    if let Some(o0) = cache[opnd0] {
                        v1 = o0;
                    } else {
                        v1 = evaluate(opnd0, instructions, cache);
                    }

                    if let Some(o1) = cache[opnd1] {
                        v2 = o1;
                    } else {
                        v2 = evaluate(opnd1, instructions, cache);
                    }

                    ending_value = v1 & v2;
                }
            }
            Instruction::Or { opnd0, opnd1, ans } => {
                if ans == &search {
                    let entry1 = cache.entry(opnd0);
                    entry1.or_insert(None);

                    let entry2 = cache.entry(opnd1);
                    entry2.or_insert(None);

                    let v1: u16;
                    let v2: u16;

                    if let Some(o0) = cache[opnd0] {
                        v1 = o0;
                    } else {
                        v1 = evaluate(opnd0, instructions, cache);
                    }

                    if let Some(o1) = cache[opnd1] {
                        v2 = o1;
                    } else {
                        v2 = evaluate(opnd1, instructions, cache);
                    }

                    ending_value = v1 | v2;
                }
            }
            Instruction::Not { opnd0, ans } => {
                if ans == &search {
                    let entry1 = cache.entry(opnd0);
                    entry1.or_insert(None);

                    let v1: u16;

                    if let Some(o0) = cache[opnd0] {
                        v1 = o0;
                    } else {
                        v1 = evaluate(opnd0, instructions, cache);
                    }

                    ending_value = !v1;
                }
            }
            Instruction::RShift { opnd0, shift, ans } => {
                if ans == &search {
                    let entry1 = cache.entry(opnd0);
                    entry1.or_insert(None);

                    let v1: u16;

                    if let Some(o0) = cache[opnd0] {
                        v1 = o0;
                    } else {
                        v1 = evaluate(opnd0, instructions, cache);
                    }

                    ending_value = v1 >> shift;
                }
            }
            Instruction::LShift { opnd0, shift, ans } => {
                if ans == &search {
                    let entry1 = cache.entry(opnd0);
                    entry1.or_insert(None);

                    let v1: u16;

                    if let Some(o0) = cache[opnd0] {
                        v1 = o0;
                    } else {
                        v1 = evaluate(opnd0, instructions, cache);
                    }

                    ending_value = v1 << shift;
                }
            }
        }
    }
    cache.insert(search, Some(ending_value));
    ending_value
}

fn main() {
    let file = read_to_string("input.txt");
    let mut cache: HashMap<&str, Option<u16>> = HashMap::new();

    if let Ok(file) = file {
        let mut instructions: Vec<Instruction> = Vec::new();

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
                if let Ok(i) = split[0].parse::<u16>() {
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

        let answer = evaluate("a", &instructions, &mut cache);
        println!("{}", answer);
    } else {
        println!("input.txt not found")
    }
}
