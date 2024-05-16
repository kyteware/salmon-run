use nom::{branch::alt, bytes::complete::tag, character::complete::{multispace0, newline}, error::Error, multi::{many0, separated_list1}, sequence::{delimited, preceded, terminated, tuple}, *};

#[derive(Clone, Debug)]
enum PreInstruction {
    Forwards,
    Left,
    Right,
    Backwards,
    Loop(Vec<PreInstruction>)
}

pub fn compile(code: &str) -> Result<Vec<Instruction>, Err<Error<&str>>> {
    let (_, preinstructions) = parse_code(code)?;
    Ok(flatten(preinstructions))
}

fn parse_code<'a>(i: &'a str) -> IResult<&'a str, Vec<PreInstruction>> {
   preceded(multispace0, many0(parse_statement))(i)
}

fn flatten(pres: Vec<PreInstruction>) -> Vec<Instruction> {
    let mut instructions = vec![];
    for pre in pres {
        match pre {
            PreInstruction::Forwards => instructions.push(Instruction::Forwards),
            PreInstruction::Left => instructions.push(Instruction::Left),
            PreInstruction::Right => instructions.push(Instruction::Right),
            PreInstruction::Backwards => instructions.push(Instruction::Backwards),
            PreInstruction::Loop(nested) => instructions.append(&mut flatten(nested))
        };
    }
    instructions
}

fn parse_statement<'a>(i: &'a str) -> IResult<&'a str, PreInstruction> {
    terminated(alt((
        parse_loop,
        terminated(parse_expression, tag(";"))
    )), multispace0)(i)
}

fn parse_expression<'a>(i: &'a str) -> IResult<&'a str, PreInstruction> {
    alt((
        preceded(tag("move "), alt((
            tag("forwards").map(|_| PreInstruction::Forwards),
            tag("backwards").map(|_| PreInstruction::Backwards),
            tag("left").map(|_| PreInstruction::Left),
            tag("right").map(|_| PreInstruction::Right),
        ))),
    ))(i)
}

fn parse_loop<'a>(i: &'a str) -> IResult<&'a str, PreInstruction> {
    delimited(tuple((tag("loop"), multispace0, tag("{"))), parse_code.map(|x| PreInstruction::Loop(x)), tag("}"))(i)
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Forwards,
    Backwards,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::{parse_expression, parse_statement};

    #[test]
    fn move_expression1() {
        parse_expression("move forwards").unwrap();
        parse_expression("move backwards").unwrap();
        parse_expression("move left").unwrap();
        parse_expression("move right").unwrap();
    }

    #[test]
    fn move_expression() {
        parse_expression("move diagonal").unwrap_err();
        parse_expression("move  right").unwrap_err();
        parse_expression("mov back").unwrap_err();
        parse_expression("move ").unwrap_err();
        parse_expression(" right").unwrap_err();
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
