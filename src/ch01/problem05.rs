use itertools::Itertools;

use crate::{
    ngram::{NgramChar, NgramWord},
    problem::Problem,
};

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
