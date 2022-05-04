use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ArrayOfFinancialSanctionsTarget {
    #[serde(rename = "FinancialSanctionsTarget", default)]
    pub target: Vec<Designation>,
}

#[derive(Deserialize, Debug)]
pub struct Designation {
    #[serde(rename = "Name6")]
    pub name6: String,
    pub name1: String,
    pub name2: String,
    pub name3: String,
    pub name4: String,
    pub name5: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "OtherInformation")]
    pub details: String,
    #[serde(rename = "Individual_DateOfBirth")]
    pub date_of_birth: String,
}
