use std::io::stdin;
use std::str::FromStr;
use std::thread;
use std::thread::{sleep, Thread};
use std::time::Duration;

pub struct Query {
    pub question: String,
    pub fail: String
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

    pub fn query_uint(&self) -> usize {
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
    pub options: usize
}

impl OptionQuery {
    pub fn new(q: &str, o: Vec<String>) -> OptionQuery {
        let mut fullQuery = String::from(q);
        let mut num = 0;
        let count = o.len();
        for item in o {
            num += 1;
            fullQuery += &format!("\n  {}. {}.", num, item).clone();
        }
        OptionQuery {
            question: Query {
                question: fullQuery,
                fail: "Please enter a number.".to_string()
            },
            options: count
        }
    }

    pub fn query(&self) -> usize {
        let mut val = self.question.query_uint();
        loop {
            if val <= self.options && val > 0 { break }
            else { val = Query {
                question: "Enter a valid option.".to_string(),
                fail: "Please enter a number.".to_string()
            }.query_uint()

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