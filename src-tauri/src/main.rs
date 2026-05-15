#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dnd_encounter_manager::application::services::encounter_service::EncounterService;
use eframe::egui;
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_title("D&D Encounter Manager"),
        ..Default::default()
    };

    eframe::run_native(
        "D&D Encounter Manager",
        options,
        Box::new(|_cc| Ok(Box::new(EncounterApp::default()))),
    )
}

struct EncounterApp {
    service: EncounterService,
    selected_entity_id: Option<String>,
}

impl Default for EncounterApp {
    fn default() -> Self {
        Self {
            service: EncounterService::new(),
            selected_entity_id: None,
        }
    }
}

impl eframe::App for EncounterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("initiative_panel").show(ctx, |ui| {
            ui.heading("Initiative Order");
            ui.separator();

            if ui.button("➕ Add Monster").clicked() {
                // TODO: Open dialog or use default
                let _ = self.service.add_monster("Goblin", 15);
            }

            if ui.button("➕ Add Player").clicked() {
                let _ = self.service.add_player("Hero", 14);
            }

            ui.separator();

            for entity in self.service.get_sorted_entities() {
                let is_selected = self.selected_entity_id.as_ref() == Some(&entity.instance_id);
                let label = format!("{}  (Init: {})  HP: {}", 
                    entity.display_name, 
                    entity.initiative,
                    entity.current_hp.unwrap_or(0)
                );

                if ui.selectable_label(is_selected, label).clicked() {
                    self.selected_entity_id = Some(entity.instance_id.clone());
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Stat Block / Details");

            if let Some(id) = &self.selected_entity_id {
                if let Some(entity) = self.service.get_entity(id) {
                    ui.label(format!("Name: {}", entity.display_name));
                    ui.label(format!("Initiative: {}", entity.initiative));
                    if let Some(hp) = entity.current_hp {
                        ui.label(format!("HP: {}", hp));
                    }
                    // More fields coming soon...
                }
            } else {
                ui.label("Select an entity from the left panel");
            }
        });

        // Bottom bar
        egui::TopBottomPanel::bottom("bottom_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("⏭️ Next Turn").clicked() {
                    self.service.advance_turn();
                }
                if ui.button("↩️ Undo").clicked() {
                    let _ = self.service.undo();
                }
            });
        });
    }
}