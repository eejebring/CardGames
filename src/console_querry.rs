use std::io::stdin;

struct querry<T> {
    question: str,
    fail: str,
}

pub fn read_line(output: &mut String) {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer);
    output.insert_str(0,buffer.trim_end());
}