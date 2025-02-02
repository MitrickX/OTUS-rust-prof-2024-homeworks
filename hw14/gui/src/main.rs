use eframe::egui;
use tcp_smart_devices::TcpSmartSocketClient;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "TCP Smart Socket Application",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),
    )
}

const ADDR: &str = "127.0.0.1:55331";

struct MyApp {
    client: TcpSmartSocketClient,
}

impl MyApp {
    pub fn new() -> Self {
        Self {
            client: TcpSmartSocketClient::new(ADDR).unwrap(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.client.get_info().unwrap().split('\n').for_each(|s| {
                ui.heading(s.trim().to_owned());
            });

            if self.client.is_on().unwrap() && ui.button("Turn off").clicked() {
                self.client.turn_off().unwrap();

                return;
            }

            if !self.client.is_on().unwrap() && ui.button("Turn on").clicked() {
                self.client.turn_on().unwrap();
            }
        });
    }
}
