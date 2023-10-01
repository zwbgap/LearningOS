/*
 * @Author: ZAM
 * @Date: 2023-10-01 14:11:18
 * @LastEditors: zam
 * @LastEditTime: 2023-10-01 15:23:14
 * @Description: file content
 * 
 * @FilePath: \LearningOS\rustlings\Rust Course\2  Basic introduction to Rust\2.2 Basic type\numeric_type\src\main.rs
 */

 use num::complex::Complex;
fn main() {
    //println!("Hello, world!");
    floating_point_number();
    nan_test();
    number_crunching();
    comprehensive_example();
    bit_operations();
    rational_numbers();
}


/**
 * @description: floating_point_number
 * @return {*}
 */
fn floating_point_number() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    //assert!(xyz.0 + xyz.1 == xyz.2);
    println!("abc.0 + abc.1 == abc.2: {}", abc.0 + abc.1 == abc.2);
    println!("xyz.0 + xyz.1 == xyz.2: {}", xyz.0 + xyz.1 == xyz.2);
}
/*
Output:
abc (f32)
   0.1 + 0.2: 3e99999a
         0.3: 3e99999a

xyz (f64)
   0.1 + 0.2: 3fd3333333333334
         0.3: 3fd3333333333333

thread 'main' panicked at 'assertion failed: xyz.0 + xyz.1 == xyz.2', src\main.rs:29:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
 */

 
 /**
  * @description: nan_test
  * @return {*}
  */ 
 fn nan_test() {
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
      println!("{} is NaN!", x);
      println!("This is not printed!");
    }
    assert!(x.is_nan());
  }
/*
Output:
NaN is NaN!
This is not printed!
*/

/**
 * @description: number_crunching
 * @return {*}
 */
fn number_crunching() {
    // 加法
    let sum = 5 + 10;
    // 减法
    let difference = 95.5 - 4.3;
    // 乘法
    let product = 4 * 30;
    // 除法
    let quotient = 56.7 / 32.2;
    // 求余
    let remainder = 43 % 5;
    println!("sum: {},\ndifference: {},\nproduct: {},\nquotient: {},\nremainder: {}", sum, difference, product, quotient, remainder);
}
/*
Output:
sum: 15,
difference: 91.2,
product: 120,
quotient: 1.7608695652173911,
remainder: 3
*/


/**
 * @description: Comprehensive example of number crunching
 * @return {*}
 */
fn comprehensive_example() {
    // 编译器会进行自动推导，给予twenty i32的类型
    let twenty = 20;
    // 类型标注
    let twenty_one: i32 = 21;
    // 通过类型后缀的方式进行类型标注：22是i32类型
    let twenty_two = 22i32;

    // 只有同样类型，才能运算
    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    // 对于较长的数字，可以用_进行分割，提升可读性
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    // 定义一个f32数组，其中42.0会自动被推导为f32类型
    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];

    // 打印数组中第一个值，并控制小数位为2位
    println!("{:.2}", forty_twos[0]);

}

/*
Output:
20 + 21 + 22 = 63
1000000000000
42.00
*/



fn bit_operations() {
    // 二进制为00000010
    let a:i32 = 2;
    // 二进制为00000011
    let b:i32 = 3;

    println!("(a & b) value is {}", a & b);

    println!("(a | b) value is {}", a | b);

    println!("(a ^ b) value is {}", a ^ b);

    println!("(!b) value is {} ", !b);

    println!("(a << b) value is {}", a << b);

    println!("(a >> b) value is {}", a >> b);

    let mut a = a;
    // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    a <<= b;
    println!("(a << b) value is {}", a);
}

/*
Output:
(a & b) value is 2
(a | b) value is 3
(a ^ b) value is 1
(!b) value is -4
(a << b) value is 16
(a >> b) value is 0
(a << b) value is 16
*/


/**
 * @description: rational_numbers and plural
 * @return {*}
 */
fn rational_numbers(){
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
 
    println!("{} + {}i", result.re, result.im)
}

/*
Output:
13.2 + 21i
*/