use uk_sanctions_list::ArrayOfFinancialSanctionsTarget;

pub mod uk_sanctions_list;

pub async fn read_uk_sanctions_list() -> anyhow::Result<()> {
    let sanctions_list = reqwest::get(
        "https://ofsistorage.blob.core.windows.net/publishlive/2022format/ConList.xml",
    )
    .await?
    .text()
    .await?;

    let sanctions: ArrayOfFinancialSanctionsTarget = quick_xml::de::from_str(&sanctions_list)?;

    sanctions
        .target
        .into_iter()
        .filter(|t| t.group_type != "Individual")
        .for_each(|t| println!("{:?}", t));

    Ok(())
}
