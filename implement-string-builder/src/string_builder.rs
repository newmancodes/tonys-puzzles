use std::usize;

pub struct StringBuilder {
    chars: Vec<char>,
    max_capacity: usize,
}

impl StringBuilder {
    pub fn new() -> Self {
        StringBuilder::with_capacity(16)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let chars = Vec::with_capacity(capacity);
        StringBuilder {
            chars,
            max_capacity: usize::MAX,
        }
    }

    pub fn with_value(value: &str) -> Self {
        let chars = value.chars().collect();
        StringBuilder {
            chars,
            max_capacity: usize::MAX,
        }
    }

    pub fn with_max_capacity(capacity: usize, max_capacity: usize) -> Self {
        if max_capacity < 1 {
            panic!("Max capacity should be at least one.");
        }

        if capacity > max_capacity {
            panic!("Capacity must not exceed max_capacity");
        }

        let chars = Vec::with_capacity(capacity);
        StringBuilder {
            chars,
            max_capacity,
        }
    }

    pub fn capacity(&self) -> usize {
        self.chars.capacity()
    }

    pub fn max_capacity(&self) -> usize {
        self.max_capacity
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

    #[test]
    fn with_max_capacity_has_max_capacity() {
        let capacity: usize = 16;
        let max_capacity: usize = 32;
        let sb = StringBuilder::with_max_capacity(capacity, max_capacity);
        assert_eq!(capacity, sb.capacity());
        assert_eq!(max_capacity, sb.max_capacity());
    }

    #[test]
    #[should_panic]
    fn max_capacity_can_not_be_zero() {
        let capacity: usize = 0;
        let max_capacity: usize = 0;
        let _ = StringBuilder::with_max_capacity(capacity, max_capacity);
    }

    #[test]
    #[should_panic]
    fn max_capacity_can_not_be_less_than_capacity() {
        let capacity: usize = 16;
        let max_capacity: usize = 15;
        let _ = StringBuilder::with_max_capacity(capacity, max_capacity);
    }
}