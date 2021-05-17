pub fn read(x: String) -> String {
    x
}

pub fn eval(x: String) -> String {
    x
}

pub fn print(x: String) -> String {
    x
}

pub fn rep(x: String) -> String {
    print(eval(read(x)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_string() {
        assert_eq!(rep("abcABC123".to_string()), "abcABC123");
    }

    #[test]
    fn string_containing_spaces() {
        assert_eq!(rep("hello mal world".to_string()), "hello mal world");
    }

    #[test]
    fn string_containing_symbols() {
        assert_eq!(rep("[]{}\"'* ;:()".to_string()), "[]{}\"'* ;:()");
    }

    #[test]
    fn long_string() {
        let string = "hello world abcdefghijklmnopqrstuvwxyz ABCDEFGHIJKLMNOPQRSTUVWXYZ 0123456789 (;:() []{}\"'* ;:() []{}\"'* ;:() []{}\"'*)".to_string();
        assert_eq!(
            rep(string.clone()),
            string
        );
    }

    #[test]
    fn non_alphanumeric_characters() {
        assert_eq!(rep("!".to_string()), "!");
        assert_eq!(rep("&".to_string()), "&");
        assert_eq!(rep("+".to_string()), "+");
        assert_eq!(rep(",".to_string()), ",");
        assert_eq!(rep("-".to_string()), "-");
        assert_eq!(rep("/".to_string()), "/");
        assert_eq!(rep("<".to_string()), "<");
        assert_eq!(rep("=".to_string()), "=");
        assert_eq!(rep(">".to_string()), ">");
        assert_eq!(rep("?".to_string()), "?");
        assert_eq!(rep("@".to_string()), "@");
        assert_eq!(rep("^".to_string()), "^");
        assert_eq!(rep("_".to_string()), "_");
        assert_eq!(rep("`".to_string()), "`");
        assert_eq!(rep("~".to_string()), "~");
        assert_eq!(rep("#".to_string()), "#");
        assert_eq!(rep("$".to_string()), "$");
        assert_eq!(rep("%".to_string()), "%");
        assert_eq!(rep(".".to_string()), ".");
        assert_eq!(rep("|".to_string()), "|");
    }
}
