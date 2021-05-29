pub trait Problem {
    type Args;
    type Result;
    fn answer(args: Self::Args) -> anyhow::Result<Self::Result>;
}
