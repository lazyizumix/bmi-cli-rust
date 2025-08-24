use std::io;

fn main() {
    println!("身長(m)を入力してください: 例)1.71");
    let height = get_line();

    println!("体重(kg)を入力してください: 例)65.5");
    let weight = get_line();

    println!("身長: {:.2}m", height);
    println!("体重: {:.1}kg", weight);

    let bmi = get_bmi(height, weight);
    println!("BMI: {:.2}", bmi);
    println!("判定: {}", classify_bmi(bmi));
}

fn get_line() -> f64 {
    loop {
        let mut line = String::new();

        io::stdin().read_line(&mut line).expect("入力エラー");

        match line.trim().parse() {
            Ok(num) => {
                if num > 0.0 {
                    break num;
                } else {
                    println!("0より大きい数を入力してください");
                    continue;
                }
            }
            Err(_) => {
                println!("数値を入力してください！");
                continue;
            }
        }
    }
}

fn get_bmi(height: f64, weight: f64) -> f64 {
    weight / (height * height)
}

fn classify_bmi(bmi: f64) -> &'static str {
    match bmi {
        0.0..18.5 => "低体重",
        18.5..25.0 => "普通体重",
        25.0..30.0 => "肥満(1度)",
        30.0..35.0 => "肥満(2度)",
        35.0..40.0 => "肥満(3度)",
        _ => "肥満(4度)",
    }
}
