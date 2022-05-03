use sword::read_uk_sanctions_list;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    read_uk_sanctions_list().await
}
