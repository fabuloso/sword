use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Designations {
    #[serde(rename = "Designation", default)]
    pub designations: Vec<Designation>
}

#[derive(Deserialize, Debug)]
pub struct Names {
    #[serde(rename = "Name", default)]
    pub name: Vec<Name>
}

#[derive(Deserialize, Debug)]
pub struct Designation {
    #[serde(rename = "Names")]
    pub names: Names
}

#[derive(Deserialize, Debug)]
pub struct Name {
    #[serde(rename = "Name6", default)]
    pub name6: String
}