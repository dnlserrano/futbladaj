#[derive(Debug)]
pub struct Params<'a> {
    pub pitch: &'a str,
    pub day: i32,
    pub month: i32,
    pub year: i32,
    pub start_hour: i32,
    pub start_minute: i32,
    pub end_hour: i32,
    pub end_minute: i32,
    pub user_type: &'a str,
    pub username: &'a str,
    pub email: &'a str,
    pub phone: &'a str,
    pub bday_day: i32,
    pub bday_month: i32,
    pub bday_year: i32,
    pub fiscal_number: &'a str,
    pub address: &'a str,
    pub postcode: &'a str,
    pub location: &'a str,
}

impl<'a> Params<'a> {
    pub fn new(username: &'a str) -> Self {
        Params {
            pitch: "123",
            day: 1,
            month: 1,
            year: 2020,
            start_hour: 22,
            start_minute: 00,
            end_hour: 22,
            end_minute: 30,
            user_type: "outros",
            username: username,
            email: "email@email.com",
            phone: "91 123 12 12",
            bday_day: 1,
            bday_month: 1,
            bday_year: 1990,
            fiscal_number: "123123123",
            address: "Rua do Exemplo",
            postcode: "1234-123",
            location: "Lisboa",
        }
    }
}
