/*
 * @Author: ZAM
 * @Date: 2023-10-20 23:35:12
 * @LastEditors: zam
 * @LastEditTime: 2023-10-21 20:21:18
 * @Description: file content
 * @FilePath: \LearningOS\rustlings\Day07\Code\enumerate.rs
 */
#[derive(Debug)]
#[allow(dead_code)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}
#[derive(Debug)]
#[allow(dead_code)]
enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(char),
    Hearts(char),
}

fn enum_test1() {
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;
    
    print_suit(heart);
    print_suit(diamond);
    
}

fn print_suit(card: PokerSuit) {
    println!("{:?}",card);
}

fn enum_test2() {
   let c1 = PokerCard::Spades(5);
   let c2 = PokerCard::Diamonds('Q');
   println!("{:?}",c1);
   println!("{:?}",c2);
}

fn enum_test3() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
    println!("{:?}",some_number);
    println!("{:?}",some_string);
    println!("{:?}",absent_number);

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn enum_test4() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}",six);
    println!("{:?}",none);
    println!("{:?}",five);
}

fn main() {
    enum_test1();
    enum_test2();
    enum_test3();
    enum_test4();
    
}