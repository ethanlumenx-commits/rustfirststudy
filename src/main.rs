use std::io;

fn add_src(mut name:String)-> String{
    name.push_str("iiii");
    name
}
fn s(){}

fn main(){
    let first = String::from("join");
    let second  = add_src(first);
    println!(",{second}");
}

