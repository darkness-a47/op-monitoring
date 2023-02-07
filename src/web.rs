pub mod web {
    use actix_web::{get, Responder};
    use askama_actix::Template;
    use crate::gpio::gpio::{GpioStatus, read_status};


    #[derive(Template)]
    #[template(path = "index.html")]
    pub struct Status {
        state: Vec<GpioStatus>
    }

    #[get("/")]
    pub async fn handler() -> impl Responder {
        let gpio_status = read_status();
        let status = Status {
            state: gpio_status
        };

        status
    }
}
