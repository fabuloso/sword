use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ConsolidatedList {
    #[serde(rename = "INDIVIDUALS")]
    pub individuals: Individuals,
}

#[derive(Deserialize, Debug)]
pub struct Individuals {
    #[serde(rename = "INDIVIDUAL")]
    pub targets: Vec<Designation>,
}

#[derive(Deserialize, Debug)]
pub struct Designation {
    #[serde(rename = "FIRST_NAME")]
    pub first_name: String,
    #[serde(rename = "SECOND_NAME")]
    pub second_name: Option<String>,
    #[serde(rename = "THIRD_NAME")]
    pub third_name: Option<String>,
    #[serde(rename = "FOURTH_NAME")]
    pub fourth_name: Option<String>,
}
