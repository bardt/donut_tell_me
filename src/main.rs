use bevy::{prelude::*, sprite::Anchor};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use rand::prelude::*;
use serde;

mod component;
mod system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
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
    #[asset(path = "Character Pack/Spritesheet/sheet_face.png")]
    face_texture: Handle<Image>,
    #[asset(path = "Character Pack/Spritesheet/sheet_face.atlas.json")]
    face_texture_data: Handle<TextureAtlasData>,
    #[asset(path = "Character Pack/Spritesheet/sheet_hair.png")]
    hair_texture: Handle<Image>,
    #[asset(path = "Character Pack/Spritesheet/sheet_hair.atlas.json")]
    hair_texture_data: Handle<TextureAtlasData>,
    // #[asset(path = "Character Pack/Spritesheet/sheet_pants.png")]
    // pants_texture: Handle<Image>,
    // #[asset(path = "Character Pack/Spritesheet/sheet_pants.atlas.json")]
    // pants_texture_data: Handle<TextureAtlasData>,
    // #[asset(path = "Character Pack/Spritesheet/sheet_shirts.png")]
    // shirts_texture: Handle<Image>,
    // #[asset(path = "Character Pack/Spritesheet/sheet_shirts.atlas.json")]
    // shirts_texture_data: Handle<TextureAtlasData>,
    // #[asset(path = "Character Pack/Spritesheet/sheet_shoes.png")]
    // shoes_texture: Handle<Image>,
    // #[asset(path = "Character Pack/Spritesheet/sheet_shoes.atlas.json")]
    // shoes_texture_data: Handle<TextureAtlasData>,
    #[asset(path = "Character Pack/Spritesheet/sheet_skin.png")]
    skin_texture: Handle<Image>,
    #[asset(path = "Character Pack/Spritesheet/sheet_skin.atlas.json")]
    skin_texture_data: Handle<TextureAtlasData>,
}

pub struct Handles {
    donuts_atlas: Handle<TextureAtlas>,
    face_atlas: Handle<TextureAtlas>,
    hair_atlas: Handle<TextureAtlas>,
    skin_atlas: Handle<TextureAtlas>,
}

struct FacesMetadata {
    face_indexes: Vec<usize>,
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

    let mut faces_metadata = FacesMetadata {
        face_indexes: vec![],
    };
    let mut face_atlas =
        TextureAtlas::new_empty(my_assets.face_texture.clone(), Vec2::new(1024., 2048.));
    if let Some(face_texture_data) = texture_atlas_data_assets.get(&my_assets.face_texture_data) {
        for (
            index,
            SubTexture {
                x,
                y,
                width,
                height,
                name,
            },
        ) in face_texture_data
            .texture_atlas
            .sub_textures
            .iter()
            .enumerate()
        {
            if name.starts_with("face") {
                faces_metadata.face_indexes.push(index);
            }

            face_atlas.add_texture(bevy::sprite::Rect {
                min: Vec2::new(*x as f32, *y as f32),
                max: Vec2::new((x + width) as f32, (y + height) as f32),
            });
        }
    }

    let mut hair_atlas =
        TextureAtlas::new_empty(my_assets.hair_texture.clone(), Vec2::new(1024., 2048.));
    if let Some(hair_texture_data) = texture_atlas_data_assets.get(&my_assets.hair_texture_data) {
        for SubTexture {
            x,
            y,
            width,
            height,
            ..
        } in hair_texture_data.texture_atlas.sub_textures.iter()
        {
            hair_atlas.add_texture(bevy::sprite::Rect {
                min: Vec2::new(*x as f32, *y as f32),
                max: Vec2::new((x + width) as f32, (y + height) as f32),
            });
        }
    }

    let mut skin_atlas =
        TextureAtlas::new_empty(my_assets.skin_texture.clone(), Vec2::new(1024., 2048.));
    if let Some(skin_texture_data) = texture_atlas_data_assets.get(&my_assets.skin_texture_data) {
        for SubTexture {
            x,
            y,
            width,
            height,
            ..
        } in skin_texture_data.texture_atlas.sub_textures.iter()
        {
            skin_atlas.add_texture(bevy::sprite::Rect {
                min: Vec2::new(*x as f32, *y as f32),
                max: Vec2::new((x + width) as f32, (y + height) as f32),
            });
        }
    }

    let handles = Handles {
        donuts_atlas: texture_atlases.add(donuts_atlas),
        face_atlas: texture_atlases.add(face_atlas),
        hair_atlas: texture_atlases.add(hair_atlas),
        skin_atlas: texture_atlases.add(skin_atlas),
    };

    commands.spawn_bundle(Camera2dBundle::default());

    let mut rng = rand::thread_rng();

    // @TODO: spawn customer somewhere else
    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform::from_translation(Vec3::new(0., 150., 0.)),
            ..default()
        })
        .insert(component::Taste::random())
        .insert(component::CurrentCustomer)
        .with_children(|parent| {
            parent.spawn_bundle(SpriteSheetBundle {
                texture_atlas: handles.skin_atlas.clone(),
                sprite: TextureAtlasSprite {
                    index: 2,
                    anchor: Anchor::Center,
                    ..Default::default()
                },
                ..Default::default()
            });

            parent.spawn_bundle(SpriteSheetBundle {
                texture_atlas: handles.face_atlas.clone(),
                transform: Transform::default().with_translation(Vec3::new(0., 0., 1.)),
                sprite: TextureAtlasSprite {
                    index: faces_metadata
                        .face_indexes
                        .choose(&mut rng)
                        .cloned()
                        .unwrap_or(0),
                    anchor: Anchor::Center,
                    ..Default::default()
                },
                ..Default::default()
            });

            // parent.spawn_bundle(SpriteSheetBundle {
            //     texture_atlas: handles.hair_atlas.clone(),
            //     sprite: TextureAtlasSprite {
            //         index: 28,
            //         // @TODO: each hair style will need a custom anchor point
            //         anchor: Anchor::Custom(Vec2::new(0.0, 0.0)),
            //         ..Default::default()
            //     },
            //     ..Default::default()
            // });
        });

    commands.insert_resource(handles);
    commands.insert_resource(faces_metadata)
}

// JSON converted from XML via https://javadev.github.io/xml-to-json/
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
    #[serde(rename(deserialize = "-name"))]
    name: String,
    #[serde(rename(deserialize = "-x"))]
    x: String,
    #[serde(rename(deserialize = "-y"))]
    y: String,
    #[serde(rename(deserialize = "-width"))]
    width: String,
    #[serde(rename(deserialize = "-height"))]
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
