pub use serde::{Serialize, Deserialize};

use crate::params::Params;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    submitted: Submission,
    details: Details,
    op: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Details {
    sid: String,
    #[serde(with = "crate::string")]
    page_num: i32,
    #[serde(with = "crate::string")]
    page_count: i32,
    #[serde(with = "crate::string")]
    finished: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Submission {
    instalacoes: String,
    utente: String,
    dados_da_reserva: Booking,
    responsavel_pela_reserva: User,
    dados_para_faturacao: FiscalInfo,
}

#[derive(Serialize, Deserialize, Debug)]
struct Booking {
    modalidade: String,
    numero_de_participantes: String,
    continuidade: String,
    observacoes: String,
    data_da_reserva: Day,
    hora: Hours,
}

#[derive(Serialize, Deserialize, Debug)]
struct Day {
    #[serde(with = "crate::string")]
    day: i32,
    #[serde(with = "crate::string")]
    month: i32,
    #[serde(with = "crate::string")]
    year: i32
}

#[derive(Serialize, Deserialize, Debug)]
struct Hours {
    hora_de_inicio: Hour,
    hora_de_fim: Hour,
}

#[derive(Serialize, Deserialize, Debug)]
struct Hour {
    #[serde(with = "crate::string")]
    hour: i32,
    #[serde(with = "crate::string")]
    minute: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    nome_responsavel: String,
    e_mail_responsavel: String,
    telefone_responsavel: String,
    data_de_nascimento: Day,
}

#[derive(Serialize, Deserialize, Debug)]
struct FiscalInfo {
    usar_o_nome_do_responsavel: String,
    nome_completo: String,
    nr_utente: String,
    no_de_contribuinte: String,
    morada: String,
    codigo_postal: Postcode,
    user_o_telefone_do_responsavel: String,
    telefone: String,
    usar_o_e_mail_do_responsavel: String,
    e_mail: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Postcode {
    codigo_postal: String,
    localidade: String,
}

impl Request {
    pub fn new(params: &Params) -> Self {
        Self {
            submitted: Submission::new(params),
            details: Details::new(),
            op: "Enviar".to_string(),
        }
    }
}

impl Submission {
    fn new(params: &Params) -> Self {
        Self {
            instalacoes: params.pitch.to_owned(),
            utente: params.user_type.to_owned(),
            dados_da_reserva: Booking::new(params),
            responsavel_pela_reserva: User::new(params),
            dados_para_faturacao: FiscalInfo::new(params),
        }
    }
}

impl Booking {
    fn new(params: &Params) -> Self {
        Self {
            modalidade: "Futsal".to_string(),
            numero_de_participantes: "1P".to_string(),
            continuidade: "unica".to_string(),
            observacoes: "".to_string(),
            data_da_reserva: Day::new(
                params.day,
                params.month,
                params.year
                ),
            hora: Hours::new(params),
        }
    }
}

impl User {
    fn new(params: &Params) -> Self {
        let bday = Day::new(
            params.bday_day,
            params.bday_month,
            params.bday_year,
            );

        Self {
            nome_responsavel: params.username.to_owned(),
            e_mail_responsavel: params.email.to_owned(),
            telefone_responsavel: params.phone.to_owned(),
            data_de_nascimento: bday,
        }
    }
}

impl FiscalInfo {
    fn new(params: &Params) -> Self {
        let postcode = Postcode::new(params);

        Self {
            usar_o_nome_do_responsavel: "1".to_string(),
            nome_completo: params.username.to_owned(),
            nr_utente: "".to_string(),
            no_de_contribuinte: params.fiscal_number.to_owned(),
            morada: params.address.to_owned(),
            codigo_postal: postcode,
            user_o_telefone_do_responsavel: "1".to_string(),
            telefone: params.phone.to_owned(),
            usar_o_e_mail_do_responsavel: "1".to_string(),
            e_mail: params.email.to_owned(),
        }
    }
}

impl Postcode {
    fn new(params: &Params) -> Self {
        Self {
            localidade: params.location.to_owned(),
            codigo_postal: params.postcode.to_owned(),
        }
    }
}

impl Hours {
    fn new(params: &Params) -> Self {
        Self {
            hora_de_inicio: Hour {
                hour: params.start_hour,
                minute: params.start_minute
            },
            hora_de_fim: Hour {
                hour: params.end_hour,
                minute: params.end_minute
            }
        }
    }
}

impl Day {
    fn new(day: i32, month: i32, year: i32) -> Self {
        Self {
            day,
            month,
            year,
        }
    }
}

impl Details {
    fn new() -> Self {
        Self {
            sid: "".to_string(),
            page_num: 1,
            page_count: 1,
            finished: 0
        }
    }
}
