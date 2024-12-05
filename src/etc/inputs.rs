use std::fs;


pub fn get_input_file(daynum: u8) -> String {
    let fname = format!("inputs/day{:02}_input.txt", daynum);

    let input = fs::read_to_string(fname).expect("expected input file");
    input
}