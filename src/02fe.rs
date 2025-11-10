use std::io;

fn input_num()->u32{
    loop {
        let mut choose_num = String::new();
        io::stdin().read_line(&mut choose_num).unwrap();
        let choose_num: u32 = match choose_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid number");
                continue;
            }
        };
        return  choose_num;
    }
}

fn fe(input_num:u32)->u32{
    if input_num == 0 {
        return 0;
    }else if input_num == 1 || input_num == 2 {
        return 1;
    }
    let mut a = 1;
    let mut b = 1;
    let mut result = 0;
    for _ in 3..=input_num {
        result = a + b;
        a = b;
        b = result;
    }
    result
}

fn main(){
    
    loop{
        println!("Please input a number to calculate its factorial:");
        let num = input_num();
        let result = fe(num);
        println!("The factorial of {} is {}", num, result);
    }
}

