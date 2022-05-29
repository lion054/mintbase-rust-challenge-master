pub struct StringWrapper {
    data: String,
}

impl StringWrapper {
    pub fn from(value: &str) -> Self {
        Self {
            data: String::from(value),
        }
    }

    pub fn as_ref(&self) -> &str {
        return self.data.as_str().as_ref();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_it_compile() {
        assert_eq!(StringWrapper::from("string").as_ref(), "string");
    }
}
