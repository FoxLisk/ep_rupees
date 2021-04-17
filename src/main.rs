use proceduralisk::{repeat};
use std::collections::HashMap;
use itertools::Itertools;

// this should be lazy and parametric on type instead of just bools
// but im lazy!!
fn all_bit_strings(length: usize) -> Vec<Vec<bool>> {
    if length < 1 {
        return vec![vec![]];
    }
    if length == 1 {
        vec![vec![true], vec![false]]
    } else {
        let mut leaving: Vec<Vec<bool>> = vec![];
        for bit_string in all_bit_strings(length - 1) {
            let mut wtr = vec![true];
            wtr.extend(bit_string.clone());
            let mut wfa = vec![false];
            wfa.extend(bit_string.clone());

            leaving.push(wtr);
            leaving.push(wfa);
        }
        leaving
    }
}

// this is dumb, only the index changes. probably should be a Box<[u8, 8]> that all the
// instances of the rupee pack reference?
#[derive(Clone, Copy)]
struct PrizePack {
    drops: [u8; 8],
    // no reason for this to be tied to rupees, but also lol
    idx: usize,
}

impl PrizePack {
    fn new(pack: [u8; 8], start_idx: usize) -> Self {
        PrizePack {
            drops: pack,
            idx: start_idx,
        }
    }

    fn next(&mut self) -> u8 {
        let val = self.drops[self.idx];
        self.idx = (self.idx + 1) % 8;
        val
    }

    // ok i guess the reason is that you'd need to use Option<T> here if it's not just how many rupees total
    // i GUESS you'd return some kind of like CumulativeDrops that would be a vec of drops that
    // degenerates to a sum in the case of rupees?
    // look i write rust *because* i like thinking about type systems, but it's not actually useful.
    fn next_if(&mut self, drop: bool) -> u8 {
        if drop {
            self.next()
        } else {
            0
        }
    }
}

fn rupee_pack(start_idx: usize) -> PrizePack {
    PrizePack::new([5, 1, 5, 20, 5, 1, 5, 5], start_idx)
}

fn display_results(results: HashMap<u8, u8>) {
    let mut denominator: u32 = 0;
    let mut total: u32 = 0;
    let mut at_least_9: u32 = 0;

    let mut at_least_4: u32 = 0;
    let it = results.iter();
    for (rupees, times) in it.sorted_by(|a, b| Ord::cmp((*b).1, (*a).1)) {
        let (r, t) = (*rupees, *times);
        println!("Got {:?} rupees {:?} times", rupees, times);
        if r >= 9 {
            at_least_9 += t as u32;
        }if r >= 4 {
            at_least_4 += t as u32;
        }
        total += (r as u32 * t as u32);
        denominator += t as u32;
    }
    println!("EV: {}", (total as f32)/(denominator as f32));
    println!("At least 4 found: {}", at_least_4);
    println!("At least 9 found: {}", at_least_9);
}

fn main() {
    for i in 0..8 {
        println!("On drop #{}", i);
        display_results(from_starting_index(i));
        println!();
    }
}

// HashMap<rupees grabbed, times>
// sum(map.values) == 6!
// this should take a prize pack or like, a prize pack generator function
// but i fucked all this up
fn from_starting_index(start_idx: usize) -> HashMap<u8, u8> {
    let pp = rupee_pack(start_idx);
    let mut map = HashMap::default();
    for bs in all_bit_strings(6) {
        let rupees = drops(bs, pp);
        *map.entry(rupees).or_insert(0) += 1;
    }
    map
}

// this is the dumbest way to model this: the way it actually works in game
// could use some probability theory to reduce work a lot, but:
// 1. i'm stupid
// 2. if i ever want to calculate drops for anything else having the model be accurate might be nice?
// 3. the cardinality is ~0 so who cares
fn drops(rolls: Vec<bool>, mut pp: PrizePack) -> u8 {
    //seems like there must be a better way to assert length == 6
    assert_eq!(6, rolls.len());
    let mut riter = rolls.iter();
    repeat!(4, pp.next_if(*riter.next().unwrap()););
    pp.next_if(*riter.next().unwrap()) + pp.next_if(*riter.next().unwrap())
}

#[cfg(test)]
mod tests {
    use crate::{rupee_pack, drops, all_bit_strings, from_starting_index};
    use itertools::Itertools;

    #[test]
    fn test_all_drop() {
        let rp = rupee_pack(0);
        assert_eq!(6, drops(vec![true, true, true, true, true, true], rp));
    }

    #[test]
    fn test_only_red() {
        let rp = rupee_pack(0);
        assert_eq!(20, drops(vec![true, true, true, false, true, false], rp));
    }

    #[test]
    fn test_all_bitstrings() {
        assert_eq!(
            vec![vec![true], vec![false]],
            all_bit_strings(1));
        assert_eq!(
        vec![
            vec![true, true],
            vec![false, true],
            vec![true, false],
            vec![false, false]
            ],
            all_bit_strings(2));

        assert_eq!(
            vec![
                vec![true, true, true],
                vec![false, true, true],
                vec![true, false, true],
                vec![false, false, true],
                vec![true, true, false],
                vec![false, true, false],
                vec![true, false, false],
                vec![false, false, false],
            ],
            all_bit_strings(3));
    }

    #[test]
    fn test_from_starting_index() {
        let results = from_starting_index(0);
        let t: u32 = results.values().copied().map(|x| x as u32).sum();
        println!("{:?}", results);
        assert_eq!(u32::pow(2, 6), t);

    }
}