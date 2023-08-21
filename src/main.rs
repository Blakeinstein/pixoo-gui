#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::sync::Arc;

use divoom::{DivoomServiceClient, DivoomDeviceInfo};
use eframe::egui;
use tokio::runtime::{self, Runtime};
use flowync::{error::Cause, Flower};

type DeviceFlower = Flower<String, Vec<DivoomDeviceInfo>>;


struct PixooGui {
    runtime: Runtime,
    client: Arc<DivoomServiceClient>,
    device_flower: DeviceFlower,
    device_list: Vec<DivoomDeviceInfo>,
}

impl PixooGui {

    fn get_devices_handler(&self) {
        let handle = self.device_flower.handle();

        let client_ref = self.client.clone();

        self.runtime.spawn(async move {
            handle.activate();
            handle.set_result(client_ref.get_same_lan_devices().await.map_err(|e| e.into()));
        });
    }

    fn new() -> Self {
        let client = DivoomServiceClient::new();
        Self {
            runtime: runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
            client: Arc::from(client),
            device_flower: DeviceFlower::new(1),
            device_list: Vec::new(),
        }
    }
}

impl eframe::App for PixooGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.device_flower.result_is_ready() {
                self.device_flower
                    .try_result(|result| {
                        match result {
                            Ok(devices) => {
                                self.device_list = devices;
                            },
                            Err(Cause::Suppose(msg)) => {
                                println!("{}", msg)
                            }
                            Err(Cause::Panicked(msg)) => {
                                panic!("{}", msg)
                                // Handle things if stuff unexpectedly panicked at runtime.
                            }
                        }
                    });
            }

            if ui.button("Search").clicked() {
                self.get_devices_handler();
            }

            ui.separator();

            if self.device_flower.is_active() {
                ui.label("Loading…");
            } else {
                ui.label(format!("Found {} devices", self.device_list.len()));
                for device in &self.device_list {
                    ui.label(format!("Device: {} at {}", device.device_name, device.device_private_ip));
                }
            }
        });
    }

}

fn main() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native( 
        "Pixoo Gui",
        options,
        Box::new(|_ctx| Box::new(PixooGui::new())),
    ).unwrap();
}