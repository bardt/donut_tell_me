use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;

mod assets;
mod component;
mod system;

fn main() {
    static PHOTO: &str = "photo";
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(JsonAssetPlugin::<assets::TextureAtlasData>::new(&[
            "atlas.json",
        ]))
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
                .with_system(system::update_donut_sprites)
                .with_system(system::cook_another_donut)
                .with_system(system::offer_cooked_donut)
                .with_system(system::winning),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::GameOver).with_system(system::setup_game_over),
        )
        .add_system(system::disappearing)
        .add_system(system::mouse_scroll)
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    AssetLoading,
    InGame,
    GameOver,
}
