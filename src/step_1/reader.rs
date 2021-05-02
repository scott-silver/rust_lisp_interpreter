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

    pub fn next(&mut self) -> &String {
        &self.tokens[self.position]
    }

    pub fn peek(&self) -> Option<&String> {
        self.tokens.get(self.position)
    }
}

fn read_string(str: &str) -> MalDataType {
    let reader = Reader::new(tokenize(str));
    read_form(reader)
}

fn read_form(reader: Reader) -> MalDataType {
    let first_token = reader.peek();
    match first_token {
        // Some("(") => read_list(reader),
        Some(_) => read_atom(reader),
        None => MalDataType::None,
    }
}

fn read_list(reader: Reader) {}

fn read_atom(mut reader: Reader) -> MalDataType {
    let first_token = reader.next();
    if Regex::new(r"^\d+$").unwrap().is_match(&first_token) {
        MalDataType::Number(first_token.parse::<isize>().unwrap())
    } else {
        // should this be a clone?
        MalDataType::Symbol(first_token.clone())
    }
}

#[derive(Debug, PartialEq)]
enum MalDataType {
    Number(isize),
    Symbol(String),
    None
}

#[cfg(test)]
mod read_atom_tests {
    use super::*;

    #[test]
    fn returns_numbers() {
        let reader = Reader::new(vec![String::from("1")]);
        assert_eq!(read_atom(reader), MalDataType::Number(1));
    }

    #[test]
    fn returns_symbols() {
        let reader = Reader::new(vec![String::from("+")]);
        assert_eq!(read_atom(reader), MalDataType::Symbol(String::from("+")));
    }
}

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
        assert_eq!(n, &String::from("a"));
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
