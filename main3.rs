use chrono::Local;

fn main() {
    let name = "Bhajan"; // change this to your name
    let now = Local::now();

    println!(
        "Hello {}, right now the time is {}",
        name,
        now.format("%Y-%m-%d %H:%M:%S")
    );
}
