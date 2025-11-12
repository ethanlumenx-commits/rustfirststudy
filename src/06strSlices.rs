fn main(){
    let s = String::from("hello world");
    let w_slices = return_slices(&s);
    println!("{} 中从开始到第一次出现w的字符是 {}",s,w_slices);
 
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