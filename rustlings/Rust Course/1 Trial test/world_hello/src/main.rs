/*
 * @Author: ZAM
 * @Date: 2023-09-30 10:42:39
 * @LastEditors: zam
 * @LastEditTime: 2023-09-30 13:39:42
 * @Description: file content
 * @FilePath: \LearningOS\rustlings\Rust Course\1 Trial test\world_hello\src\main.rs
 */


// fn main() {
//     println!("Hello, world!");
// }
fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    let regions = [southern_germany, chinese, english];
    /*和其它语言不同，Rust 的集合类型不能直接进行循环，需要变成迭代器(这里是通过 .iter() 方法)。*/
    for region in regions.iter() {
        println!("{}", &region);
    }
    /*  2021 edition 及以后支持直接写 for region in &regions，是因为 for 隐式地将 regions 转换成迭代器。*/
    for region in regions {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}
