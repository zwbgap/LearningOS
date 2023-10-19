/*
 * @Author: ZAM
 * @Date: 2023-10-19 13:41:48
 * @LastEditors: zam
 * @LastEditTime: 2023-10-19 14:37:43
 * @Description: file content
 * @FilePath: \LearningOS\rustlings\Day05\Code\tuple-example.rs
 */

 fn main(){
    tuple_example1();
    tuple_example2();
    tuple_example3();
    tuple_example4();
    tuple_example5();
    tuple_example6();
 }

 fn tuple_example1() {
    let _t0: (u8,i16) = (0, -1);
    // 元组的成员还可以是一个元组
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    // 填空让代码工作
    //let t: (u8, __, i64, __, __) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
    let _t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
 }

 fn tuple_example2() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.0, "i");
    assert_eq!(t.1, "am");
    assert_eq!(t.2, "sunface");

 }

 fn tuple_example3() {
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple);
 }

 fn tuple_example4() {
    let tup = (1, 6.4, "hello");

    // 填空
    //let __ = tup;
    let (x,z,y) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);
 }

 fn tuple_example5() {
    let (x, y, z);

    // 填空
    //__ = (1, 2, 3);
    (y, z, x) = (1, 2, 3);
    
    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);
 }

 fn tuple_example6() {
      // 填空，需要稍微计算下
      //let (x, y) = sum_multiply(__);
      let (x, y) = sum_multiply((2, 3));

      assert_eq!(x, 5);
      assert_eq!(y, 6);
  }
  
  fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
      (nums.0 + nums.1, nums.0 * nums.1)
  }