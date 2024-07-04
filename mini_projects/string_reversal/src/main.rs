/*
 *  @Author -> AryanRanjane(ranjanearyan14@gmail.com)
 *  @Brief -> String reversal code in rust
 *  @Date -> 2/7/24 
 */

use std::io;

fn main() {
    /* main funciton */
    let mut strin = String::new();
    println!("Enter the string");

    io::stdin().read_line(&mut strin).expect("Not a string"); 
    let ans = reversal(strin);
    println!("reversed string is {ans}"); 
}

fn reversal(sen:String) -> String {
    /* reversal function */
    let mut _string = String::new();
    let _string = sen.clone();
    let _string = sen.chars().rev().collect::<String>(); 
    return _string; 
}