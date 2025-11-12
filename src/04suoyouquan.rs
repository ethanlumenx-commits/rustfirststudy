use std::rc::Rc;

fn main(){
    let values = return_string1();
    println!("返回的值：{}",values);

    let values_2 = return_string2();
    println!("返回的值：{}",values_2);

    let values_3 = return_string3();
    println!("返回的值：{}",values_3);

    let mut values_4 = String::from("old value");
    return_string4(&mut values_4);
    println!("返回的值：{}",values_4);


   
}

// 返回一个指针计数器，创建时为1，clone为2，函数结束时1释放，返回2，2在上方函数体指针变为1，不用时被释放
fn return_string1()->Rc<String>{
    let s = Rc::new(String::from("返回指针计数器，结束后不再使用该变量指针计数器就归0了"));
    Rc::clone(&s)
}

// 返回一个静态字符串，永远存在
fn return_string2()->&'static str{
    "返回static字符串，全局静态函数"
}

// 返回所有权,只有当变量的 所有权没有被转移出去 时，Rust 才会在函数结束时自动调用 drop 销毁它（释放内存）
fn return_string3()-> String{
    let s = String::from("返回所有权,只有当变量的 所有权没有被转移出去 时，Rust 才会在函数结束时自动调用 drop 销毁它（释放内存）");
    s
}

// 对已有的可变参数进行修改并返回
fn return_string4(output:&mut String){
    output.replace_range(.., "返回修改后的值，两个。。表示所有");
}


