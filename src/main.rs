use std::io;

fn main() {
    // q: 標準入力を受け取るにはどうすればいいですか
    // a: std::io::stdin() を使う
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .ok()
        .expect("Failed to read line");

    println!("You guess: {}", guess);
}
