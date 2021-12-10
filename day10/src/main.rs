fn main() -> IResult<()> {
    let input = include_str!("../input/example");
    let parsed = parse(input)?;
    let solved = solve(&parsed);
    println!("{}", solved);
    Ok(())
}

type IResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn parse(ipt: &str) -> IResult<Vec<Vec<Delimiter>>> {
    let a: Result<Vec<_>, _> = ipt
        .lines()
        .map(|line| line.chars().map(|c| c.try_into()).collect())
        .collect();
    let a = a?;
    Ok(a)
}

fn solve(data: &[Vec<Delimiter>]) -> i64 {
    let mut stack = Vec::with_capacity(data[0].len());
    let mut result = 0;
    for row in data {
        for del in row {
            use Delimiter::*;
            match del {
                LParen | LBracket | LBrace | LAngle => stack.push(del),
                x @ (RParen | RBracket | RBrace | RAngle) => {
                    if let Some(last) = stack.pop() {
                        let expected = last.get_rhs().expect("unexpected delimiter");
                        if *x != expected {
                            result += *x as i64;
                        }
                    } else {
                        unreachable!()
                    }
                }
            }
        }
    }
    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Delimiter {
    LParen,
    RParen = 3,
    LBracket,
    RBracket = 57,
    LBrace,
    RBrace = 1197,
    LAngle,
    RAngle = 25137,
}

impl Delimiter {
    fn get_rhs(&self) -> IResult<Delimiter> {
        use Delimiter::*;
        match self {
            LParen => Ok(RParen),
            LBracket => Ok(RBracket),
            LBrace => Ok(RBrace),
            LAngle => Ok(RAngle),
            _ => Err(format!("{:?} has no rhs", self).into()),
        }
    }
}

impl TryFrom<char> for Delimiter {
    type Error = &'static str;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '(' => Ok(Delimiter::LParen),
            ')' => Ok(Delimiter::RParen),
            '[' => Ok(Delimiter::LBracket),
            ']' => Ok(Delimiter::RBracket),
            '{' => Ok(Delimiter::LBrace),
            '}' => Ok(Delimiter::RBrace),
            '<' => Ok(Delimiter::LAngle),
            '>' => Ok(Delimiter::RAngle),
            _ => Err("invalid delimiter"),
        }
    }
}
