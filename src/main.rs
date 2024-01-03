use std::io;
#[derive(Debug)]

struct User {
    email: String,
    name: String,
    age: u32,
}

fn main() {
    let mut active = false;
    let mut passive = false;
    let mut user_vec: Vec<User> = Vec::new();

    // println!("type the index in which you want to store the info in the struct");
    // let mut q = String::new();
    // io::stdin().read_line(&mut q).expect("failed");
    // let q:usize = q.trim().parse().expect("failed");
    if active == false {
        loop {
            println!("please enter your email");
            let mut email1 = String::new();
            io::stdin()
                .read_line(&mut email1)
                .expect("Failed to read line");
            let email1 = email1.trim().to_string();

            println!("please enter your name");
            let mut name1 = String::new();
            io::stdin()
                .read_line(&mut name1)
                .expect("Failed to read line");
            let name1 = name1.trim().to_string();

            println!("please enter your age");
            let mut age1 = String::new();
            io::stdin()
                .read_line(&mut age1)
                .expect("Failed to read line");
            let age1: u32 = age1.trim().parse().expect("Please type a number!");

            let user1 = User {
                email: email1,
                name: name1,
                age: age1,
            };
            user_vec.push(user1);

            println!("do you want to add more data to struct?");
            let mut t = String::new();
            io::stdin()
                .read_line(&mut t)
                .expect("failed to read boolean");
            let t: bool = t.trim().parse().expect("failed");
            if t == true {
                active = false;
            } else if t == false {
                active = true;
            }

            if active == true {
                break;
            }
        }
    }

    loop {
        if passive == false {
            println!("please type the index number for the struct you want to view");
            let mut a = String::new();
            io::stdin().read_line(&mut a).expect("failed to read");
            let a: usize = a
                .trim()
                .parse()
                .expect("please type the index number for the struct you want to view");

            if let Some(user) = user_vec.get(a) {
                println!("User details: {:?}", user);
            }
            println!("do you want to view more struct indexes?");
            let mut i = String::new();
            io::stdin().read_line(&mut i).expect("failed");
            let i: bool = i.trim().parse().expect("please enter true or false");

            if i == false {
                passive = true;
            }

            if passive == true {
                break;
            }
        }
    }
}
