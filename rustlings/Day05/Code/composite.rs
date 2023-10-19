/*
 * @Author: ZAM
 * @Date: 2023-10-19 13:37:04
 * @LastEditors: zam
 * @LastEditTime: 2023-10-19 13:37:46
 * @Description: file content
 * @FilePath: \LearningOS\rustlings\Day05\Code\composite.rs
 */

fn main() {
    //println!("Hello, world!");
    string_test1();
}

fn string_test1() {
    let mut s = String::new();
    s.push_str("hello");
    s.push_str(", world!");
    println!("{}", s);
}