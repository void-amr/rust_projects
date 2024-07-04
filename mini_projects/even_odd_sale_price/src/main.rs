/*
 * @Author -> AryanRanjane(ranjanaryan14@gmail.com)
 * @Brief -> Demonstration of functions in rust
 * @Date -> 1/7/24
 */

// This store is having a sale where if the price is an even number, you get 10
// Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.

use std::io;

fn main() {
    let mut price = String::new();

    println!("Enter the price of item");

    /* Taking input from console via standard libraries IO set's stdin() method which has
     *  a .read_line() function which takes argument ie where data would be stored from the
     * data read from console and stored to the variable
     */

    io::stdin()
        .read_line(&mut price)
        .expect("Failed to enter price");

    let price: u32 = price.trim().parse().expect("Not a number");

    println!("The selling price is {:?}", sale_price(price));
}

fn sale_price(price: u32) -> u32 {
    if is_even(price) {
        /* True block calling the is_even function */
        price - 10
    } else {
        /* False block calling the is_odd function */
        price - 3
    }
}

fn is_even(num: u32) -> bool {
    /* function returns the i32 type value */
    num % 2 == 0
}
