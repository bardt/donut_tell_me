use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_ninepatch::*;

mod assets;
mod component;
mod system;

fn main() {
    static PHOTO: &str = "photo";
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        width: 720.,
        height: 590., // UI looks the best at this size
        title: "Donut Tell Me!".to_string(),
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .insert_resource(ClearColor(Color::hex("86c0d1").unwrap_or(Color::WHITE)))
    .add_plugin(JsonAssetPlugin::<assets::TextureAtlasData>::new(&[
        "atlas.json",
    ]))
    .add_plugin(NinePatchPlugin::<()>::default())
    .add_event::<component::PhotosTakenEvent>()
    .add_loading_state(
        LoadingState::new(AppState::AssetLoading)
            .continue_to_state(AppState::InGame)
            .with_collection::<assets::MyAssets>(),
    )
    .add_state(AppState::AssetLoading)
    .add_system_set(SystemSet::on_exit(AppState::AssetLoading).with_system(assets::init))
    .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(system::setup_game))
    .add_system_set(
        SystemSet::on_update(AppState::InGame)
            .label(PHOTO)
            .with_system(system::log_transaction),
    )
    .add_system_set(
        SystemSet::on_update(AppState::InGame)
            .after(PHOTO)
            .with_system(system::add_donut_sprites)
            .with_system(system::change_cooking_donut)
            .with_system(system::change_cooking_donut_buttons)
            .with_system(system::update_donut_sprites)
            .with_system(system::cook_another_donut)
            .with_system(system::offer_cooked_donut)
            .with_system(system::next_customer)
            .with_system(system::fill_line)
            .with_system(system::winning),
    )
    .add_system_set(SystemSet::on_enter(AppState::GameOver).with_system(system::setup_game_over))
    .add_system_set(SystemSet::on_update(AppState::GameOver).with_system(system::play_again_button))
    .add_system_set(SystemSet::on_exit(AppState::GameOver).with_system(system::cleanup))
    .add_system(system::disappearing)
    .add_system(system::leaving)
    .add_system(system::mouse_scroll);

    if cfg!(debug_assertions) {
        app.add_plugin(WorldInspectorPlugin::new());
    }

    app.run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    AssetLoading,
    InGame,
    GameOver,
}
