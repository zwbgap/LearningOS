/*
 * @Author: ZAM
 * @Date: 2023-10-04 22:58:52
 * @LastEditors: zam
 * @LastEditTime: 2023-10-04 23:32:48
 * @Description: file content
 * @FilePath: \LearningOS\rustlings\Rust Course\2  Basic introduction to Rust\2.3 Ownership and Borrowing\ownership\src\main.rs
 */

fn main() {
    //println!("Hello, world!");
    ownership_test1();
    ownership_test2();
}

fn ownership_test1() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s2);
    //println!("{}, world!", s1);
}

/*
Output:
error[E0382]: borrow of moved value: `s1`
  --> src\main.rs:19:28
   |
16 |     let s1 = String::from("hello");
   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait    
17 |     let s2 = s1;
   |              -- value moved here
18 |     println!("{}, world!", s2);
19 |     println!("{}, world!", s1);
   |                            ^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
*/
fn ownership_test2() {
    let x: &str = "hello, world";
    let y = x;
    println!("{},{}",x,y);
}
/*
Output:
hello, world,hello, world
*/

fn ownership_test3() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}

/*
Output:
s1 = hello, s2 = hello
x = 5, y = 5
*/