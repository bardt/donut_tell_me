use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;

mod assets;
mod component;
mod system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(JsonAssetPlugin::<assets::TextureAtlasData>::new(&["atlas.json"]))
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::InGame)
                .with_collection::<assets::MyAssets>(),
        )
        .add_state(GameState::AssetLoading)
        .add_system_set(SystemSet::on_exit(GameState::AssetLoading).with_system(assets::init))
        .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(system::setup_game))
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(system::add_donut_sprites)
                .with_system(system::change_cooking_donut)
                .with_system(system::update_donut_sprites)
                .with_system(system::cook_another_donut)
                .with_system(system::offer_cooked_donut),
        )
        .add_system(system::disappearing)
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    AssetLoading,
    InGame,
}
