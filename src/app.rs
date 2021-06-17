use eframe::{
    egui::{self, Color32},
    epi,
};
use std::fs::remove_file;

trait ByteConverter {
    fn kilobytes(&self) -> usize;
    fn megabytes(&self) -> usize;
    fn gigabytes(&self) -> usize;
}

impl ByteConverter for i32 {
    fn kilobytes(&self) -> usize {
        return (self * 1024) as usize;
    }

    fn megabytes(&self) -> usize {
        return (self * 1024 * 1024) as usize;
    }

    fn gigabytes(&self) -> usize {
        return (self * 1024 * 1024 * 1024) as usize;
    }
}

mod benchfs {
    use std::time::{Duration, Instant};

    pub fn write_once(filename: &'static str, total_size: usize, buffer_size: usize) -> Duration {
        let mut buffer = vec![0_u8; buffer_size];

        for i in 1..total_size {
            if i % 1024 == 0 {
                buffer.push(b'\n');
            }
            buffer.push(b'a');
        }
        let now: Instant = Instant::now();
        std::fs::write(filename, buffer).expect("Couldn't write file");
        return now.elapsed();
    }

    pub fn read_once(filename: &'static str) -> Duration {
        let now = Instant::now();
        std::fs::read_to_string(filename).expect("Couldn't read file");
        return now.elapsed();
    }
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct DiskTimerApp {
    total_size: i32,

    write_size: usize,
    write_time: f32,
    write_speed: f32,

    read_size: usize,
    read_time: f32,
    read_speed: f32,
}

impl Default for DiskTimerApp {
    fn default() -> Self {
        Self {
            total_size: 1,

            write_size: 0,
            write_time: 0.,
            write_speed: 0.,

            read_size: 0,
            read_time: 0.,
            read_speed: 0.,
        }
    }
}

impl epi::App for DiskTimerApp {
    fn name(&self) -> &str {
        "DiskTimer"
    }

    /// Called by the framework to load old app state (if any).
    #[cfg(feature = "persistence")]
    fn load(&mut self, storage: &dyn epi::Storage) {
        *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
    }

    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self {
            total_size,

            write_size,
            write_time,
            write_speed,

            read_size,
            read_time,
            read_speed,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("Stats");

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.colored_label(Color32::LIGHT_BLUE, "Writing");

                    ui.horizontal(|ui| {
                        ui.colored_label(Color32::GREEN, "Size");
                        ui.label(format!("{} MB", write_size));
                    });

                    ui.horizontal(|ui| {
                        ui.colored_label(Color32::GREEN, "Time");
                        ui.label(format!("{} s", write_time));
                    });

                    ui.horizontal(|ui| {
                        ui.colored_label(Color32::GREEN, "Speed");
                        ui.label(format!("{} MB/s", write_speed));
                    });
                });

                ui.vertical(|ui| {
                    ui.colored_label(Color32::LIGHT_BLUE, "Reading");

                    ui.horizontal(|ui| {
                        ui.colored_label(Color32::GREEN, "Size");
                        ui.label(format!("{} MB", read_size));
                    });

                    ui.horizontal(|ui| {
                        ui.colored_label(Color32::GREEN, "Time");
                        ui.label(format!("{} s", read_time));
                    });

                    ui.horizontal(|ui| {
                        ui.colored_label(Color32::GREEN, "Speed");
                        ui.label(format!("{} MB/s", read_speed));
                    });
                });
            });

            ui.add(egui::Slider::new(total_size, 1..=1000).text("Write size (MB)"));
            if ui.button("Start tests").clicked() {
                let size = total_size.clone().megabytes();
                let file_name = "testFile.txt";

                *write_time = benchfs::write_once(file_name, size, 8.kilobytes()).as_secs_f32();
                *write_size = total_size.clone() as usize;
                *write_speed = total_size.clone() as f32 / (write_time.clone() * 1.0);

                *read_time = benchfs::read_once(file_name).as_secs_f32();
                *read_size = total_size.clone() as usize;
                *read_speed = total_size.clone() as f32 / (read_time.clone() * 1.0);
                remove_file(file_name).unwrap();
            }
        });
    }
}
