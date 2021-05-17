mod reader;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum MalDataType {
    Atom(MalAtom),
    List(Vec<MalDataType>),
}

// delete in favor of printer function
impl fmt::Display for MalDataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MalAtom {
    Number(isize),
    Symbol(String),
}

pub fn read(x: String) -> MalDataType {
    reader::read_string(x)
}

pub fn eval(x: MalDataType) -> MalDataType {
    x
}

pub fn print(x: MalDataType) -> String {
    x.to_string()
}

pub fn rep(x: String) -> String {
    print(eval(read(x)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        assert_eq!(rep("1".to_string()), "1");
        assert_eq!(rep("7".to_string()), "7");
        assert_eq!(rep("   7".to_string()), "7");
        assert_eq!(rep("-123".to_string()), "-123");
    }

    #[test]
    fn symbols() {
        assert_eq!(rep("+".to_string()), "+");
        assert_eq!(rep("abc".to_string()), "abc");
        assert_eq!(rep("   abc".to_string()), "abc");
        assert_eq!(rep("abc5".to_string()), "abc5");
        assert_eq!(rep("abc-def".to_string()), "abc-def");
    }

    #[test]
    fn starting_with_dash() {
        assert_eq!(rep("-".to_string()), "-");
        assert_eq!(rep("-abc".to_string()), "-abc");
        assert_eq!(rep("->>".to_string()), "->>");
    }

    #[test]
    fn lists() {
        assert_eq!(rep("(+ 1 2)".to_string()), "(+ 1 2)");
        assert_eq!(rep("()".to_string()), "()");
        assert_eq!(rep("( )".to_string()), "()");
        assert_eq!(rep("(nil)".to_string()), "(nil)");
        assert_eq!(rep("((3 4))".to_string()), "((3 4))");
        assert_eq!(rep("(+ 1 (+ 2 3))".to_string()), "(+ 1 (+ 2 3))");
        assert_eq!(rep("( +   1    (+   2 3  )  )".to_string()), "(+ 1 (+ 2 3))");
        assert_eq!(rep("(* 1 2)".to_string()), "(* 1 2)");
        assert_eq!(rep("(** 1 2)".to_string()), "(** 1 2)");
        assert_eq!(rep("(* -3 6)".to_string()), "(* -3 6)");
        assert_eq!(rep("(())".to_string()), "(())");
    }

    #[test]
    fn commas_as_whitespace() {
        assert_eq!(rep("(1 2, 3,,,,,),,".to_string()), "(1 2 3)");
    }
}
