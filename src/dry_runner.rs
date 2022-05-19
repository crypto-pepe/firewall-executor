use async_trait::async_trait;

#[async_trait]
pub trait DryRunner {
    fn dry(&mut self, dry: bool);
}
