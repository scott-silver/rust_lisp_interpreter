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
    fn basic_string() {
        assert_eq!(rep("abcABC123"), "abcABC123");
    }

    #[test]
    fn string_containing_spaces() {
        assert_eq!(rep("hello mal world"), "hello mal world");
    }

    #[test]
    fn string_containing_symbols() {
        assert_eq!(rep("[]{}\"'* ;:()"), "[]{}\"'* ;:()");
    }

    #[test]
    fn long_string() {
        let string = "hello world abcdefghijklmnopqrstuvwxyz ABCDEFGHIJKLMNOPQRSTUVWXYZ 0123456789 (;:() []{}\"'* ;:() []{}\"'* ;:() []{}\"'*)";
        assert_eq!(
            rep(string),
            string
        );
    }

    #[test]
    fn non_alphanumeric_characters() {
        assert_eq!(rep("!"), "!");
        assert_eq!(rep("&"), "&");
        assert_eq!(rep("+"), "+");
        assert_eq!(rep(","), ",");
        assert_eq!(rep("-"), "-");
        assert_eq!(rep("/"), "/");
        assert_eq!(rep("<"), "<");
        assert_eq!(rep("="), "=");
        assert_eq!(rep(">"), ">");
        assert_eq!(rep("?"), "?");
        assert_eq!(rep("@"), "@");
        assert_eq!(rep("^"), "^");
        assert_eq!(rep("_"), "_");
        assert_eq!(rep("`"), "`");
        assert_eq!(rep("~"), "~");
        assert_eq!(rep("#"), "#");
        assert_eq!(rep("$"), "$");
        assert_eq!(rep("%"), "%");
        assert_eq!(rep("."), ".");
        assert_eq!(rep("|"), "|");
    }
}
