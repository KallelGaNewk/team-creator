pub trait StringExt {
    /// Return `default` if the string is empty, otherwise return the string as `&str`.
    fn as_str_or<'a>(&'a self, default: &'a str) -> &'a str;
}

impl StringExt for str {
    fn as_str_or<'a>(&'a self, default: &'a str) -> &'a str {
        if self.is_empty() { default } else { self }
    }
}

impl StringExt for String {
    fn as_str_or<'a>(&'a self, default: &'a str) -> &'a str {
        self.as_str().as_str_or(default)
    }
}
