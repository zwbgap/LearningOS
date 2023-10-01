/*
 * @Author: ZAM
 * @Date: 2023-10-01 14:41:06
 * @LastEditors: zam
 * @LastEditTime: 2023-10-01 14:41:26
 * @Description: file content
 * @FilePath: \LearningOS\rustlings\Day01\variables.rs
 */

 /*fn main() {
//     println!("Hello, world!");
}*/

// 变量的不可变性
/*fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}*/

// 变量的可变性
/*fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}*/

//使用下划线开头忽略未使用的变量
/*fn main() {
    let _x = 5;
    //let y = 10;
    let _y = 15;
}*/

//常量的声明
/*
- 常量不允许使用 mut。常量不仅仅默认不可变，而且自始至终不可变，
- 常量使用 const 关键字而不是 let 关键字来声明，并且值的类型必须标注。
*/
const _MAX_POINTS: u32 = 100_000; //常量未被使用

//变量遮蔽(shadowing)
fn main() {
    let x = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    let x = x + 1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
