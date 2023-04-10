/**
 * The only performance critical methods here are `combine_diceset` and `MAX_VAL` (the size of the Vec required for a key-value map with DiceSet as the key)
 * The string methods are for convenience / debugging
 */
use freqdist::FrequencyDistribution;

use crate::defs::NUM_DICE;

pub type DiceSet = u32;

const BASE: u32 = 7u32;
const POWS: &[u32] = &[
    0,
    BASE.pow(0),
    BASE.pow(1),
    BASE.pow(2),
    BASE.pow(3),
    BASE.pow(4),
    BASE.pow(5),
];

const CHARS: &[char] = &['_', '1', '2', '3', '4', '5', '6'];

pub fn get_chars() -> &'static [char] {
    &CHARS[1..]
}

pub const MAX_VAL: u32 = BASE.pow(NUM_DICE as u32);

pub fn empty() -> DiceSet {
    0
}

pub fn combine_diceset(d1: DiceSet, d2: DiceSet) -> DiceSet {
    d1 + d2
}

pub fn to_human_readable(mut d: DiceSet) -> Vec<String> {
    let mut res = Vec::new();
    if d == 0 {
        return res;
    };
    for i in (1..POWS.len()).rev() {
        let m = d / POWS[i];
        for _ in 0..m {
            res.push(CHARS[i].to_string())
        }
        d -= m * POWS[i];
    }
    res
}

pub fn to_sorted_string(d: DiceSet) -> String {
    let mut hr = to_human_readable(d);
    hr.sort();
    hr.join("")
}

pub fn from_char(c: char) -> DiceSet {
    let pos = CHARS.iter().position(|x| *x == c).unwrap();
    from_ind(pos)
}

pub fn from_ind(ind: usize) -> DiceSet {
    POWS[ind]
}

pub fn from_human_readable(readable: Vec<char>) -> DiceSet {
    let mut res: DiceSet = empty();
    for c in readable {
        res = combine_diceset(res, from_char(c))
    }
    res
}

pub fn from_string(readable: &str) -> DiceSet {
    return from_human_readable(readable.chars().collect::<Vec<_>>());
}

pub fn from_human_readable_str(readable: &Vec<&String>) -> DiceSet {
    let mut res: Vec<char> = Vec::new();
    for s in readable {
        let c = s.chars().next().unwrap();
        res.push(c);
    }
    from_human_readable(res)
}

pub fn get_freqdist(d: DiceSet) -> FrequencyDistribution<char> {
    let mut freqdist = FrequencyDistribution::new();
    for s in to_human_readable(d) {
        let c = s.chars().next().unwrap();
        freqdist.insert(c);
    }
    freqdist
}

pub fn subtract_dice(d1: DiceSet, s: &str) -> DiceSet {
    debug_assert!(
        d1 >= from_string(s),
        "{d1} {} not bigger than {s} ({})",
        to_sorted_string(d1),
        from_string(s)
    );
    d1 - from_string(s)
}

#[cfg(test)]
mod tests {
    use crate::dice_set::{from_string, to_sorted_string};

    #[test]
    fn test_to_human_readable() {
        let my_dice = from_string("123456");
        let my_dice_after = to_sorted_string(my_dice);
        assert_eq!("123456", my_dice_after);
    }

    #[test]
    fn test_to_human_readable_blank() {
        let my_dice = from_string("");
        let my_dice_after = to_sorted_string(my_dice);
        assert_eq!("", my_dice_after);
    }
}
