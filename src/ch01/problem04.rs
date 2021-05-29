use std::collections::HashMap;

use crate::problem::Problem;

struct Problem04;

struct Args {
    string: String,
    one_word_of_indexes: Vec<u64>,
}

impl Problem for Problem04 {
    type Args = Args;
    type Result = HashMap<String, u64>;
    fn answer(args: Self::Args) -> anyhow::Result<Self::Result> {
        let words = args
            .string
            .split_ascii_whitespace()
            .into_iter()
            .map(String::from);

        let result: HashMap<String, u64> = words
            .enumerate()
            .map(|(index, word)| {
                let index = index as u64 + 1;
                let word: String = if args.one_word_of_indexes.contains(&index) {
                    word.chars().take(1).collect()
                } else {
                    word.chars().take(2).collect()
                };

                (word, index)
            })
            .collect();

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let args = Args {
            string: "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.".into(),
            one_word_of_indexes: vec![1, 5, 6, 7, 8, 9, 15, 16, 19],
        };

        let result = Problem04::answer(args)?;

        assert_eq!(
            hashmap! {
                "H".into() => 1,
                "He".into() => 2,
                "Li".into() => 3,
                "Be".into() => 4,
                "B".into() => 5,
                "C".into() => 6,
                "N".into() => 7,
                "O".into() => 8,
                "F".into() => 9,
                "Ne".into() => 10,
                "Na".into() => 11,
                "Mi".into() => 12,
                "Al".into() => 13,
                "Si".into() => 14,
                "P".into() => 15,
                "S".into() => 16,
                "Cl".into() => 17,
                "Ar".into() => 18,
                "K".into() => 19,
                "Ca".into() => 20
            },
            result
        );

        Ok(())
    }
}
