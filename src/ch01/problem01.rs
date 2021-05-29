use crate::problem::Problem;

struct Problem01;

impl Problem for Problem01 {
    type Args = String;

    type Result = String;

    fn answer(args: Self::Args) -> anyhow::Result<Self::Result> {
        let str: String = args
            .chars()
            .enumerate()
            .filter(|(index, _)| index % 2 == 1)
            .map(|(_, chr)| chr)
            .collect();

        Ok(str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let arg = "パタトクカシーー".into();
        assert_eq!("タクシー", Problem01::answer(arg)?);

        Ok(())
    }
}
