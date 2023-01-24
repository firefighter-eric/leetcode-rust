struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let s = s.as_str();
        let words: Vec<&str> = s.split(' ').collect();
        for word in words.iter().rev() {
            if word.len() == 0 {
                continue
            } else {
                return word.len() as i32
            }
        }
        0
    }
}

fn main() {
    let s = String::from("Hello World");
    let r1 = Solution::length_of_last_word(s);
    println!("{:?}", r1);

    let s = String::from("   fly me   to   the moon  ");
    let r2 = Solution::length_of_last_word(s);
    println!("{:?}", r2);
}