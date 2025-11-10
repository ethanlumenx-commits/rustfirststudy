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

fn sing_a_song(){
    let song = [
    "十二个打鼓的鼓手,",
    "十一个吹风笛的风笛手,",
    "十个跳跃的男人,",
    "九位跳舞的女士,",
    "八位挤奶的佣妇,",
    "七只游水的天鹅,",
    "六只生蛋的鹅,",
    "五只金戒指,",
    "四只鸣唱的鸟儿,",
    "三只法国母鸡,",
    "两只鸠,",
    "及一只站在梨树上的鹧鸪鸟"
    ];

    let song_len = [
        "一","二","三","四","五","六","七","八","九","十","十一","十二"
    ];

    for i in 0..song_len.len(){
        println!("在圣诞节的第{}天，我的真爱送我：",song_len[i]);
        let mut y:String = String::new();
        for j in 0..=i{
            y += song[j];
        }
        println!("{}", y);
    }
}

fn main(){
    sing_a_song();
}

