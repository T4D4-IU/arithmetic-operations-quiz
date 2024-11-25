fn main() {
    println!("1+1=??");
    println!("??の値を入力してください：");
    let mut ans_input = String::new(); // userからの回答を保持する変数
    // 標準入力から1行取得し、ans_inputに格納
    std::io::stdin().read_line(&mut ans_input).unwrap();
    dbg!(&ans_input); // ans_inputの中身を表示
}
