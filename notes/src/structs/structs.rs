pub fn structs() {
    // group of related data
    struct User {
        active: bool,
        username: String,
        email: String,
        sig_in_count: u64,
    }
    let mut kevin = User {
        active: true,
        username: String::from("kevv"),
        email: String::from("kev@mail.m"),
        sig_in_count: 0,
    };
    println!("email is: {}", kevin.email);
    kevin.email = String::from("kevin@mail.m");

    let user = build_user(String::from("Mario"), String::from("Sanchez"));
    println!("username is: {}", user.username);

    // we can create a new User based on the existing user like this
    // after this we can not use kevin's email or usename anymore because we passing ownership
    // of username and email to inactive_kevin, so kevin is destroyed
    let inactive_kevin = User {
        active: false,
        ..kevin
    };
    println!(
        "active status is: {}, sign in count is: {}",
        inactive_kevin.active, inactive_kevin.sig_in_count
    );
    // Error: username moved println!("active status is: {}", kevin.username);

    // functions that creates a User and returns its ownership
    fn build_user(username: String, email: String) -> User {
        User {
            username,
            email,
            active: true,
            sig_in_count: 1,
        }
    }
    // we can define tuple structs that are simply named tuples, this is useful to differentiate
    // two tuples who has the same fields
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    // p1 and red are different values
    let p1 = Point(0, 0, 0);
    let red = Color(255, 0, 0);
    println!("point is: {}", p1.0);
    println!("red is: {}", red.0);

    // simple example to calculate the are of a rectangle
    #[derive(Debug)]
    struct Rectangle {
        width: i32,
        height: i32,
    }
    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };
    fn area(rec: &Rectangle) -> i32 {
        rec.width * rec.height
    }
    println!("the area of the rectangle is: {}", area(&rect1));
    println!("rectangle is: {:?}", &rect1);
    // we can use the dbg! macro to print helful info, dbg! takes ownership of a variable and
    // returns the variable
    dbg!(&rect1);
    // we can add methods to Rectangle which will be available only on Rectangle instances
    impl Rectangle {
        fn area(&self) -> i32 {
            self.height * self.width
        }
        // we can have associated functions that dont use self (usefull for constructors)
        fn create(width: i32, height: i32) -> Rectangle {
            Rectangle {
                width,
                height,
            }
        }
    }
    println!("the area of the rectangle is: {}", rect1.area());
    println!("rectangle created is: {:?}", Rectangle::create(2, 2));
}
