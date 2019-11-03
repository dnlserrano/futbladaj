use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(default = "default_pitch")]
    pub pitch: String,
    #[serde(default = "default_user_type")]
    pub user_type: String,
    #[serde(default = "default_bday_day")]
    pub bday_day: i32,
    #[serde(default = "default_bday_month")]
    pub bday_month: i32,
    #[serde(default = "default_bday_year")]
    pub bday_year: i32,
    pub day: i32,
    pub month: i32,
    pub year: i32,
    pub start_hour: i32,
    #[serde(default = "default_minute")]
    pub start_minute: i32,
    pub end_hour: i32,
    #[serde(default = "default_minute")]
    pub end_minute: i32,
    pub username: String,
    pub email: String,
    pub phone: String,
    pub fiscal_number: String,
    pub address: String,
    pub postcode: String,
    #[serde(default = "default_location")]
    pub location: String,
}

fn default_pitch() -> String { "123".to_string() }
fn default_user_type() -> String { "outros".to_string() }
fn default_bday_day() -> i32 { 1 }
fn default_bday_month() -> i32 { 1 }
fn default_bday_year() -> i32 { 1990 }
fn default_minute() -> i32 { 0 }
fn default_location() -> String { "Lisboa".to_string() }

impl Params {
    pub fn new(
        username: String,
        email: String,
        fiscal_number: String,
        phone: String,
        address: String,
        postcode: String,
        day: i32,
        month: i32,
        year: i32,
        start_hour: i32,
        end_hour: i32,
        ) -> Self {
        Params {
            username,
            email,
            fiscal_number,
            phone,
            address,
            postcode,
            day,
            month,
            year,
            start_hour,
            end_hour,
            start_minute: default_minute(),
            end_minute: default_minute(),
            pitch: default_pitch(),
            user_type: default_user_type(),
            bday_day: default_bday_day(),
            bday_month: default_bday_month(),
            bday_year: default_bday_year(),
            location: default_location(),
        }
    }
}
