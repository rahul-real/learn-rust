fn main() {
    //borrow_func();
    // let mut x: i32 = 0;``
    // while x < 1000000000 {
    //     x += 1;
    // }
    // println!("x value is {}", x);
    //declaring Strings
    let s1 = String::from("Rahul");
    //s1 value is assigned to s2 so s2 pointes to the s1 pointer and s1 is freed you cannot use
    //s1 if you need s1 then use clone which now creates a new data in heap and a new pointer and points to it
    let s2 = s1.clone();
    println!("{} {}", s1, s2);

    let s = String::from("s");
    //s ownership is now moved to new_string fn s is freed
    do_something(s);

    let x = 5;
    //incase of x the value is saved on stack rather on heap
    pass_number(x);
    println!("{}", x);

    let s3 = get_s3();
    println!("{}", s3);

    let mut x: Box<i32> = Box::new(1);
    *x = 12;
    assert_eq!(*x, 12);
    println!("{}", *x);

    //introduction to structs are nothing but DTOs(java)
    struct Student {
        name: String,
        number: i32,
    }
    let student = Student {
        name: String::from("Rahul"),
        number: 32,
    };
    println!("name {}", student.name);
    println!("number {}", student.number);
    let Student { name, ref number } = student;
    //println!("name {}", student.name);
    println!("number {}", student.number);
    println!("{} and {}", name, number);

    //tuples
    let y = (String::from("hi"), String::from("Dad"));
    println!("{} {}", y.0, y.1);
    //references
    let b = String::from("value");
    let size = size_of_b(&b);
    println!("the size of {} is {}", b, size);
    //mutable references
    //let m3 = 6;
    // let m4 = &m3;
    // let mut m = String::from("value");
    // let m1 = &m;
    let mut r = String::from("hi");
    let r1 = change_r(&mut r);
    let r2 = &mut r;
    println!("{} {}", r1, r2);
    println!("Changed String form {} to {} ", r, r1);

    let b = 32;
    println!("b value is {}", b);
    println!("b value is {}", b);
    change_b_value(b);
    println!("b value is {}", b);
    let b1 = String::from("Nothing to see here");
    println!("b value is {}", b1);
    //the ownership is moved from b to the function removing_ownership func we cannot use b1 again but can use clone method to use b1 agian
    removing_ownership(b1.clone());
    println!("b value is {}", b1);
    //Referencing
    let b2 = String::from("Peek");
    println!("{}", b2);
    only_giving_ref(&b2);
    println!("{}", b2);
    //referencing with mut
    let mut b3 = String::from("Hola");
    println!("{}", b3);
    change_b3(&mut b3);
    println!("{}", b3);

    //Strings in Rust
    let mut st = String::from("Love");
    st.push(' ');
    st.push_str("From");
    st += " India";
    println!("{}", st);
    let st1 = st.replace("India", "Bharat");
    println!("{}", st);
    println!("{}", st1);
    //cannot use st again st2 is the new owner of st and st1 is still the owner of it's value bcz we only used its reference
    let st2 = st + " " + &st1;
    println!("{} {}", st2, st1);

    //Arrays
    let arr = [1, 2, 4, 5];
    let arrel = arr[1];
    let arrel1 = arr.get(2).unwrap();
    let arrel2 = &arr[3];
    let arr_el = &arr[0..4];
    println!(
        "{} {} {} {} {} ",
        arr[0],
        arrel,
        arrel1,
        arrel2,
        arr_el.get(0).unwrap()
    );
    let _a = 1;
    //slice
    let sl = "Slice";
    println!("{}", sl);
    let sl_1 = &arr[0..4];
    println!("{:?}", sl_1);

    //Tuples it only holds 12 elements 1 to 12
    let tu = (1, "dd", String::from("value"));
    println!("{:?}", tu);
    let (a, b, c) = tu;
    println!("{} {} {} {:?}", a, c, b, tu.1);
    let (z, x, c);
    (c, x, z) = (1, "X", 8.8);
    println!("{} {} {} ", c, x, z);
    let (x, y) = sum_multiply((2, 5));
    println!("{} {}", x, y);

    //Structs
    let person = Person {
        name: String::from("Rahul"),
        age: 23,
    };
    println!("{:?}", person);
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    let c1 = Color(12, 123, 322);
    println!("{:?}", c1);
    let p1 = Person { name, ..person };
    println!("{:?}", p1);
    //enums
    #[derive(Debug)]
    enum Names {
        Male(String),
        Female(String),
    }
    let m1 = Names::Male(String::from("Rahul"));
    let f1 = Names::Female(String::from("Samantha"));
    let f2 = Names::Female(String::from("Rebecca"));
    println!("{:?} {:?} {:?} ", m1, f1, f2);
    println!("{:?}", Numbers::Zero as u8);
    let diff = Different::ExStruct { x: 12, y: 15 };
    println!("{:?}", x);
    let diff_1 = Different::ExTuple(String::from("ex_tuple"), 45);
    println!("{:?}", diff_1);
    let diff_2 = Different::ExTuple1(String::from("ex_Tuple_2"));
    println!("{:?}", diff_2);
}
#[derive(Debug)]
enum Different {
    ExStruct { x: i32, y: i32 },
    ExTuple(String, i32),
    ExTuple1(String),
}
#[derive(Debug)]
enum Numbers {
    Zero,
    One,
    Two,
}
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}
fn sum_multiply(num: (i32, i32)) -> (i32, i32) {
    (num.0 * num.1, num.0 + num.1)
}
fn change_b3(b3: &mut String) {
    b3.push_str(" Amigo")
}
fn only_giving_ref(b2: &String) {
    println!("we got only the address of the b2 {}", b2);
}
fn removing_ownership(b1: String) {
    println!("{}", b1);
}
fn change_b_value(b1: i32) {
    println!("{}", b1);
}
fn change_r(r: &mut String) -> String {
    r.push_str(" Brother");
    r.to_string()
}
fn size_of_b(s: &String) -> usize {
    s.len()
}
fn get_s3() -> String {
    let s4 = String::from("Hi MOM");
    s4
}
fn do_something(new_string: String) {
    println!("{}", new_string);
}

fn pass_number(get_number: i32) {
    println!("{}", get_number);
}
