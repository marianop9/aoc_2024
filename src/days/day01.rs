use std::iter::zip;

use crate::etc::{get_input_file, Solution};

const DAYNUM: u8 = 1;
const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";

fn find_distance(input: &str) -> u32 {
    let (mut left, mut right) = (vec![], vec![]);

    for line in input.lines() {
        let values: Vec<u32> = line
            .split("   ")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let (lval, rval) = (values[0], values[1]);

        left.push(lval);
        right.push(rval);
    }

    left.sort();
    right.sort();

    let distance = zip(left, right)
        .map(|(l, r)| if l > r { l - r } else { r - l })
        .sum();
    distance
}

pub fn solve() -> Solution {
    let input = get_input_file(DAYNUM);
    let sol = find_distance(&input);

    Solution::U32(sol)
}
