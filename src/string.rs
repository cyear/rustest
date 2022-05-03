fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    let z = s; //废除s变量
    println!("{}", z);
}
