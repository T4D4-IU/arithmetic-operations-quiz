fn main() {
    println!("1+1=??");
    println!("??の値を入力してください：");
    let mut ans_input = String::new(); // userからの回答を保持する変数
    // 標準入力から1行取得し、ans_inputに格納
    std::io::stdin().read_line(&mut ans_input).unwrap();
    // ans_inputからtrim()で改行を取り除き、parse()で数値(u32)型に変換する
    let ans_input = ans_input.trim().parse::<u32>().unwrap();
    dbg!(&ans_input); // ans_inputの中身を表示
    if dbg!(ans_input == 1 + 1) { // この部分がエラーになる
        println!("正解です！");
    } else {
        println!("不正解です！");
    }
}
