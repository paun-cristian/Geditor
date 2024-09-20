fn main() {
    let mut s = String::from("hello");

    let mut r1 = &mut s;
    change(&mut r1);
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}