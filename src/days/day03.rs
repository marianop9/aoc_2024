use crate::etc::{get_input_file, Solution};

pub fn solve() -> Solution {
    let input = get_input_file(3);

    Solution::U64(parse_instructions(&input))
}

const VALID_CHARS: [char; 12] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', ',', ')'];

fn parse_instructions(input: &str) -> u64 {
    let mut total: u64 = 0;

    let mut total_offset = 0;
    
    while let Some(offset) = input.get(total_offset..).unwrap().find("mul(") {
        total_offset += offset + 4;

        let instr_start = input
            .get(total_offset..)
            .expect("expected to be within string bounds");

        // let rem = &input[total_offset-4..];
        // println!("(offset {total_offset}) remaining string: {rem}");

        let mut comma_idx: Option<usize> = None;
        let mut nums: [u64; 2] = [0; 2];
        let mut num_count = 0;

        for (i, ch) in instr_start.chars().enumerate() {
            if !VALID_CHARS.contains(&ch) {
                // println!("invaild chars - find next instr (iter {i})");
                total_offset += i;

                if total_offset > input.len() {
                    return total;
                }
                break;
            }

            if ch == ',' && comma_idx.is_none() {
                let numstr = instr_start.get(..i).unwrap();
                // println!("first number is {numstr}");
                if let Ok(n) = numstr.parse::<u64>() {
                    comma_idx = Some(i);

                    nums[0] = n;
                    num_count += 1;

                    continue;
                }
            } else if ch == ',' {
                total_offset += i;
                break;
            }

            if comma_idx.is_some() && ch != ')' && !ch.is_numeric() {
                // println!("break: no num after comma");
                total_offset += i;
                break;
            }

            if ch == ')' {
                if num_count == 1 && comma_idx.is_some() {
                    let comma_idx = comma_idx.unwrap();
                    let numstr = instr_start.get(comma_idx + 1..i).unwrap();
                    // println!("second number is {numstr}");

                    if let Ok(n) = numstr.parse::<u64>() {
                        nums[1] = n;
                        num_count += 1;
                        break;
                    }
                } else {
                    total_offset += i;
                    break;
                }
            }
        }

        if num_count == 2 {
            total += nums[0] * nums[1];
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day03() {
        const TEST_INPUT: &str =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
            // "xxxmul(33,3)";

        let result = parse_instructions(TEST_INPUT);

        assert_eq!(result, 161);
    }
}
