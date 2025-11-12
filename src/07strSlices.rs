
fn main(){
    let s = String::from("hello world");
    let w_slices = return_slices(&s);
    println!("{} 中从开始到第一次出现w的字符是 {}",s,w_slices);

    // 下面会导致修改s，导致 w_slices 失效，对数据的修改会使所有指向该数据的不可变引用（&str）立刻失效
    // 修改后原状态被破坏，引用也就成了 “悬垂引用”（指向无效数据）。
    // 确保了w_slices始终指向s，安全性提高
    // s.clear();

 
}

//提取一段字符中符合条件的，返回字符切片，如果直接返回值，后续字符释放后再使用会错误
fn return_slices(s:&String)->&str{
    let bytes = s.as_bytes();
    // &item  返回的是字符下标数，&u8（ascii字节码，值可能是104等） 模式引用中的&是穿透引用，会穿透&u8，直接拿到值104，
    for(i,&item)in bytes.iter().enumerate(){
        if item == b'w'{
            return &s[..=i];
        }
    }
    &s[..]
}






