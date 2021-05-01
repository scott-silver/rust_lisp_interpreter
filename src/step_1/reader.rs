use regex::Regex;

pub struct Reader {
    tokens: Vec<String>,
    position: usize,
}

impl Reader {
    pub fn new(tokens: Vec<String>) -> Reader {
        Reader {
            tokens,
            position: 0,
        }
    }

    pub fn next(&mut self) -> Option<&String> {
        if self.position < self.tokens.len() {
            let r = self.tokens.get(self.position);
            self.position = self.position + 1;
            r
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&String> {
        self.tokens.get(self.position)
    }
}

fn read_string(str: &str) {}

// https://github.com/kanaka/mal/blob/master/impls/rust/reader.rs#L32
fn tokenize(str: &str) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]+)"###
        )
        .unwrap();
    }

    let mut res = vec![];
    for cap in RE.captures_iter(str) {
        if cap[1].starts_with(";") {
            continue;
        }
        res.push(String::from(&cap[1]));
    }
    res
}
#[cfg(test)]
mod reader_tests {
    use super::*;

    #[test]
    fn peek() {
        // no longer necessary
        let a = String::from("a");
        let reader = Reader::new(vec![a, "b".to_string(), "c".to_string()]);
        assert_eq!(reader.peek(), Some(&String::from("a")));
    }

    #[test]
    fn next() {
        let v = vec![String::from("a"), String::from("b"), String::from("c")];
        let mut reader = Reader::new(v);
        let n = reader.next();
        // returns the first element
        assert_eq!(n, Some(&String::from("a")));
        // and increments the position
        assert_eq!(reader.peek(), Some(&String::from("b")));
    }
}
#[cfg(test)]
mod tokenize_tests {
    use super::*;

    #[test]
    fn tokenize_list() {
        let tokens = tokenize("(+ 1 2 3)");
        // println!("{:?}", &tokens);
        assert_eq!(&tokens[0], "(");
        assert_eq!(&tokens[1], "+");
        assert_eq!(&tokens[2], "1");
        assert_eq!(&tokens[3], "2");
        assert_eq!(&tokens[4], "3");
        assert_eq!(&tokens[5], ")");
    }
}
