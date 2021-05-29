use itertools::Itertools;

use crate::problem::Problem;

#[derive(Debug, Default)]
struct NgramWord {
    words: Vec<String>,
    n: u64,

    _current: u64,
}

impl NgramWord {
    fn new(words: Vec<String>, n: u64) -> Self {
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
struct NgramChar {
    text: String,
    n: u64,

    _current: u64,
}

impl NgramChar {
    fn new(text: String, n: u64) -> Self {
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

struct Problem05;

struct Args {
    text: String,
    n: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Result {
    pub ngram_char: Vec<String>,
    pub ngram_word: Vec<Vec<String>>,
}

impl Problem for Problem05 {
    type Args = Args;
    type Result = Result;

    fn answer(args: Self::Args) -> anyhow::Result<Self::Result> {
        let args = &args;
        let ngram_char = NgramChar::new(args.text.clone(), args.n).collect_vec();
        let ngram_word = NgramWord::new(
            args.text
                .split_ascii_whitespace()
                .map(String::from)
                .collect_vec(),
            args.n,
        )
        .collect_vec();

        Ok(Result {
            ngram_char,
            ngram_word,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let args = Args {
            text: "I am an NLPer".into(),
            n: 2,
        };

        let result = Problem05::answer(args)?;

        assert_eq!(
            vec!["I ", " a", "am", "m ", " a", "an", "n ", " N", "NL", "LP", "Pe", "er"],
            result.ngram_char
        );

        assert_eq!(
            vec![vec!["I", "am"], vec!["am", "an"], vec!["an", "NLPer"]],
            result.ngram_word
        );

        Ok(())
    }
}
