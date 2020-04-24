use chrono::{DateTime, Utc};
use log::{error, info, trace};
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::time::Duration;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Error);
    proxy_wasm::set_http_context(|context_id, _| -> Box<dyn HttpContext> {
        Box::new(HttpHeaders { context_id })
    });
}

struct HttpHeaders {
    context_id: u32,
}

impl Context for HttpHeaders {}

impl HttpContext for HttpHeaders {
    fn on_http_request_headers(&mut self, _: usize) -> Action {
        for (name, value) in &self.get_http_request_headers() {
            trace!("#{} -> {}: {}", self.context_id, name, value);
        }
        Action::Continue
    }

    fn on_http_response_headers(&mut self, _: usize) -> Action {
        for (name, value) in &self.get_http_response_headers() {
            trace!("#{} <- {}: {}", self.context_id, name, value);
        }

        let template = liquid::ParserBuilder::with_stdlib()
            .build()
            .unwrap()
            .parse("XX:: {{num | minus: 2}}")
            .unwrap();

        let globals = liquid::object!({
            "num": 10
        });

        self.set_http_response_header(
          "Liquid value",
          Some(&template.render(&globals).unwrap()));
        Action::Continue
    }

    fn on_log(&mut self) {
        trace!("#{} completed.", self.context_id);
    }
}

impl RootContext for HttpHeaders {
    fn on_vm_start(&mut self, _: usize) -> bool {
        error!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
        error!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
        error!("-------------------START-------------------------");
        error!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
        error!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");

        self.set_tick_period(Duration::from_secs(5));
        true
    }

    fn on_tick(&mut self) {
        let datetime: DateTime<Utc> = self.get_current_time().into();
        error!("##### -> Ticker time at:{}", datetime);
    }
}
