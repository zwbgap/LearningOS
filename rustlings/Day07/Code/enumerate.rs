/*
 * @Author: ZAM
 * @Date: 2023-10-20 23:35:12
 * @LastEditors: zam
 * @LastEditTime: 2023-10-20 23:35:46
 * @Description: file content
 * @FilePath: \LearningOS\rustlings\Day07\Code\enumerate.rs
 */
#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
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
fn main() {
    enum_test1();
}