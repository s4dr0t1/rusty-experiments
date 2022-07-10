use rand::Rng;
use clap::Parser;

///A simple program which generates a random number between 1 - 100 and tells if your guess is correct or not
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    ///The user input, must be an unsigned integer
    #[clap(short, long, value_parser)]
    user_input: usize
}


fn main() {
    let cli_arguments = Args::parse();

    let random_number: usize = rand::thread_rng().gen_range(0..101);
//    dbg!(cli_arguments);

    println!("Your input: {}\nThe random number: {}", cli_arguments.user_input, random_number);
    if cli_arguments.user_input == random_number {
        println!("Your guess is correct");
    }
    else {
        println!("Your guess is incorrect");
    }

}
