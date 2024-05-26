use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use std::{fs, time};

use crate::explorer::Explorer;
use crate::file_system;
use crate::search::Indexer;

use serde_json;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Fexplorer {
    #[serde(skip)] // This how you opt-out of serialization of a field
    explorer: Explorer,

    #[serde(skip)]
    is_first_iteration: bool,
}

impl Default for Fexplorer {
    fn default() -> Self {
        Self {
            explorer: Explorer::default(),
            is_first_iteration: true,
        }
    }
}

impl Fexplorer {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for Fexplorer {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.is_first_iteration {
            self.is_first_iteration = false;

            let indexer = Indexer::new(&PathBuf::from_str("/").unwrap());

            let now = time::SystemTime::now();

            let directories = indexer.index_directories().unwrap();
            let files = indexer.index_files().unwrap();
            let links = indexer.index_links().unwrap();

            let time_needed = now.elapsed().unwrap();

            let dirs_str = serde_json::to_string_pretty(&directories).unwrap();
            let files_str = serde_json::to_string_pretty(&files).unwrap();
            let links_str = serde_json::to_string_pretty(&links).unwrap();

            let mut dir_f = fs::File::create("dirs.json").unwrap();
            let mut file_f = fs::File::create("files.json").unwrap();
            let mut link_f = fs::File::create("links.json").unwrap();

            dir_f.write_all(dirs_str.as_bytes()).unwrap();
            file_f.write_all(files_str.as_bytes()).unwrap();
            link_f.write_all(links_str.as_bytes()).unwrap();

            println!("Secs: {}", time_needed.as_secs_f32(),);
            println!("Count: {}", directories.len() + files.len() + links.len());
        };

        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                if ui.button("<-").clicked() {
                    match self.explorer.set_to_parent() {
                        Ok(_) => (),
                        Err(_) => return,
                    };
                };

                ui.add_space(16.0);

                let mut path = file_system::get_string_from_path(self.explorer.get_path());
                let output = egui::text_edit::TextEdit::singleline(&mut path).show(ui); // TODO: Can not edit text field because it refreshes
                if output.response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    match self.explorer.set_path(&PathBuf::from("/")) {
                        // TODO: Use path that is typed
                        Ok(_) => (),
                        Err(e) => println!("{}", e),
                    };
                };
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut change_path = false;
                let mut rel_path: PathBuf = PathBuf::new();

                let (directories, files, links) = self.explorer.get_entries();
                for directory in directories {
                    let name = format!("[Directory] {}", file_system::get_path_name(directory));

                    if ui.button(name).clicked() {
                        change_path = true;
                        rel_path = file_system::get_rel_path(directory);
                        break;
                    }
                }

                for file in files {
                    let name = format!("[File] {}", file_system::get_path_name(file));

                    if ui.button(name).clicked() {
                        change_path = true;
                        rel_path = file_system::get_rel_path(&file);
                        break;
                    }
                }

                for link in links {
                    let name = format!("[Link] {}", file_system::get_path_name(link));

                    if ui.button(name).clicked() {
                        change_path = true;
                        rel_path = file_system::get_rel_path(&link);
                        break;
                    }
                }

                if change_path {
                    self.explorer.add_path(&rel_path).unwrap();
                };
            });
        });
    }
}
