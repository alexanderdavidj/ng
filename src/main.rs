use rand::Rng;
use std::cmp::Ordering;
use std::env;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut low: i32 = 1;
    let high: i32;

    if args.len() > 1 && args.len() > 2 {
        low = args[2].trim().parse().expect("FUCK");
        high = args[1].trim().parse().expect("FUCK");
    } else if args.len() > 1 && args.len() < 2 {
        high = args[1].trim().parse().expect("FUCK");
    } else {
        high = 50
    }

    println!("{} --- {}", low, high);

    let number = rand::thread_rng().gen_range(low..high);
    let mut wrong: i32 = 0;
    let mut sum: i32 = 0;

    loop {
        wrong += 1;
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("NaN");
        let guess: i32 = guess.trim().parse().expect("NaN");
        sum += guess;

        match guess.cmp(&number) {
            Ordering::Less => println!(">"),
            Ordering::Greater => println!("<"),
            Ordering::Equal => {
                wrong -= 1;
                sum -= guess;

                println!(
                    "{} - {} - {} = {}",
                    number,
                    sum,
                    wrong,
                    number - sum - wrong
                );

                break;
            }
        }
    }
}
