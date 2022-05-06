fn main() {
    /*
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let n1 = 10i8;
    let n2 = num(n1);
    println!("n1:{} n2:{}", n1, n2);

    let s1 = String::from("hello");
    let s2 = string(s1);
    // Error println!("s1:{} s2:{}", s1, s2);
    */
    let mut s1 = String::from("hello");
    let r1 = &s1;
    let r2 = &s1;
    println!("r1:{} r2:{}", r1, r2);
    //println!("r1:{} r2:{}", r1, r2); 可以输出两次
    let r3 = &mut s1;
    println!("r3:{}", r3);
    //*r3 = String::from("world");
    //println!("r3:{}", r3);
    first_word(&r3);

}
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
            }
    }
    &s[..]
}
fn string(s: String) -> String {
    s
}
fn num(n: i8) -> i8 {
    n
}
fn calculate_length(s: &String) -> usize {
        s.len()
}

