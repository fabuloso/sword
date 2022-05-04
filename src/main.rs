use sword::crawler::{read_uk_sanctions_list, read_un_sanctions_list};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Start UK");
    read_uk_sanctions_list().await?;
    println!("Start UN");
    read_un_sanctions_list().await
}
