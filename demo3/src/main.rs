struct User {
    username: String,
    age: u32,
    email: String,
}
//  利用impl关键字来定义结构体成员方法
impl User {
    fn sayhello(&self){
        println!("Hello, I'm {}, I'm {} years old, my email is {}.", 
            self.username, 
            self.age, 
            self.email
        );
    }
    // 修改年龄
    fn change_age(&mut self, newage: u32){
        self.age = newage;
    }
}

// fn sayhello(user:&User){
//     println!("Hello, I'm {}, I'm {} years old, my email is {}.", user.username, user.age, user.email);
// }

fn main() {
    let mut someone = User{
        username: String::from("someone"),
        age:35,
        email: String::from("someone@example.com"),
    };
    // someone.age = 18;
    // println!("username is {}", someone.username);
    // println!("age is {}", someone.age);
    // print!("email is {}", someone.email);
    // sayhello(&someone);
    // someone.sayhello();
    someone.change_age(25);
    someone.sayhello();
}