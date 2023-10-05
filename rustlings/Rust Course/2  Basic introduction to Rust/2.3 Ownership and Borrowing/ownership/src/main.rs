/*
 * @Author: ZAM
 * @Date: 2023-10-04 22:58:52
 * @LastEditors: zam
 * @LastEditTime: 2023-10-05 15:28:56
 * @Description: file content
 * @FilePath: \LearningOS\rustlings\Rust Course\2  Basic introduction to Rust\2.3 Ownership and Borrowing\ownership\src\main.rs
 */

fn main() {
    //println!("Hello, world!");
    ownership_test1();
    ownership_test2();
    ownership_test3();
    ownership_test4();
    ownership_test5();
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

fn ownership_test4() {
    let s = String::from("hello");
    takes_ownership(s);
    //println!("{}", s);  error[E0382]: borrow of moved value: `s`
    let x = 5;
    makes_copy(x);
    println!("{}", x);
}
fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

/*
Output:
hello
5
5
*/

fn ownership_test5() {
    let s1 = gives_ownership();         // gives_ownership 将返回值
    // 移给 s1
    let s2 = String::from("hello");     // s2 进入作用域
    let s3 = takes_and_gives_back(s2);  // s2 被移动到
    // takes_and_gives_back 中,
    // 它也将返回值移给 s3
    println!("s1 = {}, s3 = {}", s1, s3);
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 移出作用域并被丢弃
fn gives_ownership() -> String {             // gives_ownership 将返回值移动给
    // 调用它的函数
    let some_string = String::from("hello"); // some_string 进入作用域.
    some_string                              // 返回 some_string 并移出给调用的函数
}
// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    a_string  // 返回 a_string 并移出给调用的函数
}
/*
Output:
s1 = hello, s3 = hello
*/
