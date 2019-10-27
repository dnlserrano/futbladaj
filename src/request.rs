pub use serde::{Serialize, Deserialize};

use crate::params::Params;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request<'a> {
    submitted: Submission<'a>,
    details: Details<'a>,
    op: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
struct Details<'a> {
    sid: &'a str,
    #[serde(with = "crate::string")]
    page_num: i32,
    #[serde(with = "crate::string")]
    page_count: i32,
    #[serde(with = "crate::string")]
    finished: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Submission<'a> {
    instalacoes: &'a str,
    utente: &'a str,
    dados_da_reserva: Booking<'a>,
    responsavel_pela_reserva: User<'a>,
    dados_para_faturacao: FiscalInfo<'a>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Booking<'a> {
    modalidade: &'a str,
    numero_de_participantes: &'a str,
    continuidade: &'a str,
    observacoes: &'a str,
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
struct User<'a> {
    nome_responsavel: &'a str,
    e_mail_responsavel: &'a str,
    telefone_responsavel: &'a str,
    data_de_nascimento: Day,
}

#[derive(Serialize, Deserialize, Debug)]
struct FiscalInfo<'a> {
    usar_o_nome_do_responsavel: &'a str,
    nome_completo: &'a str,
    nr_utente: &'a str,
    no_de_contribuinte: &'a str,
    morada: &'a str,
    codigo_postal: Postcode<'a>,
    user_o_telefone_do_responsavel: &'a str,
    telefone: &'a str,
    usar_o_e_mail_do_responsavel: &'a str,
    e_mail: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
struct Postcode<'a> {
    codigo_postal: &'a str,
    localidade: &'a str,
}

impl<'a> Request<'a> {
    pub fn new(params: &'a Params) -> Self {
        Self {
            submitted: Submission::new(params),
            details: Details::new(),
            op: "Enviar",
        }
    }
}

impl<'a> Submission<'a> {
    fn new(params: &'a Params) -> Self {
        Self {
            instalacoes: &params.pitch,
            utente: &params.user_type,
            dados_da_reserva: Booking::new(params),
            responsavel_pela_reserva: User::new(params),
            dados_para_faturacao: FiscalInfo::new(params),
        }
    }
}

impl<'a> Booking<'a> {
    fn new(params: &'a Params) -> Self {
        Self {
            modalidade: "Futsal",
            numero_de_participantes: "1P",
            continuidade: "unica",
            observacoes: "",
            data_da_reserva: Day::new(
                params.day,
                params.month,
                params.year
                ),
            hora: Hours::new(params),
        }
    }
}

impl<'a> User<'a> {
    fn new(params: &'a Params) -> Self {
        let bday = Day::new(
            params.bday_day,
            params.bday_month,
            params.bday_year,
            );

        Self {
            nome_responsavel: &params.username,
            e_mail_responsavel: &params.email,
            telefone_responsavel: &params.phone,
            data_de_nascimento: bday,
        }
    }
}

impl<'a> FiscalInfo<'a> {
    fn new(params: &'a Params) -> Self {
        let postcode = Postcode::new(params);

        Self {
            usar_o_nome_do_responsavel: "1",
            nome_completo: &params.username,
            nr_utente: "",
            no_de_contribuinte: &params.fiscal_number,
            morada: &params.address,
            codigo_postal: postcode,
            user_o_telefone_do_responsavel: "1",
            telefone: &params.phone,
            usar_o_e_mail_do_responsavel: "1",
            e_mail: &params.email,
        }
    }
}

impl<'a> Postcode<'a> {
    fn new(params: &'a Params) -> Self {
        Self {
            localidade: &params.location,
            codigo_postal: &params.postcode,
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

impl<'a> Details<'a> {
    fn new() -> Self {
        Self {
            sid: "",
            page_num: 1,
            page_count: 1,
            finished: 0
        }
    }
}
