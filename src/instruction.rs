use nom::{branch::alt, bytes::complete::tag, character::complete::newline, error::Error, multi::separated_list1, sequence::{preceded, terminated}, *};

#[derive(Clone, Debug)]
pub enum Instruction {
    Forwards,
    Left,
    Right,
    Backwards
}

pub fn parse_code<'a>(i: &'a str) -> Result<Vec<Instruction>, Err<Error<&str>>> {
    separated_list1(newline, parse_statement)(i).map(|x| x.1)
}

fn parse_statement<'a>(i: &'a str) -> IResult<&'a str, Instruction> {
    terminated(parse_instruction, tag(";"))(i)
}

fn parse_instruction<'a>(i: &'a str) -> IResult<&'a str, Instruction> {
    alt((
        preceded(tag("move "), alt((
            tag("forwards").map(|_| Instruction::Forwards),
            tag("backwards").map(|_| Instruction::Backwards),
            tag("left").map(|_| Instruction::Left),
            tag("right").map(|_| Instruction::Right),
        ))),
    ))(i)
}

#[cfg(test)]
mod tests {
    use super::{parse_instruction, parse_statement};

    #[test]
    fn move_instruction1() {
        parse_instruction("move forwards").unwrap();
        parse_instruction("move backwards").unwrap();
        parse_instruction("move left").unwrap();
        parse_instruction("move right").unwrap();
    }

    #[test]
    fn move_instruction_wrong() {
        parse_instruction("move diagonal").unwrap_err();
        parse_instruction("move  right").unwrap_err();
        parse_instruction("mov back").unwrap_err();
        parse_instruction("move ").unwrap_err();
        parse_instruction(" right").unwrap_err();
    }

    #[test]
    fn statement() {
        parse_statement("move right;").unwrap();
        parse_statement("move left;").unwrap();
    }

    #[test]
    fn statement_wrong() {
        parse_statement("mo forwards;").unwrap_err();
        parse_statement("move backwards").unwrap_err();
        parse_statement(";").unwrap_err();
    }
}
