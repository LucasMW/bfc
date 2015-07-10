use std::fmt;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Instruction {
    Increment(i32),
    Set(i32),
    PointerIncrement(i32),
    Read,
    Write,
    Loop(Vec<Instruction>)
}

fn fmt_with_indent(instr: &Instruction, indent: i32, f: &mut fmt::Formatter) {
    for _ in 0..indent {
        let _ = write!(f, "  ");
    }
    
    match instr {
        &Instruction::Loop(ref loop_body) => {
            let _ = write!(f, "Loop");

            for loop_instr in loop_body.iter() {
                let _ = write!(f, "\n");
                fmt_with_indent(loop_instr, indent + 1, f);
            }
        }
        instr @ _ => {
            let _ = write!(f, "{:?}", instr);
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_with_indent(self, 0, f);
        Ok(())
    }
}

/// Given a string of BF source code, parse and return our BF IR
/// representation.
pub fn parse(source: &str) -> Vec<Instruction> {
    parse_between(source, 0, source.chars().count())
}

/// Parse BF source code from index `start` up to (but excluding)
/// index `end`.
fn parse_between(source: &str, start: usize, end: usize) -> Vec<Instruction> {
    let chars: Vec<_> = source.chars().collect();
    assert!(start <= end);
    assert!(end <= chars.len());

    let mut instructions = Vec::new();
    let mut index = start;
    
    while index < end {
        match chars[index] {
            '+' => 
                instructions.push(Instruction::Increment(1)),
            '-' => 
                instructions.push(Instruction::Increment(-1)),
            '>' => 
                instructions.push(Instruction::PointerIncrement(1)),
            '<' => 
                instructions.push(Instruction::PointerIncrement(-1)),
            ',' => 
                instructions.push(Instruction::Read),
            '.' => 
                instructions.push(Instruction::Write),
            '[' => {
                // TODO: handle unbalanced parens gracefully.
                let close_index = find_close(source, index).unwrap();
                let loop_body = parse_between(source, index + 1, close_index);
                instructions.push(Instruction::Loop(loop_body));

                index = close_index;
            }
            _ => ()
        }

        index += 1;
    }

    instructions
}

/// Find the index of the `]` that matches the `[` at `open_index`.
fn find_close(source: &str, open_index: usize) -> Option<usize> {
    assert_eq!(source.chars().nth(open_index), Some('['));

    let mut nesting_depth = 0;
    for (index, c) in source.chars().enumerate() {
        if index < open_index {
            continue;
        }

        match c {
            '[' => nesting_depth += 1,
            ']' => nesting_depth -= 1,
            _ => ()
        }

        if nesting_depth == 0 {
            return Some(index)
        }
    }
    None
}

#[test]
fn parse_increment() {
    assert_eq!(parse("+"), [Instruction::Increment(1)]);
    assert_eq!(parse("++"), [Instruction::Increment(1),
                            Instruction::Increment(1)]);
}

#[test]
fn parse_decrement() {
    assert_eq!(parse("-"), [Instruction::Increment(-1)]);
}

#[test]
fn parse_pointer_increment() {
    assert_eq!(parse(">"), [Instruction::PointerIncrement(1)]);
}

#[test]
fn parse_pointer_decrement() {
    assert_eq!(parse("<"), [Instruction::PointerIncrement(-1)]);
}

#[test]
fn parse_read() {
    assert_eq!(parse(","), [Instruction::Read]);
}

#[test]
fn parse_write() {
    assert_eq!(parse("."), [Instruction::Write]);
}

#[test]
fn parse_empty_loop() {
    let expected = [Instruction::Loop(vec![])];
    assert_eq!(parse("[]"), expected);
}

#[test]
fn parse_simple_loop() {
    let loop_body = vec![Instruction::Increment(1)];
    let expected = [Instruction::Loop(loop_body)];
    assert_eq!(parse("[+]"), expected);
}

#[test]
fn parse_complex_loop() {
    let loop_body = vec![Instruction::Read, Instruction::Increment(1)];
    let expected = [Instruction::Write,
                    Instruction::Loop(loop_body),
                    Instruction::Increment(-1)];
    assert_eq!(parse(".[,+]-"), expected);
}

#[test]
fn parse_comment() {
    assert_eq!(parse("foo! "), []);
}
