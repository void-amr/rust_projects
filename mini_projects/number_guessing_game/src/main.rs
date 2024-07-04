/*
 *  @Author -> AryanRanjane(ranjanearyan14@gmail.com)
 *  @Brief -> Implemetation of fundamental concepts learnt in rust
 *  @Date -> 3/7/24
 */

/* including or importing a library just as we do in c/c++ or java */
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn taking_input() -> u32 {
    /* this function is used to take user-input */
    let mut guess: String = String::new();
    println!("  *_*Enter a number*_*  ");
    io::stdin()
        .read_line(&mut guess)
        .expect("Input a valid data");
    /* parsing the String to integer of 4byte */
    let my_field: u32 = guess.trim().parse().expect("Not a number");
    println!("\n  you entered {my_field}");
    my_field // this returns the value of my_field and passes on to main_logic function.
}

fn main_logic(my_field: u32) {
    /* this function is used to generate random number and compare them with struct
       data my_field which is set to public

        here rand we are using the rand crat and using the thread_rng()
        random number generator trait
        we are using gen_range()method on thread_rng() method or instance of rand crate
        gen_range() takes parameter here it takes start value ..=endvalue
        it generates random number in the range of 1 to 100
        also the random number generator is local to current thread of execution
        which means if any other tries using this rng instance the current thread will
        not let it to cause it is not synchronized
    */
    let secret_number = rand::thread_rng().gen_range(1..=100);
    match my_field.cmp(&secret_number) {
        Ordering::Less => println!("  Too small!"),
        Ordering::Greater => println!("  Too big!"),
        Ordering::Equal => println!("  You win!"),
    }
}

fn main() {
    /* main function */
    let mut choice: String = String::new();
    loop {
        let input = taking_input();
        main_logic(input);
        println!("  Do you want to continue? [y/n] ");
        io::stdin().read_line(&mut choice).expect("Invalid choice");
        if choice == "y" {
            let input = taking_input();
            main_logic(input);
        } else {
            return;
        }
    }
}
