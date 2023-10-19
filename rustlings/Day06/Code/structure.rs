/*
 * @Author: ZAM
 * @Date: 2023-10-19 15:10:17
 * @LastEditors: zam
 * @LastEditTime: 2023-10-19 23:45:48
 * @Description: file content
 * @FilePath: \LearningOS\rustlings\Day06\Code\structure.rs
 */
struct USER {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn user_test1() {
    let mut user1 = USER{
        username: String::from("user1"),
        email: String::from("zwbgap@outlook.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("user1: {}, {}, {}, {}", user1.username, user1.email, user1.sign_in_count, user1.active);
    user1.username = String::from("admin");
    println!("user1: {}",user1.username);
}

#[derive(Debug)]
 struct File {
   name: String,
   data: Vec<u8>,
 }

 fn file_test1() {
   let f1 = File {
     name: String::from("f1.txt"),
     data: Vec::new(),
   };

   let f1_name = &f1.name;
   let f1_length = &f1.data.len();

   println!("{:?}", f1);
   println!("{} is {} bytes long", f1_name, f1_length);
 }

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn rectangle_test1() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}



fn main() {
    user_test1();
    file_test1();
    rectangle_test1();
}