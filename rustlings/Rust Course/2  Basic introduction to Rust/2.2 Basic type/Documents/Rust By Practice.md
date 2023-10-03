### **数值类型**
#### **整数**
##### **1.🌟**
Tips: 如果我们没有显式的给予变量一个类型，那编译器会自动帮我们推导一个类型
```rust
// 移除某个部分让代码工作
fn main() {
    let x: i32 = 5;
    let mut y: u32 = 5;

    y = x;
    
    let z = 10; // 这里 z 的类型是? 
}
Update:
fn main() {
    let x: i32 = 5;
    let mut y = 5;
    y = x;
    let z = 10; // 这里 z 的类型是 i32 
}
```

##### **2.🌟**
```rust
// 填空
fn main() {
    let v: u16 = 38_u8 as __;
}
Update:
fn main() {
    let v: u16 = 38_u8 as u16;
}
```

##### **3.🌟🌟🌟**
Tips: 如果我们没有显式的给予变量一个类型，那编译器会自动帮我们推导一个类型
```rust
// 修改 `assert_eq!` 让代码工作
fn main() {
    let x = 5;
    assert_eq!("u32".to_string(), type_of(&x));
}
// 以下函数可以获取传入参数的类型，并返回类型的字符串形式，例如  "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
Update:
fn main() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));
}
```

##### **4.🌟🌟**
```rust
// 填空，让代码工作
fn main() {
    assert_eq!(i8::MAX, __); 
    assert_eq!(u8::MAX, __); 
}
Update:
fn main() {
    assert_eq!(i8::MAX, 127); 
    assert_eq!(u8::MAX, 255); 
}
```
##### **5.🌟🌟**
```rust

// 修改 `assert!` 让代码工作
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1579);
}
Update:
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert_eq!(v == 1579);
}
```

##### **6.🌟🌟**
```rust
// 解决代码中的错误和 `panic`
fn main() {
   let v1 = 251_u8 + 8;
   let v2 = i8::checked_add(127, 8).unwrap();
   println!("{},{}",v1,v2);
}
Update:
fn main() {
   let v1 = 247_u8 + 8;
   let v2 = i8::checked_add(119, 8).unwrap();
   println!("{},{}",v1,v2);
}
Output:
255,127
```

#### **浮点数**
##### **7.🌟**
```rust
// 将 ? 替换成你的答案
fn main() {
    let x = 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64
    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}
Update:
fn main() {
    let x = 1_000.000_1; // f64
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64
    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}
Output:
Success!
```
##### **8.🌟🌟 使用两种方法来让下面代码工作**
```rust
fn main() {
    assert!(0.1+0.2==0.3);
}
Update:
fn main() {
    assert!(0.1_f32+0.2_f32==0.3_f32);
    assert!((0.1_f64+ 0.2 - 0.3).abs() < 0.001);
}
```
#### **序列Range**
##### **🌟🌟 两个目标: 1. 修改 assert! 让它工作 2. 让 println! 输出: 97 - 122**
```rust
fn main() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -3);

    for c in 'a'..='z' {
        println!("{}",c);
    }
}
Update:
fn main() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);// -3 + -2 + -1 + 0 + 1 = -5 

    for c in 'a'..='z' {
        println!("{}",c as u8);
    }
}
```
##### **10.🌟🌟**
```rust
// 填空
use std::ops::{Range, RangeInclusive};
fn main() {
    assert_eq!((1..__), Range{ start: 1, end: 5 });
    assert_eq!((1..__), RangeInclusive::new(1, 5));
}
Update:
use std::ops::{Range, RangeInclusive};
fn main() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
}
```
explain:
这段 `Rust` 代码演示了如何使用 `std::ops` 模块中的 `Range` 和 `RangeInclusive` 类型来表示一个区间。

首先，使用 `use` 关键字导入了 `std::ops` 模块中的 `Range` 和 `RangeInclusive` 类型。

然后，在 `main` 函数中，使用 `assert_eq!` 宏检查两个区间是否相等。第一个区间是 (1..5)，表示从 1 开始，到 5 结束（不包括 5）的区间。这里使用了 Rust 中的 range 语法，其中 .. 表示不包括右端点。

第二个区间是 (1..=5)，表示从 1 开始，到 5 结束（包括 5）的区间。这里使用了 Rust 中的 range 语法，其中 ..= 表示包括右端点。

注意，`RangeInclusive` 类型的构造函数需要使用 `new` 方法来创建一个新的区间对象。

#### **计算**
##### **11.🌟🌟**
```rust
// 填空，并解决错误
fn main() {
    // 整数加法
    assert!(1u32 + 2 == __);

    // 整数减法
    assert!(1i32 - 2 == __);
    assert!(1u8 - 2 == -1);
    
    assert!(3 * 50 == __);

    assert!(9.6 / 3.2 == 3.0); // error ! 修改它让代码工作

    assert!(24 % 5 == __);
    
    // 逻辑与或非操作
    assert!(true && false == __);
    assert!(true || false == __);
    assert!(!true == __);

    // 位操作
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
Update:
fn main() {
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
```

