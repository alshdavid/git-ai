#[derive(Debug, clap::Parser)]
pub struct PullRequestCommand {}

pub fn main(_args: PullRequestCommand) -> anyhow::Result<()> {
    Ok(())
}
