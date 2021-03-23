pub struct StringBuilder {
    chars: Vec<char>,
}

impl StringBuilder {
    pub fn new() -> Self {
        StringBuilder::with_capacity(16)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let chars = Vec::with_capacity(capacity);
        StringBuilder {
            chars
        }
    }

    pub fn with_value(value: &str) -> Self {
        let chars = value.chars().collect();
        StringBuilder {
            chars
        }
    }

    pub fn capacity(&self) -> usize {
        self.chars.capacity()
    }

    pub fn append(&mut self, value: &str) {
        for char in value.chars() {
            self.chars.push(char);
        }
    }
}

impl From<StringBuilder> for String {
    fn from(sb: StringBuilder) -> Self {
        sb.chars.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stringbuilder_has_capacity_16() {
        let sb = StringBuilder::new();
        assert_eq!(16, sb.capacity());
    }

    #[test]
    fn with_capacity_stringbuilder_has_capacity() {
        let capacity = 512;
        let sb = StringBuilder::with_capacity(capacity);
        assert_eq!(capacity, sb.capacity());
    }

    #[test]
    fn with_value_stringbuilder_has_value() {
        let value = String::from("some_value");
        let sb = StringBuilder::with_value(&value);
        assert_eq!(value, String::from(sb));
    }
}