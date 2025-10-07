// Simple, given a string of words, return the length of the shortest word(s).
fn find_short(s: &str) -> u32 { // "bitcoin take over the world maybe who knows perhaps" => 3 (3 is the length of the shortest word "who")
    s.split_whitespace() // Split the string into words by whitespace
        .map(|word| word.len() as u32) // Map each word to its length as u32
        .min() // Find the minimum length
        .unwrap_or(0) // Return 0 if there are no words
}

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

fn main() {
    println!("The shortest word contains {} characters.", find_short("bitcoin take over the world maybe who knows perhaps"));
    println!("The Quarter of Month is {}", quarter_of(4))
}
