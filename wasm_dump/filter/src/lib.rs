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
        self.set_tick_period(Duration::from_secs(30));
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
            info!(
                "#Request HEADERS::{} -> {}: {}",
                self.context_id, name, value
            );
        }

        info!("ON HTTP REQUEST_HEADERS",);
        self.set_http_request_header("TEST", Some("foobar"));
        self.send_http_response(403, vec![], Some(b"Access forbidden.\n"));
        Action::Continue
    }

    fn on_log(&mut self) {
        info!("Request #{} completed.", self.context_id);
    }
}
