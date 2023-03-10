use std::collections::BTreeMap;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{
    egui::{self, FontId},
    EguiContexts, EguiSettings,
};
use vinox_common::networking::protocol::NetworkIP;

use crate::states::components::{GameState, Menu};

pub fn configure_visuals(mut contexts: EguiContexts) {
    contexts.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

pub fn update_ui_scale_factor(
    keyboard_input: Res<Input<KeyCode>>,
    mut toggle_scale_factor: Local<Option<bool>>,
    mut egui_settings: ResMut<EguiSettings>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if keyboard_input.just_pressed(KeyCode::Slash) || toggle_scale_factor.is_none() {
        *toggle_scale_factor = Some(!toggle_scale_factor.unwrap_or(true));

        if let Ok(window) = windows.get_single() {
            let scale_factor = if toggle_scale_factor.unwrap() {
                1.0
            } else {
                1.0 / window.scale_factor()
            };
            egui_settings.scale_factor = scale_factor;
        }
    }
}

pub fn create_ui(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut ip_res: ResMut<NetworkIP>,
) {
    egui::SidePanel::left("menu_side_panel")
        .default_width(250.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.ctx().set_style(egui::Style {
                    text_styles: {
                        let mut texts = BTreeMap::new();
                        texts.insert(egui::style::TextStyle::Small, FontId::proportional(16.0));
                        texts.insert(egui::style::TextStyle::Body, FontId::proportional(16.0));
                        texts.insert(egui::style::TextStyle::Heading, FontId::proportional(36.0));
                        texts.insert(egui::style::TextStyle::Monospace, FontId::monospace(16.0));
                        texts.insert(egui::style::TextStyle::Button, FontId::proportional(26.0));
                        texts
                    },
                    ..Default::default()
                });
                ui.heading("Vinox");

                ui.allocate_space(egui::Vec2::new(1.0, 100.0));

                ui.horizontal(|ui| {
                    ui.label("IP: ");
                    ui.text_edit_singleline(&mut ip_res.0);
                });

                ui.allocate_space(egui::Vec2::new(1.0, 26.0));

                if ui.button("Start").clicked() {
                    commands.insert_resource(NextState(Some(GameState::Loading)));
                }

                ui.allocate_space(egui::Vec2::new(1.0, 100.0));
            });
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(egui::Hyperlink::from_label_and_url(
                    "made by vixeliz",
                    "https://github.com/vixeliz/vinox/",
                ));
            });
        });

    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        egui::warn_if_debug_build(ui);

        ui.separator();
    });
}

pub fn ui_events() {}

pub fn start(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), Menu));
}
