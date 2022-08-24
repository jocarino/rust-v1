use std::io;

fn main() {
    // let mut s = "hello";
    // s.push_str(", world!");

    println!("Need a string with words separated by space");
    let mut s = String::new();

    io::stdin().read_line(&mut s).expect("Gimme a string!");

    fn get_first_word(set: &String) -> String {
        let mut fist_word = String::from("");
        let set_string = String::from(set);

        for i in 0..set_string.len() - 1 {
            // Makes the string into chars and selects the nth char
            let char = set_string.chars().nth(i).unwrap();
            if char != ' ' {
                let char_to_push = String::from(char);
                fist_word.push_str(&char_to_push);
            } else {
                break;
            }
        }
        fist_word
    }

    let first = get_first_word(&s);

    println!("{}", &first);

    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }
    let first = first_word(&s);
    println!("{}", first);

    fn first_word_by_index(s: &String) -> String {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return s[0..i].to_string();
            }
        }
        s[..].to_string()
    }

    let first = first_word_by_index(&s);
    println!("{}", first);
}
