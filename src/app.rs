use eframe::egui;
use ehttp::Request;
use std::sync::{Arc, Mutex};

pub struct MyApp {
    response: Arc<Mutex<Option<String>>>,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            response: Arc::new(Mutex::new(None)),
        }
    }

    fn send_led_request(&self, url: &str) {
        let response_clone = Arc::clone(&self.response);

        let request = Request::get(url);
        ehttp::fetch(request, move |result| {
            let message = match result {
                Ok(response) => format!("Success: {}", response.status),
                Err(err) => format!("Error: {}", err),
            };
            *response_clone.lock().unwrap() = Some(message);
        });
    }

    fn turn_led_on(&self) {
        self.send_led_request("http://tw/led/on"); // Replace with your ESP32 IP
    }

    fn turn_led_off(&self) {
        self.send_led_request("http://tw/led/off"); // Replace with your ESP32 IP
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("LED Control");

            if ui.button("Turn ON").clicked() {
                self.turn_led_on();
            }

            if ui.button("Turn OFF").clicked() {
                self.turn_led_off();
            }

            match &*self.response.lock().unwrap() {
                Some(text) => ui.label(text),
                None => ui.label("Waiting for response..."),
            };
        });

        ctx.request_repaint(); // Ensure UI updates when response changes
    }
}
