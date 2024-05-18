use nom::{branch::alt, bytes::complete::tag, character::complete::multispace0, combinator::eof, error::{Error, ErrorKind}, sequence::{preceded, terminated, tuple}, *};

use crate::grid::{Direction, Tile};

#[derive(Clone, Debug)]
enum PreInstruction {
    Move(Direction),
    Loop(Vec<PreInstruction>),
    If((Conditional, Vec<PreInstruction>))
}

pub fn compile(code: &str) -> Result<Vec<Instruction>, Err<Error<&str>>> {
    let (_, (preinstructions, ended_curly)) = parse_code(code)?;
    if ended_curly {
        return Err(Err::Error(Error::new(code, ErrorKind::Tag)))
    }
    Ok(flatten(0, preinstructions).1)
}

/// if bool is true, ended with curly
fn parse_code<'a>(i: &'a str) -> IResult<&'a str, (Vec<PreInstruction>, bool)> {
    enum AltReturn {
        Statement(PreInstruction),
        Termination { curly: bool }
    }
    use AltReturn::*;
    let (mut rem, _) = multispace0(i)?;
    let mut pres = vec![];
    loop {
        let (r, pre) = alt((
            tag("}").map(|_| Termination { curly: true }),
            parse_statement.map(|x| Statement(x)),
            eof.map(|_| Termination { curly: false })
        ))(rem)?;
        rem = r;

        match pre {
            Statement(pre) => pres.push(pre),
            Termination { curly } => { break Ok((rem, (pres, curly))) }
        }

        let (r, _) = multispace0(rem)?;
        rem = r;
    }
}

fn parse_code_block<'a>(i: &'a str) -> IResult<&'a str, Vec<PreInstruction>> {
    let (rem, _) = tuple((tag("{"), multispace0))(i)?;
    let (rem, (code, ends_curly)) = parse_code(rem)?;
    if ends_curly {
        Ok((rem, code))
    } else {
        return Err(Err::Error(Error::new(rem, ErrorKind::Eof)))
    }
}

fn flatten(start: usize, pres: Vec<PreInstruction>) -> (usize, Vec<Instruction>) {
    let mut loc = start;
    let mut instructions = vec![];
    for pre in pres {
        match pre {
            PreInstruction::Move(dir) => instructions.push(Instruction::Move(dir)),
            PreInstruction::Loop(nested) => {
                let (len, mut flattened) = flatten(loc, nested);
                flattened.push(Instruction::Goto(loc));
                instructions.append(&mut flattened);
                loc += len;
            }
            PreInstruction::If((cond, nested)) => {
                let (len, mut flattened) = flatten(loc + 1, nested);
                instructions.push(Instruction::BranchIfNot { cond, dest: loc + len + 1 });
                instructions.append(&mut flattened);
                loc += len;
            }
        };
        loc += 1;
    }
    (loc - start, instructions)
}

fn parse_statement<'a>(i: &'a str) -> IResult<&'a str, PreInstruction> {
    terminated(alt((
        parse_loop,
        parse_if,
        terminated(parse_expression, tag(";"))
    )), multispace0)(i)
}

fn parse_expression<'a>(i: &'a str) -> IResult<&'a str, PreInstruction> {
    preceded(tag("move "), parse_direction.map(|x| PreInstruction::Move(x)))(i)
}

fn parse_direction<'a>(i: &'a str) -> IResult<&'a str, Direction> {
    alt((
        tag("up").map(|_| Direction::Up),
        tag("down").map(|_| Direction::Down),
        tag("left").map(|_| Direction::Left),
        tag("right").map(|_| Direction::Right)
    ))(i)
}

fn parse_tile<'a>(i: &'a str) -> IResult<&'a str, Tile> {
    alt((
        tag("rock").map(|_| Tile::Rock),
        tag("empty").map(|_| Tile::Empty),
        tag("finish").map(|_| Tile::Finish),
    ))(i)
}

fn parse_loop<'a>(i: &'a str) -> IResult<&'a str, PreInstruction> {
    let (r, _) = tuple((tag("loop"), multispace0))(i)?;
    let (rem, code) = parse_code_block(r)?;
    Ok((rem, PreInstruction::Loop(code)))
}

fn parse_if<'a>(i: &'a str) -> IResult<&'a str, PreInstruction> {
    let (r, (_, _, dir, _, tile, _)) = tuple((tag("if"), multispace0, parse_direction, multispace0, parse_tile, multispace0))(i)?;
    let (rem, code) = parse_code_block(r)?;
    Ok((rem, PreInstruction::If((Conditional::TileCheck { dir, tile }, code))))
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Move(Direction),
    Goto(usize),
    BranchIfNot {
        cond: Conditional,
        dest: usize
    }
}

#[derive(Clone, Debug)]
pub enum Conditional {
    TileCheck {
        dir: Direction,
        tile: Tile
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_expression, parse_statement};

    #[test]
    fn move_expression1() {
        parse_expression("move up").unwrap();
        parse_expression("move down").unwrap();
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
        parse_statement("mo up;").unwrap_err();
        parse_statement("move down").unwrap_err();
        parse_statement(";").unwrap_err();
    }
}
