use async_trait::async_trait;

#[async_trait]
pub trait Crawler<O> {
    async fn get() -> anyhow::Result<O>;
}
