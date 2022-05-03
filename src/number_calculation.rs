fn main() {
    println!("hello");
    let mut num = 3;
    println!("number: {}", num);
    num = {
        num + 1
    };
    println!("number: {}", num);
    let mut n = 1;
    'one: loop {
        loop {
            num += 1;
            if num > 50 {
                break;
            }
        }
            n += 1;
            if n > 10 {
                break 'one;
        }
    };
    println!("number: {}", num);
}
