use std::{
    collections::HashSet,
    ops::{BitAnd, BitOr},
};

use crate::{ngram::NgramChar, problem::Problem};

struct Problem06;

struct Args {
    text1: String,
    text2: String,
}

#[derive(Debug)]
struct Result {
    x: HashSet<String>,
    y: HashSet<String>,
    union: HashSet<String>,
    product: HashSet<String>,
    difference: HashSet<String>,
}

impl Problem for Problem06 {
    type Args = Args;
    type Result = Result;

    fn answer(args: Self::Args) -> anyhow::Result<Self::Result> {
        let args = &args;
        let x: HashSet<_> = NgramChar::new(args.text1.clone(), 2).collect();
        let y: HashSet<_> = NgramChar::new(args.text2.clone(), 2).collect();

        let result = Result {
            x: x.clone(),
            y: y.clone(),
            union: x.bitor(&y),
            product: x.bitand(&y),
            difference: x.difference(&y).cloned().collect(),
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {

    use maplit::hashset;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let args = Args {
            text1: "paraparaparadise".into(),
            text2: "paragraph".into(),
        };

        let result = Problem06::answer(args)?;

        {
            let left = hashset! {
                "pa".into(),
                "ar".into(),
                "ra".into(),
                "ap".into(),
                "ad".into(),
                "di".into(),
                "is".into(),
                "se".into()
            };

            assert_eq!(&left, &result.x);
        };

        {
            let left = hashset! {
                "pa".into(),
                "ar".into(),
                "ra".into(),
                "ag".into(),
                "gr".into(),
                "ap".into(),
                "ph".into(),
            };

            assert_eq!(&left, &result.y);
        };

        // union
        assert_eq!(
            hashset! {
                "pa".into(),
                "ar".into(),
                "ra".into(),
                "ap".into(),
                "ad".into(),
                "di".into(),
                "is".into(),
                "se".into(),
                "ag".into(),
                "gr".into(),
                "ap".into(),
                "ph".into(),
            },
            result.union
        );

        // product
        assert_eq!(
            hashset! {
                "pa".into(),
                "ar".into(),
                "ra".into(),
                "ap".into()
            },
            result.product
        );

        // difference
        assert_eq!(
            hashset! {
                "ad".into(),
                "is".into(),
                "di".into(),
                "se".into(),
            },
            result.difference
        );

        assert_eq!(result.x.contains("se"), true);
        assert_eq!(result.y.contains("se"), false);

        Ok(())
    }
}
