#[derive(Debug)]
pub struct SeqString {
    data: Vec<char>,
}

impl SeqString {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn from_str(s: &str) -> Self {
        let mut new_string = Self::new();
        for char in s.chars() {
            new_string.data.push(char);
        }
        new_string
    }

    pub fn copy(&self) -> SeqString {
        Self {
            data: self.data.clone(),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn equals(&self, other: &SeqString) -> bool {
        self.data == other.data
    }

    pub fn substring(&self, start: usize, len: usize) -> Option<SeqString> {
        if start + len > self.len() {
            return None;
        }
        let mut new_string = SeqString::new();
        new_string.data = self.data.iter().skip(start).take(len).copied().collect();

        Some(new_string)
    }

    pub fn index_kmp(&self, pattern: &SeqString) -> Option<usize> {
        fn build_next(p: &Vec<char>) -> Vec<usize> {
            let mut next = vec![0; p.len()];
            let mut j = 0;
            for i in 1..p.len() {
                while j > 0 && p[j] != p[i] {
                    j = next[j - 1];
                }
                if p[j] == p[i] {
                    j += 1;
                }
                next[i] = j;
            }
            next
        }
        let mut j = 0;
        let mut i = 0;
        let next = build_next(&pattern.data);
        let s = &self.data;
        let p = &pattern.data;
        while i < s.len() && j < p.len() {
            if s[i] == p[j] {
                i += 1;
                j += 1;
            } else if j > 0 {
                j = next[j - 1];
            } else {
                i += 1;
            }
        }
        if j == p.len() { Some(i - j) } else { None }
    }
}
