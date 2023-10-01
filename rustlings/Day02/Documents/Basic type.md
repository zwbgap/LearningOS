### **基本类型**
Rust 每个值都有其确切的数据类型，总的来说可以分为两类：基本类型和复合类型。 基本类型意味着它们往往是一个最小化原子类型，无法解构为其它类型(一般意义上来说)，由以下组成：

- 数值类型: 有符号整数 (i8, i16, i32, i64, isize)、 无符号整数 (u8, u16, u32, u64, usize) 、浮点数 (f32, f64)、以及有理数、复数
- 字符串：字符串字面量和字符串切片 &str
- 布尔类型： true和false
- 字符类型: 表示单个 Unicode 字符，存储为 4 个字节
- 单元类型: 即 () ，其唯一的值也是 ()
  
#### **类型推到与标注**

Rust 编译器很聪明，它可以根据变量的值和上下文中的使用方式来自动推导出变量的类型。 但是，有时候我们需要标注变量的类型，这样做的好处是可以提高代码的可读性，同时也可以为编译器提供更多的信息，帮助编译器更好的进行类型检查。

先来看段代码：
```rust
let guess = "42".parse().expect("Not a number!");
```
忽略 `.parse().expect..`部分，这段代码的目的是将字符串 "42" 进行解析，而编译器在这里无法推导出我们想要的类型：整数？浮点数？字符串？因此编译器会报错：
```rust
$ cargo build
   Compiling no_type_annotations v0.1.0 (file:///projects/no_type_annotations)
error[E0282]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^ consider giving `guess` a type

```
我们需要提供给编译器更多的信息。
例如给 `guess`` 变量一个显式的类型标注：
`let guess: i32 = "42".parse().expect("Not a number!");`
或者在调用 .parse() 方法时，显式的标注出我们想要的类型：
`let guess = "42".parse::<i32>().expect("Not a number!");`
