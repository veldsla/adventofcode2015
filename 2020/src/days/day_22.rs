use std::collections::{hash_map::DefaultHasher, HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use std::str;

use anyhow::{anyhow, Result};

use crate::Problem;
use crate::parsers::positive_integer;

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, multispace1},
    multi::separated_list1,
    combinator::all_consuming,
    sequence::{preceded, terminated, tuple},
    IResult
};

#[derive(Default)]
pub struct Solution {
    deck_1: Vec<u8>,
    deck_2: Vec<u8>,
}

fn parse(i: &str) -> IResult<&str, (Vec<u8>, Vec<u8>)> {
    let deck1 = preceded(terminated(tag("Player 1:"), line_ending),
            separated_list1(line_ending, positive_integer));
    let deck2 = preceded(terminated(tag("Player 2:"), line_ending),
            separated_list1(line_ending, positive_integer));

    all_consuming(tuple((terminated(deck1, multispace1), terminated(deck2, multispace1))))(i)
}

fn score(hand: &VecDeque<u8>) -> usize {
    hand.iter().rev().enumerate().map(|(i, &c)| (i+1) * c as usize).sum()
}

fn play(d1: &[u8], d2: &[u8]) -> usize {

    let mut player_1: VecDeque<u8> = d1.iter().copied().collect();
    let mut player_2: VecDeque<u8> = d2.iter().copied().collect();

    loop {
        match (player_1.pop_front(), player_2.pop_front()) {
            (Some(c1), Some(c2)) if c1 > c2 => {
                player_1.push_back(c1);
                player_1.push_back(c2);
            },
            (Some(c1), Some(c2)) if c2 > c1 => {
                player_2.push_back(c2);
                player_2.push_back(c1);
            },
            _ => unreachable!()
        }
        if player_1.is_empty() || player_2.is_empty() {
            break;
        }
    }

    if player_1.is_empty() {
        score(&player_2)
    } else {
        score(&player_1)
    }
}

fn play_rec(d1: &[u8], d2: &[u8]) -> usize {
    let mut player_1: VecDeque<u8> = d1.iter().copied().collect();
    let mut player_2: VecDeque<u8> = d2.iter().copied().collect();
    
    match rec_round(&mut player_1, &mut player_2) {
        1 | 3  => score(&player_1),
        2  => score(&player_2),
        _ => unreachable!()
    }
}

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn rec_round(p1: &mut VecDeque<u8>, p2: &mut VecDeque<u8>) -> usize {
    let mut hand_cache_1 = HashSet::new();
    let mut hand_cache_2 = HashSet::new();
    loop {
        if p1.is_empty() {
            return 2;
        } else if p2.is_empty() {
            return 1;
        }
        
        let h1 = hash(p1);
        let h2 = hash(p2);

        if hand_cache_1.contains(&h1) || hand_cache_2.contains(&h2) {
            return 1;
        }
        hand_cache_1.insert(h1);
        hand_cache_2.insert(h2);

        let (c1, c2) =  (p1.pop_front().unwrap(), p2.pop_front().unwrap());
        let winner = if p1.len() >= c1 as usize && p2.len() >= c2 as usize {
            let mut p1sub = p1.iter().copied().take(c1 as usize).collect();
            let mut p2sub = p2.iter().copied().take(c2 as usize).collect();
            rec_round(&mut p1sub, &mut p2sub)

        } else {
            if c1 > c2 {
                1
            } else {
                2
            }
        };

        match winner {
            1 => {
                p1.push_back(c1);
                p1.push_back(c2);
            },
            2 => {
                p2.push_back(c2);
                p2.push_back(c1);
            },
            _ => unreachable!()
        }
    }
}

impl Problem for Solution {
    fn parse(&mut self, i: &[u8]) -> Result<()> {
        let result = parse(str::from_utf8(i)?).map_err(|e| anyhow!(e.to_string()))?;
        self.deck_1 = result.1.0;
        self.deck_2 = result.1.1;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        Ok(format!("{}", play(&self.deck_1, &self.deck_2)))
    }

    fn part2(&self) -> Result<String> {
        Ok(format!("{}", play_rec(&self.deck_1, &self.deck_2)))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
";
    #[test]
    fn p1() {
        let result = parse(TEST);
        println!("{:?}", result);
        assert!(result.is_ok());
        let (d1, d2) = result.unwrap().1;
        assert_eq!(d1, vec![9,2,6,3,1]);
        assert_eq!(d2, vec![5,8,4,7,10]);
        assert_eq!(play(&d1, &d2), 306);
        assert_eq!(play_rec(&d1, &d2), 291);
    }
}

