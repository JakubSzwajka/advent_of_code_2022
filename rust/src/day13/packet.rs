use anyhow::{ensure, Error};
use itertools::EitherOrBoth;
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{char, digit0, space0};
use nom::combinator::{map, map_res};
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;
use std::cmp::Ordering;
use std::str::FromStr;

type Int = i32;

#[derive(Debug, Eq, Clone)]
pub enum Packet {
    List(Vec<Packet>),
    Int(Int),
}

impl Packet {
    pub fn parse(s: &str) -> IResult<&str, Packet> {
        // try one of parsers
        alt((Self::parse_int, Self::parse_list))(s)
    }

    fn parse_int(s: &str) -> IResult<&str, Packet> {
        map(map_res(digit0, |s: &str| s.parse()), Packet::Int)(s)
    }

    fn parse_list(s: &str) -> IResult<&str, Packet> {
        map(
            delimited(
                char('['),
                separated_list0(delimited(space0, char(','), space0), Packet::parse),
                char(']'),
            ),
            Packet::List,
        )(s)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

fn cmp_packet_list(a: &[Packet], b: &[Packet]) -> std::cmp::Ordering {
    for pair in a.iter().zip_longest(b.iter()) {
        match pair {
            EitherOrBoth::Both(a, b) => match a.cmp(b) {
                r @ (Ordering::Greater | Ordering::Less) => return r,
                Ordering::Equal => (),
            },
            EitherOrBoth::Left(_) => return Ordering::Greater,
            EitherOrBoth::Right(_) => return Ordering::Less,
        };
    }
    Ordering::Equal
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match [self, other] {
            [Packet::Int(a), Packet::Int(b)] => a.cmp(b),
            [Packet::List(a), Packet::List(b)] => cmp_packet_list(a, b),
            [Packet::Int(a), Packet::List(b)] => cmp_packet_list(&[Packet::Int(*a)], b),
            [Packet::List(a), Packet::Int(b)] => cmp_packet_list(a, &[Packet::Int(*b)]),
        }
    }
}

impl FromStr for Packet {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rem, packet) = Packet::parse(s).map_err(|e| e.to_owned())?;
        ensure!(rem.len() == 0);
        Ok(packet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};

    #[test]
    fn test_parse() -> Result<()> {
        assert_eq!(Packet::Int(1), Packet::parse("1")?.1);
        assert_eq!(
            Packet::List(vec![Packet::Int(1), Packet::Int(1), Packet::Int(3)]),
            Packet::parse("[1,1,3]")?.1
        );
        assert_eq!(
            Packet::List(vec![
                Packet::Int(1),
                Packet::List(vec![Packet::Int(1), Packet::Int(2)]),
                Packet::Int(3)
            ]),
            Packet::parse("[1,[1,2],3]")?.1
        );
        Ok(())
    }

    #[test]
    fn test_compare() -> Result<()> {
        assert_eq!(true, Packet::Int(10) > Packet::Int(2));
        assert_eq!(true, Packet::Int(10) >= Packet::Int(2));
        assert_eq!(true, Packet::Int(10) >= Packet::Int(10));
        assert_eq!(true, Packet::Int(10) == Packet::Int(10));
        assert_eq!(true, Packet::Int(10) <= Packet::Int(10));
        assert_eq!(true, Packet::Int(1) <= Packet::Int(10));
        assert_eq!(true, Packet::Int(1) < Packet::Int(10));

        assert_eq!(
            true,
            Packet::List(vec![Packet::Int(1)]) == Packet::List(vec![Packet::Int(1)])
        );
        assert_eq!(
            true,
            Packet::List(vec![Packet::Int(2)]) > Packet::List(vec![Packet::Int(1)])
        );
        assert_eq!(
            true,
            Packet::List(vec![Packet::Int(2)]) < Packet::List(vec![Packet::Int(10)])
        );
        Ok(())
    }
    #[test]
    fn test_compare_use_cases() -> Result<()> {
        // - Compare [1,1,3,1,1] vs [1,1,5,1,1]
        assert_eq!(
            true,
            Packet::parse("[1,1,3,1,1]")?.1 < Packet::parse("[1,1,5,1,1]")?.1
        );
        // - Compare [[1],[2,3,4]] vs [[1],4]
        assert_eq!(
            true,
            Packet::parse("[[1],[2,3,4]]")?.1 < Packet::parse("[[1],4]")?.1
        );
        // - Compare [9] vs [[8,7,6]]
        assert_eq!(
            true,
            Packet::parse("[9]")?.1 > Packet::parse("[[8,7,6]]")?.1
        );
        // - Compare [[4,4],4,4] vs [[4,4],4,4,4]
        assert_eq!(
            true,
            Packet::parse("[[4,4],4,4]")?.1 < Packet::parse("[[4,4],4,4,4]")?.1
        );
        // - Compare [7,7,7,7] vs [7,7,7]
        assert_eq!(
            true,
            Packet::parse("[7,7,7,7]")?.1 > Packet::parse("[7,7,7]")?.1
        );
        // - Compare [] vs [3]
        assert_eq!(true, Packet::parse("[]")?.1 < Packet::parse("[3]")?.1);
        // - Compare [[[]]] vs [[]]
        assert_eq!(true, Packet::parse("[[[]]]")?.1 > Packet::parse("[[]]")?.1);

        // - Compare [1,[2,[3,[4,[5,6,7]]]],8,9] vs [1,[2,[3,[4,[5,6,0]]]],8,9]
        assert_eq!(
            true,
            Packet::parse("[1,[2,[3,[4,[5,6,7]]]],8,9]")?.1
                > Packet::parse("[1,[2,[3,[4,[5,6,0]]]],8,9]")?.1
        );

        Ok(())
    }
}
