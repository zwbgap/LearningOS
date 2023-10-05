### **所有权**
#### **1.🌟🌟**
```rust
fn main() {
    // 使用尽可能多的方法来通过编译
    let x = String::from("hello, world");
    let y = x;
    println!("{},{}",x,y);
}
Update:
fn main() {
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{},{}",x,y);
}
```
#### **2.🌟🌟**
```rust
// 不要修改 main 中的代码
fn main() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}
// 只能修改下面的代码!
fn take_ownership(s: String) -> String {
    println!("{}", s);
}
Update:
fn take_ownership(s: String) {
    println!("{}", s);
    s
}
```
#### **3.🌟🌟**
```rust
fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// 只能修改下面的代码!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    // convert String to Vec
    // 将 String 转换成 Vec 类型
    let _s = s.into_bytes();
    s
}
Update:
fn give_ownership() -> String {
    let s = String::from("hello, world");
    s
}
```