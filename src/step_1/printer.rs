use crate::step_1::{MalAtom, MalDataType};

pub fn print_string(x: MalDataType) -> String {
    match x {
        MalDataType::Atom(MalAtom::Number(x)) => x.to_string(),
        MalDataType::Atom(MalAtom::Symbol(x)) => x,
        MalDataType::List(l) => {
            let items: Vec<String> = l.into_iter().map(|x| print_string(x)).collect();
            format!("({})", items.join(" "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number() {
        assert_eq!(print_string(MalDataType::Atom(MalAtom::Number(1))), "1");
        assert_eq!(print_string(MalDataType::Atom(MalAtom::Number(500))), "500");
    }

    #[test]
    fn symbol() {
        assert_eq!(
            print_string(MalDataType::Atom(MalAtom::Symbol("+".to_string()))),
            "+"
        );
        assert_eq!(
            print_string(MalDataType::Atom(MalAtom::Symbol("-".to_string()))),
            "-"
        );
    }

    #[test]
    fn empty_list() {
        assert_eq!(print_string(MalDataType::List(vec![])), "()");
    }

    #[test]
    fn non_empty_list() {
        assert_eq!(
            print_string(MalDataType::List(vec![MalDataType::Atom(MalAtom::Symbol(
                "+".to_string()
            ))])),
            "(+)"
        );
        assert_eq!(
            print_string(MalDataType::List(vec![
                MalDataType::Atom(MalAtom::Symbol("+".to_string())),
                MalDataType::Atom(MalAtom::Symbol("1".to_string())),
                MalDataType::Atom(MalAtom::Symbol("2".to_string()))
            ])),
            "(+ 1 2)"
        );
        assert_eq!(
            print_string(MalDataType::List(vec![
                MalDataType::Atom(MalAtom::Symbol("+".to_string())),
                MalDataType::Atom(MalAtom::Symbol("1".to_string())),
                MalDataType::Atom(MalAtom::Symbol("2".to_string())),
                MalDataType::Atom(MalAtom::Symbol("3".to_string())),
                MalDataType::Atom(MalAtom::Symbol("4".to_string()))
            ])),
            "(+ 1 2 3 4)"
        );
    }

    #[test]
    fn nested_list() {
        assert_eq!(
            print_string(MalDataType::List(vec![
                MalDataType::Atom(MalAtom::Symbol("+".to_string())),
                MalDataType::Atom(MalAtom::Symbol("1".to_string())),
                MalDataType::Atom(MalAtom::Symbol("2".to_string())),
                MalDataType::List(vec![
                    MalDataType::Atom(MalAtom::Symbol("+".to_string())),
                    MalDataType::Atom(MalAtom::Symbol("3".to_string())),
                    MalDataType::Atom(MalAtom::Symbol("4".to_string()))
                ])
            ])),
            "(+ 1 2 (+ 3 4))"
        );
    }
}