### **字符、布尔、单元类型**
#### **字符**
##### **1.🌟**
```rust
// 修改2处 `assert_eq!` 让代码工作
use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),1); 

    let c2 = '中';
    assert_eq!(size_of_val(&c2),3); 

    println!("Success!")
} 
Update:
use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4); 
    let c2 = '中';
    assert_eq!(size_of_val(&c2),4); 
    println!("Success!")
} 
```
##### **2.🌟**
```rust
// 修改一行让代码正常打印
fn main() {
    let c1 = "中";
    print_char(c1);
} 

fn print_char(c : char) {
    println!("{}", c);
}
Update:
fn main() {
    let c1 = '中';//字符为单引号,字符串为双引号
    print_char(c1);
}
```
#### **布尔**
##### **3.🌟**
```rust
// 使成功打印
fn main() {
    let _f: bool = false;

    let t = true;
    if !t {
        println!("Success!")
    }
}
Update:
fn main() {
    let _f: bool = false;
    let t = true;
    if t {
        println!("Success!")
    }
}
```
##### **4.🌟**
```rust
fn main() {
    let f = true;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!")
}
Update:
fn main() {
    let f = true;
    let t = true && false;
    assert_eq!(t, !f);

    println!("Success!")
}
```
#### **单元类型**
##### **5.🌟**
```rust
// 让代码工作，但不要修改 `implicitly_ret_unit` !
fn main() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(v, implicitly_ret_unit());

    println!("Success!")
}

fn implicitly_ret_unit() {
    println!("I will return a ()")
}
// 不要使用下面的函数，它只用于演示！
fn explicitly_ret_unit() -> () {
    println!("I will return a ()")
}
Update:
fn main() {
    let v0: () = ();

    let v = (2, 3);
    assert_eq!(v0, implicitly_ret_unit());

    println!("Success!")
}
```

##### **6.🌟🌟 单元类型占用的内存大小是多少？**
```rust
// 让代码工作：修改 `assert!` 中的 `4` 
use std::mem::size_of_val;
fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 4);

    println!("Success!")
}
Update:
use std::mem::size_of_val;
fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);
    //unit类型占用的内存大小为0
    println!("Success!")
}
```
### **语句和表达式**
#### **示例**
```rust
fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // 下面表达式的值将被赋给 `y`
        x_cube + x_squared + x
    };

    let z = {
        // 分号让表达式变成了语句，因此返回的不再是表达式 `2 * x` 的值，而是语句的值 `()`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
```
#### **1.🌟🌟**
```rust
// 使用两种方法让代码工作起来
fn main() {
   let v = {
       let mut x = 1;
       x += 2
   };

   assert_eq!(v, 3);
}
Update:
// 使用两种方法让代码工作起来
fn main() {
   let v = {
       let mut x = 1;
       x += 2;
       x
   };
   assert_eq!(v, 3);
}
```

#### **2.🌟**
```rust
fn main() {
   let v = (let x = 3);

   assert!(v == 3);
}
Update:
fn main() {
    let v = {
        let x = 3;
        x
    }
    assert!(v == 3);
}
```
#### **3.🌟🌟**
```rust
fn main() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y;
}
Update:
fn sum(x: i32, y: i32) -> i32 {
    x + y
}
```

### **函数**
#### **1.🌟🌟🌟**
```rust
fn main() {
    // 不要修改下面两行代码!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);
}

fn sum(x, y: i32) {
    x + y;
}

Update:
fn sum(x: i32, y: i32) -> i32 {
    x + y
}
```
#### **2.🌟🌟**
```rust
fn main() {
   print();
}

// 使用另一个类型来替代 i32
fn print() -> i32 {
   println!("hello,world");
}
Update:
fn print() -> () {
   println!("hello,world");
}
```

#### **3.🌟🌟🌟**
```rust
// 用两种方法求解
fn main() {
    never_return();
}

fn never_return() -> ! {
    // 实现这个函数，不要修改函数签名!
    //1. implement this function without modifying the function signature!
    //panic!(""I return nothing"");
    
}
Update:
//2. implement this function without modifying the function signature!
use std::thread;
use std::time;
fn never_return() -> ! {
    loop {
          println!("I return nothing");
        // sleeping for 1 second to avoid exhausting the cpu resource
        thread::sleep(time::Duration::from_secs(1));
    }
}
```

##### **4.🌟🌟 发散函数( Diverging function )不会返回任何值，因此它们可以用于替代需要返回任何值的地方**
```rust
fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    
    // 这里与其返回一个 None，不如使用发散函数替代
    never_return_fn()
}

// 使用三种方法实现以下发散函数
fn never_return_fn() -> ! {
   
}
Update:
fn never_return_fn() -> ! {
    //1.unimplemented!();
    //2.panic!();
    //3.todo!();
    /*4.loop{
        std::thread::sleep(std::time::Duration::from_secs(1));
    };
    */
}
```

##### **5.🌟🌟**
```rust
fn main() {
    // 填空
    let b = __;

    let _v = match b {
        true => 1,
        // 发散函数也可以用于 `match` 表达式，用于替代任何类型的值
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic")
        }
    };

    println!("Exercise Failed if printing out this line!");
}
Update:
fn main() {
    // FILL in the blank
    let b = false;

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic")
        }
    };

    println!("Exercise Failed if printing out this line!");
}

```
