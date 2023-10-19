/*
 * @Author: ZAM
 * @Date: 2023-10-19 13:42:09
 * @LastEditors: zam
 * @LastEditTime: 2023-10-19 14:00:15
 * @Description: file content
 * @FilePath: \LearningOS\rustlings\Day05\Code\tuple.rs
 */
fn main() {
    //println!("Hello, world!");
    tuple_test1();
}

fn tuple_test1() {
    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度
    (s, length)
}