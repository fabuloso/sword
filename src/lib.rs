use crate::uk_sanctions_list::Designations;

pub mod uk_sanctions_list;

pub async fn read_uk_sanctions_list() -> anyhow::Result<()> {
    let sanctions_list = reqwest::get("https://assets.publishing.service.gov.uk/government/uploads/system/uploads/attachment_data/file/1071293/UK_Sanctions_List.xml")
    .await?
    .text()
    .await?;

    let sanctions: Designations = quick_xml::de::from_str(&sanctions_list)?;

    for designation in sanctions.designations {
        for name in designation.names.name {
            println!("{}", name.name6)
        }
    }

    Ok(())
}
