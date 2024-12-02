use std::iter::zip;

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
        let values: Vec<u32>  = line.split("   ")
            .map(|x| x.parse::<u32>().unwrap()).collect();
        let (lval, rval) = (values[0], values[1]);
        
        left.push(lval);
        right.push(rval);
    }

    left.sort();
    right.sort();

    let distance = zip(left, right).map(|(l, r)|
        if l > r {
            l - r
        } else {
            r - l
        }
    ).sum();
    distance
}


#[cfg(test)]
pub mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn day1_example() {
        let result = find_distance(TEST_INPUT);
        assert_eq!(result, 11);
    }

    #[test]
    fn day1() {
        let input = fs::read_to_string("day1_input.txt").expect("expected input file");
        let result = find_distance(&input);

        assert_eq!(result, 936063);
    }
}
