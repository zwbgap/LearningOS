/*
 * @Author: ZAM
 * @Date: 2023-10-05 23:51:16
 * @LastEditors: zam
 * @LastEditTime: 2023-10-07 21:52:59
 * @Description: file content
 * @FilePath: \LearningOS\rustlings\Rust Course\2  Basic introduction to Rust\2.4 Composite type\composite_type\src\main.rs
 */

fn main() {
    //println!("Hello, world!");
}

fn string_test1() {
    let mut s = String::new();
    s.push_str("hello");
    s.push_str(", world!");
    println!("{}", s);
}