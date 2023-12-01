//! https://adventofcode.com/2016/day/10

#![allow(dead_code, unused_imports, unused_variables)]

use std::cmp::max;
use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, not_line_ending, one_of};
use nom::combinator::{all_consuming, map, map_res, recognize};
use nom::multi::{many0, many1};
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

const DAY: &str = "1610";

type Chip = usize;
type BotId = usize;
type OutputId = usize;

#[derive(Default, Debug)]
struct Bot {
    /// 0 to 2 values that are already loaded into this bot.
    values: Vec<Chip>,

    low: Option<Dest>,
    high: Option<Dest>,
}

impl Bot {
    fn take(&mut self, value: Chip) {
        assert!(self.values.len() < 2, "bot is already full");
        self.values.push(value)
    }

    fn is_ready(&self) -> bool {
        self.values.len() == 2
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Dest {
    Bot(BotId),
    Output(OutputId),
}

impl Dest {
    fn bot_id(&self) -> Option<BotId> {
        if let Dest::Bot(bot_id) = self {
            Some(*bot_id)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Inst {
    Load { chip: Chip, bot: BotId },
    Give { bot: BotId, low: Dest, high: Dest },
}

impl Inst {
    fn max_bot_id(&self) -> BotId {
        match self {
            Inst::Load { bot, .. } => *bot,
            Inst::Give { bot, low, high } => [low, high]
                .iter()
                .flat_map(|d| d.bot_id())
                .max()
                .map_or(*bot, |a| max(a, *bot)),
        }
    }
}

fn parse_int(input: &str) -> IResult<&str, usize> {
    map(recognize(many1(one_of("0123456789"))), |s: &str| {
        s.parse().unwrap()
    })(input)
}

fn parse_dest(input: &str) -> IResult<&str, Dest> {
    alt((
        preceded(tag("output "), map(parse_int, Dest::Output)),
        preceded(tag("bot "), map(parse_int, Dest::Bot)),
    ))(input)
}

fn parse_line(input: &str) -> IResult<&str, Inst> {
    // dbg!(&input);
    alt((
        map(
            tuple((tag("value "), parse_int, tag(" goes to bot "), parse_int)),
            |(_, value, _, bot)| Inst::Load { chip: value, bot },
        ),
        map(
            tuple((
                tag("bot "),
                parse_int,
                tag(" gives low to "),
                parse_dest,
                tag(" and high to "),
                parse_dest,
            )),
            |(_, bot, _, low, _, high)| Inst::Give { bot, low, high },
        ),
    ))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Inst>> {
    // all_consuming(
    many1(terminated(parse_line, line_ending))(input)
}

fn max_bot_number(insts: &[Inst]) -> BotId {
    insts
        .iter()
        .map(|inst| inst.max_bot_id())
        .max()
        .expect("instructions not empty")
}

fn solve_type_a(input: &str) -> usize {
    // One complication is that some of the lines can't be immediately evaluated
    // when they're read, because the bot doesn't have enough inputs yet. It
    // seems like we need to keep them queued up per bot, and then when it does
    // get two inputs we can reevaluate them.
    let (rest, insts) = parse(input).expect("parse");
    debug_assert!(rest.is_empty());
    let n_bots = max_bot_number(&insts);
    let mut bots: Vec<Bot> = (0..=n_bots).map(|_| Bot::default()).collect();
    // First load all the chips, and queue up give instructions
    for inst in &insts {
        match inst {
            Inst::Load { chip: value, bot } => bots[*bot].take(*value),
            Inst::Give { bot, low, high } => {
                let bot = &mut bots[*bot];
                assert!(bot.low.is_none() && bot.high.is_none());
                bot.low = Some(low.clone());
                bot.high = Some(high.clone());
            }
        }
        // dbg!(&inst);
    }
    for bot in &bots {
        println!("{:?}", *bot);
    }
    // Find a robot that has two chips and instructions on what to do with them
    loop {
        for bot_id in 1..n_bots {
            if bots[bot_id].is_ready() {
                let bot = &bots[bot_id];
                println!("move on {}: {:?}", bot_id, bot);
                let lowv = *bot.values.iter().min().unwrap();
                let highv = *bot.values.iter().max().unwrap();
                if lowv == 17 && highv == 61 {
                    return bot_id;
                }
                let low = bot.low.as_ref().unwrap().clone();
                let high = bot.high.as_ref().unwrap().clone();
                if let Dest::Bot(lowbot) = low {
                    println!("give {} to {}", lowv, lowbot);
                    bots[lowbot].take(lowv)
                }
                if let Dest::Bot(highbot) = high {
                    println!("give {} to {}", highv, highbot);
                    bots[highbot].take(highv)
                }
                bots[bot_id].values.clear();
            }
        }
    }
}

fn solve_type_b(input: &str) -> usize {
    0
}

fn input() -> String {
    std::fs::read_to_string(format!("input/{}.txt", DAY)).unwrap()
}

fn solve_a() -> usize {
    solve_type_a(&input())
}

fn solve_b() -> usize {
    solve_type_b(&input())
}

fn main() {
    println!("{}a: {}", DAY, solve_a());
    println!("{}b: {}", DAY, solve_b());
}

#[cfg(test)]
mod test1610 {
    use super::*;

    #[test]
    fn parse_lines() {
        assert_eq!(
            parse_line("value 5 goes to bot 2\n"),
            Ok(("\n", Inst::Load { chip: 5, bot: 2 }))
        );
        assert_eq!(
            parse("value 5 goes to bot 2\nvalue 3000 goes to bot 112233\n"),
            Ok((
                "",
                vec![
                    Inst::Load { chip: 5, bot: 2 },
                    Inst::Load {
                        chip: 3000,
                        bot: 112233
                    }
                ]
            ))
        );
    }

    #[test]
    fn parse_give() {
        assert_eq!(
            parse_line("bot 42 gives low to output 0 and high to bot 90\n"),
            Ok((
                "\n",
                Inst::Give {
                    bot: 42,
                    low: Dest::Output(0),
                    high: Dest::Bot(90),
                }
            ))
        );
    }

    #[test]
    fn parse_all_input() {
        let input = input();
        let (rest, insts) = parse(&input).unwrap();
        assert!(rest.is_empty());
        assert_eq!(insts.len(), 231);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 27);
    }

    // #[test]
    // fn solution_b() {
    //     assert_eq!(solve_b(), 0);
    // }
}
