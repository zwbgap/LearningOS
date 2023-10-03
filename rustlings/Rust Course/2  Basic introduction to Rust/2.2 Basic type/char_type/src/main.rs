/*
 * @Author: ZAM
 * @Date: 2023-10-03 16:19:28
 * @LastEditors: zam
 * @LastEditTime: 2023-10-03 17:11:04
 * @Description: file content
 * @FilePath: \LearningOS\rustlings\Rust Course\2  Basic introduction to Rust\2.2 Basic type\char_type\src\main.rs
 */

fn main() {
    //println!("Hello, world!");
    char_test();
    bool_test();
    unit_test();
    calculate_test();
}

/**
 * @description: char_test
 * @return {*}
 */
fn char_test() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c:{},z:{},heart_eyed_cat:{}", c, z, heart_eyed_cat);
}

/**
 * @description: bool_test
 * @return {*}
 */
fn bool_test() {
    let _t = true;
    let f: bool = false; // 显式指定类型注解
    if f {
        println!("f is true");
    } else {
        println!("f is false");
    }
}

/**
 * @description: another_function
 * @return {*}
 */
fn unit_test() {
    let _x = 5;
    let y = {
        let x = 3;
        x + 1 // 注意这里没有分号
    };
    println!("y的值为: {}", y);
    let z = char_test();
    println!("z的值为: {:?}", z);
}

fn calculate_test() {
     // Integer addition
     assert!(1u32 + 2 == 3);

     // Integer subtraction
     assert!(1i32 - 2 == -1);
     assert!(1i8 - 2 == -1);
     
     assert!(3 * 50 == 150);
 
     assert!(9 / 3 == 3); // error ! make it work
     // %Take the remainder
     assert!(24 % 5 == 4);
     // Short-circuiting boolean logic
     assert!(true && false == false);
     assert!(true || false == true);
     assert!(!true == false);
 
     // Bitwise operations
     println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
     println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
     println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
     println!("1 << 5 is {}", 1u32 << 5);
     println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

/*
Output:
c:z,z:ℤ,heart_eyed_cat:😻
f is false
y的值为: 4
c:z,z:ℤ,heart_eyed_cat:😻
z的值为: ()
0011 AND 0101 is 0001
0011 OR 0101 is 0111
0011 XOR 0101 is 0110
1 << 5 is 32
0x80 >> 2 is 0x20
*/