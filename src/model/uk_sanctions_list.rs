use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ArrayOfFinancialSanctionsTarget {
    #[serde(rename = "FinancialSanctionsTarget", default)]
    pub target: Vec<Designation>,
}

#[derive(Deserialize, Debug)]
pub struct Designation {
    #[serde(rename = "GroupTypeDescription")]
    pub group_type: String,
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

impl Designation {
    fn get_name(&self) -> String {
        format!(
            "{} {} {} {} {} {} {}",
            self.title, self.name6, self.name1, self.name2, self.name3, self.name4, self.name5
        )
    }
}
