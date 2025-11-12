
fn main(){
    // 在不使用mut对一个字符进行修改并返回新值并赋值
    let name = vec![String::from("my_all_name")];
    let frist  = &name[0];
    println!("frist is {}",frist);
    println!("all name is {:?}",name);
    let full = copy_write(&name);
    println!("full is {}",full);

    let full2 = copy_write2(&name);
    println!("full2 is {}",full2);

}

// 复制一份再修改，缺点是浪费了一份内存
fn copy_write(name:&Vec<String>)->String{
    let mut name_clone = name.clone();
    name_clone.push(String::from("add push"));
    let full = name_clone.join(" ");
    full
}

fn copy_write2(name:&Vec<String>)->String{
    // 复制 Vec<String> 中所有字符串的值，并拼接成一个新的 String 赋值给 full
    let mut full = name.join(" ");
    full.push_str("esq");
    full
}




