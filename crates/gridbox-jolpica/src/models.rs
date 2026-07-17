use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RootResponse {
    #[serde(rename = "MRData")]
    pub mr_data: MrData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MrData {
    pub race_table: RaceTable,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RaceTable {
    #[serde(default)]
    pub races: Vec<RawRace>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawRace {
    pub season: String,
    pub round: String,
    #[serde(rename = "raceName")]
    pub race_name: String,
    #[serde(rename = "Circuit")]
    pub circuit: RawCircuit,
    pub date: String,
    pub time: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawCircuit {
    #[serde(rename = "circuitName")]
    pub circuit_name: String,
    #[serde(rename = "Location")]
    pub location: RawLocation,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawLocation {
    pub locality: String,
    pub country: String,
}
