use crate::step_1::{MalAtom, MalDataType};
use regex::Regex;
use std::io::{self, Write};

// https://github.com/kanaka/mal/blob/master/impls/rust/reader.rs#L32
fn tokenize(str: String) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]+)"###
        )
        .unwrap();
    }

    let mut res = vec![];
    for cap in RE.captures_iter(&str) {
        if cap[1].starts_with(";") {
            continue;
        }
        res.push(String::from(&cap[1]));
    }
    res
}

fn consume_atom(atom: String) -> MalAtom {
    if Regex::new(r"^\d+$").unwrap().is_match(&atom) {
        MalAtom::Number(atom.parse::<isize>().unwrap())
    } else {
        MalAtom::Symbol(atom.to_string())
    }
}

fn consume_list(
    tokens: Vec<String>,
    accumulated_list: Vec<MalDataType>,
) -> (Vec<String>, MalDataType) {
    let first_token = &tokens[0];
    // TODO: panic if first_token is nil

    match first_token.as_str() {
        ")" => (
            tokens[1..].to_vec(),
            MalDataType::List(accumulated_list.to_vec()),
        ),
        "(" => {
            let (remaining_tokens, sub_accumulated_list) =
                consume_list(tokens[1..].to_vec(), vec![]);
            let mut new_vec = accumulated_list.to_vec();
            new_vec.push(sub_accumulated_list);
            consume_list(remaining_tokens, new_vec)
        }
        _ => {
            let mut new_vec = accumulated_list.to_vec();
            new_vec.push(MalDataType::Atom(consume_atom(first_token.to_string())));
            consume_list(tokens[1..].to_vec(), new_vec)
        }
    }
}

fn consume_tokens(tokens: Vec<String>) -> MalDataType {
    // panic if tokens is empty?
    let first_token = &tokens[0];

    match first_token.as_str() {
        ")" => panic!("unexpected close-parens"),
        "(" => {
            let (_remaining_tokens, accumulated_list) = consume_list(tokens[1..].to_vec(), vec![]);
            accumulated_list
        }
        _ => MalDataType::Atom(consume_atom(first_token.to_string())),
    }
}

pub fn read_string(x: String) -> MalDataType {
    consume_tokens(tokenize(x))
}
