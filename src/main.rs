use crate::console_querry::read_line;

mod console_querry;

fn main() {
    let mut x = &mut "".to_string();
    read_line(x);
    println!("{}",x);
}
