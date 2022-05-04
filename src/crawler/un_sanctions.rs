use crate::model::ConsolidatedList;

pub async fn read_un_sanctions_list() -> anyhow::Result<()> {
    println!("Start Fetching");
    let sanctions_list = fetch().await?;
    println!("Fetch Complete");

    let sanctions: ConsolidatedList = quick_xml::de::from_str(&sanctions_list)?;

    sanctions
        .individuals
        .targets
        .into_iter()
        .for_each(|t| println!("{:?}", t));

    Ok(())
}

async fn fetch() -> Result<String, anyhow::Error> {
    let sanctions_list =
        reqwest::get("https://scsanctions.un.org/resources/xml/en/consolidated.xml")
            .await?
            .text()
            .await?;
    Ok(sanctions_list)
}
