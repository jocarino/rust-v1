use std::io;

fn main() {
    fn fibonnaci(n: i32) -> i32 {
        let mut prev1: i32 = 0;
        let mut result: i32 = 1;

        for _i in 1..n {
            let temp = result;
            result = prev1 + result;
            prev1 = temp;
        }
        result
    }

    println!("I'll give you the nth fibonacci, if you gimme the n: ");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Couldn't read line");

    let number: i32 = number.trim().parse().expect("Gimme a number please!");

    let result = fibonnaci(number);

    println!("{result}");
}
