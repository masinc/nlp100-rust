use crate::problem::Problem;

struct Problem02;

impl Problem for Problem02 {
    type Args = (String, String);
    type Result = String;

    fn answer(args: Self::Args) -> anyhow::Result<Self::Result> {
        let result: String = args
            .0
            .chars()
            .zip(args.1.chars())
            .map(|(chr1, chr2)| chr1.to_string() + &chr2.to_string())
            .collect();

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let args = ("パトカー".into(), "タクシー".into());

        let result = Problem02::answer(args)?;
        assert_eq!("パタトクカシーー", result);

        Ok(())
    }
}
