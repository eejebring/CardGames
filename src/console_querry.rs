use std::io::stdin;
use std::str::FromStr;
use std::thread;
use std::thread::{sleep, Thread};
use std::time::Duration;

pub struct Query {
    pub question: &'static str,
    pub fail: &'static str
}

impl Query {
    pub fn querry(&self) -> String {

        println!("{}", self.question);
        let mut buffer = String::new();
        stdin().read_line(&mut buffer);

        return buffer;
    }

    fn failSafe(&self) -> String {
        println!("{}", self.fail);
        let mut buffer = String::new();
        stdin().read_line(&mut buffer);

        return buffer;
    }

    pub fn query_int(&self) -> i64 {
        let mut buffer = self.querry().trim_end().parse();

        loop {
            match buffer {
                Ok(n) => return buffer.unwrap(),
                Err(e) => buffer = self.failSafe().trim_end().parse()
            }
        }
    }
}


fn read_line(output: &mut String) {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer);
    output.insert_str(0,buffer.trim_end());
}