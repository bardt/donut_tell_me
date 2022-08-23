use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use serde;

mod component;
mod system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(JsonAssetPlugin::<TextureAtlasData>::new(&["atlas.json"]))
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::InGame)
                .with_collection::<MyAssets>(),
        )
        .add_state(GameState::AssetLoading)
        .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(init_assets))
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(system::change_cooking_donut)
                .with_system(system::update_base_sprite)
                .with_system(system::update_glazing_sprite)
                .with_system(system::update_sprinkles_sprite)
                .with_system(system::cook_another_donut)
                .with_system(system::offer_cooked_donut),
        )
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    AssetLoading,
    InGame,
}

#[derive(AssetCollection)]
struct MyAssets {
    #[asset(path = "Donuts/Spritesheet/donuts_sheet.png")]
    donuts_texture: Handle<Image>,
    #[asset(path = "Donuts/Spritesheet/donuts_sheet.atlas.json")]
    donuts_texture_data: Handle<TextureAtlasData>,
}

pub struct Handles {
    donuts_atlas: Handle<TextureAtlas>,
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "c16f5026-27f4-4a38-902e-619a2da113bc"]
struct TextureAtlasData {
    #[serde(rename(deserialize = "TextureAtlas"))]
    texture_atlas: TextureAtlasObject,
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "39193fb7-499d-4698-8b49-d1886ac0754c"]
struct TextureAtlasObject {
    #[serde(rename(deserialize = "SubTexture"))]
    sub_textures: Vec<SubTexture>,
}

#[derive(bevy::reflect::TypeUuid)]
#[uuid = "89ce27f6-46d4-4e27-83f6-0ffdc2ad6cf2"]
struct SubTexture {
    #[allow(dead_code)]
    name: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

#[derive(serde::Deserialize)]
struct SubTextureRaw {
    #[allow(dead_code)]
    name: String,
    x: String,
    y: String,
    width: String,
    height: String,
}

impl<'de> serde::Deserialize<'de> for SubTexture {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = SubTextureRaw::deserialize(deserializer)?;

        let name = raw.name;
        let x = raw.x.parse::<i32>().unwrap();
        let y = raw.y.parse::<i32>().unwrap();
        let width = raw.width.parse::<i32>().unwrap();
        let height = raw.height.parse::<i32>().unwrap();

        Ok(SubTexture {
            name,
            x,
            y,
            width,
            height,
        })
    }
}

fn init_assets(
    mut commands: Commands,
    my_assets: Res<MyAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    texture_atlas_data_assets: Res<Assets<TextureAtlasData>>,
) {
    let mut donuts_atlas =
        TextureAtlas::new_empty(my_assets.donuts_texture.clone(), Vec2::new(1024., 2048.));

    if let Some(donuts_texture_data) = texture_atlas_data_assets.get(&my_assets.donuts_texture_data)
    {
        for SubTexture {
            x,
            y,
            width,
            height,
            ..
        } in donuts_texture_data.texture_atlas.sub_textures.iter()
        {
            donuts_atlas.add_texture(bevy::sprite::Rect {
                min: Vec2::new(*x as f32, *y as f32),
                max: Vec2::new((x + width) as f32, (y + height) as f32),
            });
        }
    }

    commands.insert_resource(Handles {
        donuts_atlas: texture_atlases.add(donuts_atlas),
    });

    commands.spawn_bundle(Camera2dBundle::default());

    // @TODO: spawn customer somewhere else
    commands
        .spawn()
        .insert(component::Taste::random())
        .insert(component::CurrentCustomer);
}
