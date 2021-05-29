use crate::problem::Problem;

struct Problem00 {}

impl Problem for Problem00 {
    type Args = String;
    type Result = String;

    fn answer(args: Self::Args) -> anyhow::Result<Self::Result> {
        Ok(args.chars().rev().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() -> anyhow::Result<()> {
        let arg = "stressed".into();

        assert_eq!("desserts", Problem00::answer(arg)?);
        Ok(())
    }
}
