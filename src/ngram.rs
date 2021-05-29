use itertools::Itertools;
#[derive(Debug, Default)]
pub struct NgramWord {
    words: Vec<String>,
    n: u64,

    _current: u64,
}

impl NgramWord {
    pub fn new(words: Vec<String>, n: u64) -> Self {
        Self {
            words,
            n,
            ..Default::default()
        }
    }
}

impl Iterator for NgramWord {
    type Item = Vec<String>;
    fn next(&mut self) -> Option<Self::Item> {
        let len = self.words.len() as u64;
        if self._current + self.n > len {
            None
        } else {
            let item = self
                .words
                .iter()
                .skip(self._current as usize)
                .take(self.n as usize)
                .map(String::clone)
                .collect_vec();
            self._current += 1;
            Some(item)
        }
    }
}

#[derive(Debug, Default)]
pub struct NgramChar {
    text: String,
    n: u64,

    _current: u64,
}

impl NgramChar {
    pub fn new(text: String, n: u64) -> Self {
        Self {
            text,
            n,
            ..Default::default()
        }
    }
}

impl Iterator for NgramChar {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.text.len() as u64;

        if self._current + self.n > len {
            None
        } else {
            let item: String = self
                .text
                .chars()
                .skip(self._current as usize)
                .take(self.n as usize)
                .collect();
            self._current += 1;
            Some(item)
        }
    }
}
