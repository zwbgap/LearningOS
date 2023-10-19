/*
 * @Author: ZAM
 * @Date: 2023-10-19 13:37:04
 * @LastEditors: zam
 * @LastEditTime: 2023-10-19 15:00:04
 * @Description: file content
 * @FilePath: \LearningOS\rustlings\Day05\Code\strings-slices.rs
 */

fn string_test1() {
    let mut s = String::new();
    s.push_str("hello");
    s.push_str(", world!");
    println!("{}", s);
}

fn string_test2() {
    let s = String::from("hello world");

    let word = first_word(&s);

    //s.clear(); // error!

    println!("the first word is: {}", word);
}
fn first_word(s: &String) -> &str {
    &s[..1]
}

fn string_test3() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]); 
}

fn string_test4() {
    let s = String::from("hello,world!");
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());
}

fn say_hello(s: &str) {
    println!("{}",s);
}

//push 追加
fn string_test5() {
    let mut s = String::from("Hello ");
    
    s.push_str("rust");
    println!("追加字符串 push_str() -> {}", s);

    s.push('!');
    println!("追加字符 push() -> {}", s);
}
//insert 插入
fn string_test6() {
    let mut s = String::from("Hello rust!");
    s.insert(5, ',');
    println!("插入字符 insert() -> {}", s);
    s.insert_str(6, " I like");
    println!("插入字符串 insert_str() -> {}", s);
}
//替换
//replace
fn string_test7() {
    let s = String::from("Hello rust!");
    let new_s = s.replace("rust", "world");
    println!("替换字符串 replace() -> {}", new_s);
}
//replacen
fn string_test8() {
    let string_replace = "I like rust. Learning rust is my favorite!";
    let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
    dbg!(new_string_replacen);
    //println!("替换字符串 replacen() -> {}", new_string_replacen);
}

//replace_range
//仅适用于String
fn string_test9() {
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);
    //println!("替换字符串 replace_range() -> {}", string_replace_range);
}

//delete 删除
fn string_test10() {
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);
}

//remove 删除并返回指定位置的字符串
fn string_test11() {
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    // 删除第一个汉字
    string_remove.remove(0);
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字
    // string_remove.remove(3);
    dbg!(string_remove);
}

//truncate 截断指定位置之后所有内容
fn string_test12() {
    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate);
}

//clear
fn string_test13() {
    let mut string_clear = String::from("测试clear");
    string_clear.clear();
    dbg!(string_clear);
}

//concatenate
fn string_test14() {
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust会自动解引用为&str
    let result = string_append + &string_rust;
    let mut result = result + "!"; // `result + "!"` 中的 `result` 是不可变的
    result += "!!!";

    println!("连接字符串 + -> {}", result);
}

//add() 定义：fn add(self, s: &str) -> String
/*
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
    let s3 = s1 + &s2;
    assert_eq!(s3,"hello,world!");
    // 下面的语句如果去掉注释，就会报错
    // println!("{}",s1);
}
 */

//format!
fn string_test15() {
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);
}

//字符串转义 : \
//在某些情况下，可能你会希望保持字符串的原样，不要转义：\\
fn string_test16() {
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    // 使用\忽略换行符
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);
}


fn main() {
    string_test1();
    string_test2();
    string_test3();
    string_test4();
    string_test5();
    string_test6();
    string_test7();
    string_test8();
    string_test9();
    string_test10();
    string_test11();
    string_test12();
    string_test13();
    string_test14();
    string_test15();
    string_test16();
}


