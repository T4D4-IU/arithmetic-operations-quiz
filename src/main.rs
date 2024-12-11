use rand::Rng;

fn main() {
    let mut num_of_correct = 0; // 正解数を保持する変数
    while num_of_correct < 3 { // 正解数が3未満の間ループ

        // quiz_modeをランダムに１か２に設定
        let quiz_mode = rand::thread_rng().gen_range(1..=2);
        match quiz_mode {
            1 => loop { // 足し算
                let op1 = rand::thread_rng().gen_range(0..100);
                let op2 = rand::thread_rng().gen_range(0..100);
                println!("{}+{}=??", op1, op2);
                println!("??の値を入力してください：");
                let mut ans_input = String::new(); // userからの回答を保持する変数
                // 標準入力から1行取得し、ans_inputに格納
                std::io::stdin().read_line(&mut ans_input).unwrap();
                // ans_inputからtrim()で改行を取り除き、parse()で数値(u32)型に変換する
                let ans_input = ans_input.trim().parse::<u32>().ok();
                match ans_input {
                    Some(ans_input) => {
                        dbg!(ans_input);
                    }
                    None => {
                        println!("数値を入力してください");
                        continue;
                    }
                }
                dbg!(&ans_input); // ans_inputの中身を表示
                if dbg!(ans_input == Some(op1 + op2)) { // この部分がエラーになる
                    println!("正解です！");
                    num_of_correct += 1; // 正解数を1増やす
                    break; // 正解したらループを抜ける
                } else {
                    println!("不正解です！");
                }
            }
            2 => loop { // 引き算
                let op1 = rand::thread_rng().gen_range(0..100);
                let op2 = rand::thread_rng().gen_range(0..100);
                println!("{} - {} =??", op1, op2);
                println!("??の値を入力してください：");
                let mut ans_input = String::new();
                std::io::stdin().read_line(&mut ans_input).unwrap();
                let ans_input = ans_input.trim().parse::<i32>().ok();
                match ans_input {
                    Some(ans_input) => {
                        dbg!(ans_input);
                    }
                    None => {
                        println!("数値を入力してください");
                        continue;
                    }
                }
                dbg!(ans_input);
                if dbg!(ans_input == Some(op1 - op2)) {
                    println!("正解です！");
                    num_of_correct += 1;
                    break;
                } else {
                    println!("不正解です！");
                }
            }
            _ => unreachable!(),
        }
    }
    println!("Congratulations!");
}
