use chrono::{DateTime, Utc};
use log::{info, trace};
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::time::Duration;

#[no_mangle]
pub fn _start() {
    // Enable log level
    proxy_wasm::set_log_level(LogLevel::Trace);

    // RootContext this context is the main one for Envoy config
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(RootConfig) });

    // This is the context call for request
    proxy_wasm::set_http_context(|context_id, _| -> Box<dyn HttpContext> {
        Box::new(HttpHeaders { context_id })
    });
}

struct RootConfig;

impl Context for RootConfig {}

impl RootContext for RootConfig {
    fn on_vm_start(&mut self, _: usize) -> bool {
        info!("Config:: VM Started correctly!");
        self.set_tick_period(Duration::from_secs(60));
        true
    }

    fn on_tick(&mut self) {
        let datetime: DateTime<Utc> = self.get_current_time().into();
        info!("Config:: New Tick at '{}'", datetime);
    }
}

struct HttpHeaders {
    context_id: u32,
}

impl Context for HttpHeaders {}

impl HttpContext for HttpHeaders {
    fn on_http_request_headers(&mut self, _: usize) -> Action {
        info!("Request headers phase here!");
        for (name, value) in &self.get_http_request_headers() {
            trace!(
                "#Request HEADERS::{} -> {}: {}",
                self.context_id,
                name,
                value
            );
        }
        self.set_http_request_header("TEST", Some("foobar"));
        Action::Continue
    }

    fn on_http_response_headers(&mut self, _: usize) -> Action {
        for (name, value) in &self.get_http_response_headers() {
            trace!(
                "#Response HEADERS::{} <- {}: {}",
                self.context_id,
                name,
                value
            );
        }

        // @TODO: Make this liquid logic in a new class
        let template = liquid::ParserBuilder::with_stdlib()
            .build()
            .unwrap()
            .parse("XX:: {{num | minus: 2}}")
            .unwrap();

        let globals = liquid::object!({
            "num": 10
        });

        self.set_http_response_header("Liquid-value", Some(&template.render(&globals).unwrap()));
        Action::Continue
    }

    fn on_log(&mut self) {
        info!("Request #{} completed.", self.context_id);
    }
}
