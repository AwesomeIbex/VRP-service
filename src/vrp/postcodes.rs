use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonResultCode {
    pub admin_district: String,
    pub admin_county: String,
    pub admin_ward: String,
    pub parish: String,
    pub parliamentary_constituency: String,
    pub ccg: String,
    pub ccg_id: String,
    pub ced: String,
    pub nuts: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonResult {
    pub postcode: String,
    pub quality: i64,
    pub eastings: i64,
    pub northings: i64,
    pub country: String,
    pub nhs_ha: String,
    pub longitude: f64,
    pub latitude: f64,
    pub european_electoral_region: String,
    pub primary_care_trust: String,
    pub region: String,
    pub lsoa: String,
    pub msoa: String,
    pub incode: String,
    pub outcode: String,
    pub parliamentary_constituency: String,
    pub admin_district: String,
    pub parish: String,
    pub admin_county: Option<String>,
    pub admin_ward: String,
    pub ced: Option<String>,
    pub ccg: String,
    pub nuts: String,
    pub codes: JsonResultCode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Json {
    pub status: i64,
    #[serde(rename = "result")]
    pub result: JsonResult,
}
