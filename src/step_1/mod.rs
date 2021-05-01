mod reader;
use reader::Reader;

pub fn read(x: &str) -> &str {
    x
}

pub fn eval(x: &str) -> &str {
    x
}

pub fn print(x: &str) -> &str {
    x
}

pub fn rep(x: &str) -> &str {
    print(eval(read(x)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        assert_eq!(rep("1"), "1");
        assert_eq!(rep("7"), "7");
        assert_eq!(rep("   7"), "7");
        assert_eq!(rep("-123"), "-123");
    }

    #[test]
    fn symbols() {
        assert_eq!(rep("+"), "+");
        assert_eq!(rep("abc"), "abc");
        assert_eq!(rep("   abc"), "abc");
        assert_eq!(rep("abc5"), "abc5");
        assert_eq!(rep("abc-def"), "abc-def");
    }

    #[test]
    fn starting_with_dash() {
        assert_eq!(rep("-"), "-");
        assert_eq!(rep("-abc"), "-abc");
        assert_eq!(rep("->>"), "->>");
    }

    #[test]
    fn lists() {
        assert_eq!(rep("(+ 1 2)"), "(+ 1 2)");
        assert_eq!(rep("()"), "()");
        assert_eq!(rep("( )"), "()");
        assert_eq!(rep("(nil)"), "(nil)");
        assert_eq!(rep("((3 4))"), "((3 4))");
        assert_eq!(rep("(+ 1 (+ 2 3))"), "(+ 1 (+ 2 3))");
        assert_eq!(rep("( +   1    (+   2 3  )  )"), "(+ 1 (+ 2 3))");
        assert_eq!(rep("(* 1 2)"), "(* 1 2)");
        assert_eq!(rep("(** 1 2)"), "(** 1 2)");
        assert_eq!(rep("(* -3 6)"), "(* -3 6)");
        assert_eq!(rep("(())"), "(())");
    }

    #[test]
    fn commas_as_whitespace() {
        assert_eq!(rep("(1 2, 3,,,,,),,"), "(1 2 3)");
    }
}
