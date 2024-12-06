use crate::etc::{get_input_file, Solution};

fn is_seq_safe(seq: Vec<u32>, skip: Option<usize>) -> Option<usize> {
    let mut is_incr: Option<bool> = None;

    let mut skip = skip;
    
    let mut i: usize = 0;
    while i < seq.len()-1 {
        let curr: u32;
        let next: u32;
        if let Some(idx) = skip {
            if idx == i {
                if idx == 0 {
                    return  Some(1);
                }
                curr = seq[i-1];
                next = seq[i+1];
            } else {
                curr = seq[i];
                next = seq[i+1];
            }
        } else {
            curr = seq[i];
            next = seq[i+1];
        }

        if !is_safe(curr, next, &mut is_incr) {
            if let Some(idx) = skip {
                if idx == i {
                    return Some(i)
                }
            }


            if let Some(next) = seq.get(i+2) {
                if !is_safe(curr, *next, &mut is_incr) {
                    skip = Some(i);
                    is_incr = None;
                    continue;
                } else {
                    skip = Some(i+1);
                    i+=1;
                    continue;
                }
            }
        }
        
        skip = None;
        i += 1;
    }

    return None;
}

fn count_safe_reports(input: &str) -> u32 {
    let mut safe_count: u32 = 0;

    for line in input.lines() {
        let seq: Vec<u32> = line
            .split(" ")
            .map(|x| x.parse::<u32>().expect("expected parseable number"))
            .collect();

        let is_safe = is_seq_safe(seq, None);

        if is_safe.is_none() {
            println!("{line}");
            // if unsafe_count == 1 {
            // }
            safe_count += 1;
        }
    }

    safe_count
}

fn is_safe(curr: u32, next: u32, is_incr: &mut Option<bool>) -> bool {
    let diff = (curr as i32) - (next as i32);
    if diff == 0 || diff < -3 || diff > 3 {
        println!("diff! {curr}-{next}");
        return false;
    }

    if let Some(inc) = *is_incr {
        if (inc && diff > 0) || (!inc && diff < 0) {
            println!("inc/dec {curr} - {next}");
            return false;
        }
    } else {
        *is_incr = Some(diff < 0);
    }

    return true;
}

pub fn solve() -> Solution {
    let input = get_input_file(2);

    return Solution::U32(count_safe_reports(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02() {
        //         const TEST_INPUT: &str = "7 6 4 2 1
        // 1 2 7 8 9
        // 9 7 6 2 1
        // 1 3 2 4 5
        // 8 6 4 4 1
        // 1 3 6 7 9
        // ";
        const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
17 19 15 14 11
2 2 3 4 5
2 3 4 5 5";

        let res = count_safe_reports(TEST_INPUT);

        assert_eq!(res, 7);
    }
}
