use async_trait::async_trait;

#[async_trait]
pub trait DryRunner {
    fn set_dry_run_mode(&mut self, mode: bool);
}
