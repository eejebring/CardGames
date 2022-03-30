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
    pub fn query(&self) -> String {

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

    pub fn query_int(&self) -> isize {
        let mut buffer = self.query().trim_end().parse();

        loop {
            match buffer {
                Ok(n) => return buffer.unwrap(),
                Err(e) => buffer = self.failSafe().trim_end().parse()
            }
        }
    }
}

pub struct OptionQuery {
    pub question: Query,
    pub options: vec<str>
}

impl OptionQuery {
    pub fn new(q: &str, o: vec<str>) -> OptionQuery {
        let mut fullQuery = String::from(q);
        let mut num = 0;
        for item in o {
            fullQuery += format!("\n  {}. {}.", num, item).clone();
        }
        OptionQuery {
            question: Query {
                question: q,
                fail: "Please enter a number."
            },
            options: o
        }
    }

    pub fn query(&self) -> isize {
        let x = self.options.iter();
        let mut val = self.question.query_int();
        loop {
            match val {
                _ if val < self.options.len => break,
                _ => {
                    val = Query {
                        question: "Enter a valid option.",
                        fail: "Please enter a number."
                    }.query_int()
                }
            }
        }
        return val;
    }
}


fn read_line(output: &mut String) {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer);
    output.insert_str(0,buffer.trim_end());
}