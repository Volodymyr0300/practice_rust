// Simple, given a string of words, return the length of the shortest word(s).
fn find_short(s: &str) -> u32 { // "bitcoin take over the world maybe who knows perhaps" => 3 (3 is the length of the shortest word "who")
    s.split_whitespace() // Split the string into words by whitespace
        .map(|word| word.len() as u32) // Map each word to its length as u32
        .min() // Find the minimum length
        .unwrap_or(0) // Return 0 if there are no words
}

// Given a month as an integer from 1 to 12, return to which quarter of the year it belongs as an integer number.
fn quarter_of(month: u8) -> u8 {
    /*
    if  month <= 3 {
        1
    } else if month <= 6 {
        2
    } else if month <= 9 {
        3
    } else {
        4
    }
     */
    //! or use match
    /* 
    match month {
        1..=3 => 1,
        4..=6 => 2,
        7..=9 => 3,
        10..=12 => 4,
        _ => panic!("month must be between 1 and 12"),
    }
    */
    //! or use arithmetic
    (month - 1) / 3 + 1
}

// function that takes two integers (a, b, where a < b) and return an array of all integers between the input parameters, including them.
fn between(a: i16, b: i16) -> Vec<i16> {
    (a..=b).collect()
}

// When provided with a number between 0-9, return it in words. Note that the input is guaranteed to be within the range of 0-9. (For example: 0 "zero", 1 "one", 2 "two", etc.) If your language supports it, try using a switch statement.
fn switch_it_up(n: usize) -> &'static str {
    match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => panic!("Input must be between 0 and 9"),
    }
}

// Your task is to make a function that can take any non-negative integer as an argument and return it with its digits in descending order. Essentially, rearrange the digits to create the highest possible number.
fn descending_order(x: u64) -> u64 {
    let mut digits: Vec<char> = x.to_string().chars().collect(); // Convert the number to a string and then to a vector of characters
    digits.sort_by(|a, b| b.cmp(a)); // Sort the characters in descending order
    digits.iter().collect::<String>().parse::<u64>().unwrap() // Collect the characters back into a string and parse it as u64
}

fn main() {
    println!("The shortest word contains {} characters.", find_short("bitcoin take over the world maybe who knows perhaps"));
    println!("The Quarter of Month is {}", quarter_of(4));
    println!("The array of all integers between the input parameters is {:?}", between(1, 5));
    println!("The number in words is {}", switch_it_up(7));
    println!("The highest possible number is {}", descending_order(872345));
}
