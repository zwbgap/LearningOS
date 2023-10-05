/*
 * @Author: ZAM
 * @Date: 2023-10-05 15:39:58
 * @LastEditors: zam
 * @LastEditTime: 2023-10-05 23:38:04
 * @Description: file content
 * @FilePath: \LearningOS\rustlings\Rust Course\2  Basic introduction to Rust\2.3 Ownership and Borrowing\quoting\src\main.rs
 */

fn main() {
    //println!("Hello, world!");
    quote_test1();
    quote_test2();
    quote_test3();
}

fn quote_test1() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn quote_test2() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
/*
Output:
The length of 'hello' is 5.
*/

fn quote_test3() {
    let mut s = String::from("hello");
    change(&mut s);
    let r1 = &mut s;
    //let r2 = &mut s;
    //println!("{}, {}", r1, r2);
}    
fn change(some_string: &mut String) {
    some_string.push_str(", world");
    println!("{}", some_string);
}
/*
Output:
hello, world
hello, world
*/