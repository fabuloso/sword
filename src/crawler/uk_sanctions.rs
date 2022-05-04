use crate::model::ArrayOfFinancialSanctionsTarget;

pub async fn read_uk_sanctions_list() -> anyhow::Result<()> {
    println!("Start Fetching");
    let sanctions_list = fetch().await?;
    println!("Fetch Complete");

    let sanctions: ArrayOfFinancialSanctionsTarget = quick_xml::de::from_str(&sanctions_list)?;

    sanctions
        .target
        .into_iter()
        .filter(|t| t.group_type != "Individual")
        .for_each(|t| println!("{:?}", t));

    Ok(())
}

async fn fetch() -> Result<String, anyhow::Error> {
    let sanctions_list = reqwest::get(
        "https://ofsistorage.blob.core.windows.net/publishlive/2022format/ConList.xml",
    )
    .await?
    .text()
    .await?;
    Ok(sanctions_list)
}
