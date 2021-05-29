use crate::problem::Problem;

struct Problem07;

struct Args {
    x: u64,
    y: String,
    z: f64,
}

impl Problem for Problem07 {
    type Args = Args;
    type Result = String;

    fn answer(args: Self::Args) -> anyhow::Result<Self::Result> {
        let result = format!("{}時の{}は{}", args.x, args.y, args.z);
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let x = 12;
        let y = "気温".into();
        let z = 22.4;

        let result = Problem07::answer(Args { x, y, z })?;
        assert_eq!("12時の気温は22.4", result);

        Ok(())
    }
}
