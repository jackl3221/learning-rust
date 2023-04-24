
fn test1()
{
    let name = "Alice";
    let age = 30;
    let height = 1.65;
    println!("\n String format");
    //use format!
    let formatted_string = format!("Name: {}, Age: {}, Height: {:.2}m", name, age, height);

    println!("{}", formatted_string);
}

fn test2()
{
    //String replace
    let original_string = "Hello, Rust!";
    let replaced_string = original_string.replace("Rust", "World");

    println!("\n String replace");
    println!("Original string: {}", original_string);
    println!("Replaced string: {}", replaced_string);

    /* append string end of string */
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("Origin string: hello, append string: word , total string: {}", s);
}

fn test3()
{
    // data move
    let s1 = String::from("hello");
    println!("\n Sample data move");
    println!("s1 = {}", s1);
    let s2 = s1;
    println!("s2 = {}", s2);
}

fn test4()
{
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("\n Sample data copy");
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn test_string(s: String) { 
    println!("{}", s);
}

fn test_int(x: i32) { 
    println!("{}", x);
}

fn test5()
{
    let s = String::from("hello");
    let x = 5;
    println!("\n Sample string fn1");
    test_string(s);
    test_int(x);
}

fn test_str_fn1() -> String {
    let s = String::from("test1");
    s
}

fn test_str_fn2(s: String) -> String {
    s
}

fn test6()
{
    let s1 = test_str_fn1();
    let s2 = String::from("test2");
    let s3 = test_str_fn2(s2);
    println!("\n Sample string fn2");
    println!("\ns1 = {}", s1);
    println!("s3 = {}", s3);
}

fn cal_len(s: &String) -> usize {
    let length = s.len();
    return length;
}

fn test7()
{
    /* Reference and Borrow */
    println!("\n cal string length");
    let s1 = String::from("12345");
    let len = cal_len(&s1);
    println!("'{}'length {}", s1, len);
}

fn add_str(s: &mut String) {
    s.push_str(", world");
}

fn test8()
{
    /* Mutable reference */
    let mut s = String::from("hello");
    println!("\n Mutable reference");
    add_str(&mut s);
    println!("s = {}", s);
}

fn test9()
{
    /* stray reference */
    let mut s = String::from("hello");
    println!("\n stray reference");
    let r1 = &s;
    let r2 = &s;
    println!("r1={} and r2={}", r1, r2);
    let r3 = &mut s;
    println!("r3={}", r3);
    let r4 = &mut s;
    println!("r4={}", r4);
}

fn main()
{
    test1();
    test2();
    test3();
    test4();
    test5();
    test6();
    test7();
    test8();
    test9();
}
