use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection)]
pub struct MyAssets {
    #[asset(path = "fonts/Kenney_Blocks.ttf")]
    pub font_blocks: Handle<Font>,
    #[asset(path = "fonts/Kenney_Pixel.ttf")]
    pub font_pixel: Handle<Font>,
    #[asset(path = "Interface Pack/PNG/Retina/panel_woodWear.png")]
    pub ui_panel_wood_wear: Handle<Image>,
    #[asset(path = "Donuts/Spritesheet/donuts_sheet.png")]
    pub donuts_texture: Handle<Image>,
    #[asset(path = "Donuts/Spritesheet/donuts_sheet.atlas.json")]
    pub donuts_texture_data: Handle<TextureAtlasData>,
    #[asset(path = "Emote Pack/Spritesheets/vector_style1.png")]
    pub emotes_texture: Handle<Image>,
    #[asset(path = "Emote Pack/Spritesheets/vector_style1.atlas.json")]
    pub emotes_texture_data: Handle<TextureAtlasData>,
    #[asset(path = "Character Pack/Spritesheet/sheet_face.png")]
    pub face_texture: Handle<Image>,
    #[asset(path = "Character Pack/Spritesheet/sheet_face.atlas.json")]
    pub face_texture_data: Handle<TextureAtlasData>,
    #[asset(path = "Character Pack/Spritesheet/sheet_hair.png")]
    pub hair_texture: Handle<Image>,
    #[asset(path = "Character Pack/Spritesheet/sheet_hair.atlas.json")]
    pub hair_texture_data: Handle<TextureAtlasData>,
    // #[asset(path = "Character Pack/Spritesheet/sheet_pants.png")]
    // pub pants_texture: Handle<Image>,
    // #[asset(path = "Character Pack/Spritesheet/sheet_pants.atlas.json")]
    // pub pants_texture_data: Handle<TextureAtlasData>,
    // #[asset(path = "Character Pack/Spritesheet/sheet_shirts.png")]
    // pub shirts_texture: Handle<Image>,
    // #[asset(path = "Character Pack/Spritesheet/sheet_shirts.atlas.json")]
    // pub shirts_texture_data: Handle<TextureAtlasData>,
    // #[asset(path = "Character Pack/Spritesheet/sheet_shoes.png")]
    // pub shoes_texture: Handle<Image>,
    // #[asset(path = "Character Pack/Spritesheet/sheet_shoes.atlas.json")]
    // pub shoes_texture_data: Handle<TextureAtlasData>,
    #[asset(path = "Character Pack/Spritesheet/sheet_skin.png")]
    pub skin_texture: Handle<Image>,
    #[asset(path = "Character Pack/Spritesheet/sheet_skin.atlas.json")]
    pub skin_texture_data: Handle<TextureAtlasData>,
}

pub struct Atlases {
    pub donuts_atlas: Handle<TextureAtlas>,
    pub emotes_atlas: Handle<TextureAtlas>,
    pub face_atlas: Handle<TextureAtlas>,
    pub _hair_atlas: Handle<TextureAtlas>,
    pub skin_atlas: Handle<TextureAtlas>,
}

pub struct FacesMetadata {
    pub face_indexes: Vec<usize>,
}

pub fn init(
    mut commands: Commands,
    my_assets: Res<MyAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    texture_atlas_data_assets: Res<Assets<TextureAtlasData>>,
) {
    let mut donuts_atlas =
        TextureAtlas::new_empty(my_assets.donuts_texture.clone(), Vec2::new(1024., 2048.));
    donuts_atlas.fill_textures(texture_atlas_data_assets.get(&my_assets.donuts_texture_data));

    let mut emotes_atlas =
        TextureAtlas::new_empty(my_assets.emotes_texture.clone(), Vec2::new(1024., 2048.));
    emotes_atlas.fill_textures(texture_atlas_data_assets.get(&my_assets.emotes_texture_data));

    let mut face_atlas =
        TextureAtlas::new_empty(my_assets.face_texture.clone(), Vec2::new(256., 512.));
    face_atlas.fill_textures(texture_atlas_data_assets.get(&my_assets.face_texture_data));

    let faces_metadata = FacesMetadata {
        face_indexes: texture_atlas_data_assets
            .get(&my_assets.face_texture_data)
            .map(|face_texture_data| {
                face_texture_data
                    .texture_atlas
                    .sub_textures
                    .iter()
                    .enumerate()
                    .filter_map(|(index, sub_texture)| {
                        if sub_texture.name.starts_with("face") {
                            Some(index)
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_else(Vec::new),
    };

    let mut hair_atlas =
        TextureAtlas::new_empty(my_assets.hair_texture.clone(), Vec2::new(2048., 2048.));
    hair_atlas.fill_textures(texture_atlas_data_assets.get(&my_assets.hair_texture_data));

    let mut skin_atlas =
        TextureAtlas::new_empty(my_assets.skin_texture.clone(), Vec2::new(1024., 1024.));
    skin_atlas.fill_textures(texture_atlas_data_assets.get(&my_assets.skin_texture_data));

    let handles = Atlases {
        donuts_atlas: texture_atlases.add(donuts_atlas),
        emotes_atlas: texture_atlases.add(emotes_atlas),
        face_atlas: texture_atlases.add(face_atlas),
        _hair_atlas: texture_atlases.add(hair_atlas),
        skin_atlas: texture_atlases.add(skin_atlas),
    };

    commands.insert_resource(handles);
    commands.insert_resource(faces_metadata);
}

// JSON converted from XML via https://javadev.github.io/xml-to-json/
#[derive(serde::Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "c16f5026-27f4-4a38-902e-619a2da113bc"]
pub struct TextureAtlasData {
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

trait FillTextures {
    fn fill_textures(&mut self, texture_atlas_data: Option<&TextureAtlasData>);
}
impl FillTextures for TextureAtlas {
    fn fill_textures(&mut self, texture_atlas_data: Option<&TextureAtlasData>) {
        if let Some(texture_data) = texture_atlas_data {
            for SubTexture {
                x,
                y,
                width,
                height,
                ..
            } in texture_data.texture_atlas.sub_textures.iter()
            {
                self.add_texture(bevy::sprite::Rect {
                    min: Vec2::new(*x as f32, *y as f32),
                    max: Vec2::new((x + width) as f32, (y + height) as f32),
                });
            }
        }
    }
}
