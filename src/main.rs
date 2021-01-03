use std::fmt::Debug;

fn match_option<T: Debug>(o: Option<T>) {
    match o {
        Some(i) => println!("{:?}", i),
        None => println!("nothing"),
    }
}

fn main(){
    let a:Option<i32> = Some(3);
    let b:Option<&str> = Some("hello");
    let c:Option<char> = Some('A');
    let d:Option<u32> = None;

    match_option(a);
    match_option(b);
    match_option(c);
    match_option(d);
}