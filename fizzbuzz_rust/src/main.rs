


fn main() {

    for i in 1..101 {
        fn fizzbuzz(i: u32) -> String {
            let result = if i%3 == 0 && i%5 == 0 {
                 String::from("FizzBuzz")
            } else if i%3 == 0 {
                 String::from("Fizz")
            } else if i%5 == 0 {
                 String::from("Buzz")
            } else {
                 format!("{}", i)
            };
            result
        }
        println!("{}",fizzbuzz(i));
    }
}

