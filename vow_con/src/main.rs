fn main() {
    let ch = 'a';

    match ch {
        'a' | 'e' | 'i' | 'o' | 'u' => println!("{} is a vowel", ch),
        'b' | 'c' | 'd' | 'f' | 'g' | 'h' | 'j' | 'k' | 'l' | 'm' | 'n' |
        'p' | 'q' | 'r' | 's' | 't' | 'v' | 'w' | 'x' | 'y' | 'z' => println!("{} is a consonant", ch),
        _ => println!("{} is not an alphabetic character", ch),
    }
}

