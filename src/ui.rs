use crate::assets::MenuSprites;
use crate::networking::RoomNetworkSettings;
use crate::GameState;
use bevy::app::AppExit;
use bevy::prelude::{
    default, App, AssetServer, Bundle, Camera, Commands, Component, CursorIcon, Entity,
    EventWriter, In, Plugin, Query, Res, ResMut, RunCriteriaDescriptorCoercion, Windows,
};
use bevy_egui::egui::{Align2, Color32, FontData, FontDefinitions, FontFamily, FontId, Frame, RichText, Stroke, TextStyle};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use iyes_loopless::prelude::{AppLooplessStateExt, IntoConditionalSystem, NextState};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_enter_system(GameState::Menu,setup_ui)
            .add_system(main_menu_ui.run_in_state(GameState::Menu));
    }
}

#[inline]
fn small_button_font() -> TextStyle {
    TextStyle::Name("SmallButtonText".into())
}

#[inline]
fn heading3() -> TextStyle {
    TextStyle::Name("ContextHeading".into())
}

fn setup_ui(mut egui_context: ResMut<EguiContext>) {
    let mut fonts = FontDefinitions::default();
    let mut style = (*egui_context.ctx_mut().style()).clone();
    fonts.font_data.insert(
        "main_font".to_owned(),
        FontData::from_static(include_bytes!("../assets/Abaddon Bold.ttf")),
    ); // .ttf and .otf supported
    // Large button text:
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "main_font".to_owned());

    let (font_family, _) = fonts
        .families
        .get_key_value(&FontFamily::Proportional)
        .unwrap();
    let font_id = FontId {
        size: 24.0,
        family: font_family.clone(),
    };

    let small_button_font_id = FontId {
        size: 20.0,
        family: font_family.clone(),
    };

    egui_context.ctx_mut().set_fonts(fonts);
    style.text_styles.insert(TextStyle::Body, font_id.clone());
    style.text_styles.insert(TextStyle::Button, font_id.clone());
    style
        .text_styles
        .insert(small_button_font(), small_button_font_id.clone());

    egui_context.ctx_mut().set_style(style);
}

fn quit_game(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit);
}

pub fn main_menu_ui(
    mut egui_context: ResMut<EguiContext>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    menu_sprites: Res<MenuSprites>,
    cam_query: Query<(Entity, &Camera)>,
    mut exit: EventWriter<AppExit>,
    windows: Res<Windows>,
) {
    let wnd = windows.get_primary().unwrap();

    let my_frame = Frame {
        fill: Color32::from_rgba_unmultiplied(0, 0, 0, 255),
        stroke: Stroke::new(0., Color32::WHITE),
        ..default()
    };

    egui::Window::new("main_menu_window")
        .frame(my_frame)
        .anchor(
            Align2::CENTER_BOTTOM,
            egui::Vec2 {
                x: 0.0,
                y: -(wnd.height() / 2.7),
            },
        )
        .resizable(false)
        .collapsible(false)
        .title_bar(false)
        .show(egui_context.ctx_mut(), |ui| {
            // options below the main panel with system stuff
            ui.columns(2, |ui| {
                let menu_button =
                    ui[0].add_sized([80., 26.], egui::Button::new(RichText::new("QUIT")));
                if menu_button.clicked() {
                    quit_game(exit);
                };
                let menu_button =
                    ui[1].add_sized([80., 26.], egui::Button::new(RichText::new("PLAY")));
                if menu_button.clicked() {
                    commands.insert_resource(RoomNetworkSettings::testing_local());
                    commands.insert_resource(NextState(GameState::WaitingForPlayers));
                };
            });
        });
}
