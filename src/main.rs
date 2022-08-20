use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(load_assets)
        .add_system(change_base)
        .add_system(update_base_sprite)
        .run();
}

trait ToSpriteIndex {
    const START_SPRITE_INDEX: usize = 0;
    fn to_sprite_index(self: &Self) -> usize {
        Self::START_SPRITE_INDEX
    }
}

#[derive(Component)]
struct Base(i32);

impl ToSpriteIndex for Base {
    const START_SPRITE_INDEX: usize = 0;

    fn to_sprite_index(self: &Self) -> usize {
        Self::START_SPRITE_INDEX + (self.0 as usize)
    }
}

#[derive(Component)]
struct Glazing(i32);

#[derive(Component)]
struct Sprinkles(i32);

#[derive(Component)]
struct Stripes(i32);

#[derive(Bundle)]
struct DonutBundle {
    base: Base,
    glazing: Glazing,
    sprinkles: Sprinkles,
    stripes: Stripes,
}

struct SubTexture {
    #[allow(dead_code)]
    name: &'static str,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let donuts_texture: Handle<Image> = asset_server.load("Donuts/Spritesheet/donuts_sheet.png");

    let mut donuts_atlas = TextureAtlas::new_empty(donuts_texture.clone(), Vec2::new(1024., 2048.));
    for SubTexture {
        x,
        y,
        width,
        height,
        ..
    } in ATLAS_DATA.into_iter()
    {
        donuts_atlas.add_texture(bevy::sprite::Rect {
            min: Vec2::new(x as f32, y as f32),
            max: Vec2::new((x + width) as f32, (y + height) as f32),
        });
    }

    let donuts_atlas_handle = texture_atlases.add(donuts_atlas);

    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn_bundle(SpatialBundle::default())
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: donuts_atlas_handle.clone(),
                    sprite: TextureAtlasSprite {
                        index: 0,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Base(1));

            parent.spawn_bundle(SpriteSheetBundle {
                texture_atlas: donuts_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: 3,
                    ..Default::default()
                },
                ..Default::default()
            });

            parent.spawn_bundle(SpriteSheetBundle {
                texture_atlas: donuts_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: 13,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

fn change_base(keys: Res<Input<KeyCode>>, mut query: Query<&mut Base>) {
    for mut base in query.iter_mut() {
        if keys.just_pressed(KeyCode::Right) {
            base.0 += 1;
        }

        if keys.just_pressed(KeyCode::Left) {
            base.0 -= 1;
        }
    }
}

fn update_base_sprite(mut query: Query<(&Base, &mut TextureAtlasSprite), Changed<Base>>) {
    for (base, mut sprite) in query.iter_mut() {
        sprite.index = base.to_sprite_index();
    }
}

// I was too lazy to load and parse XML
const ATLAS_DATA: [SubTexture; 22] = [
    SubTexture {
        name: "donut_1.png",
        x: 0,
        y: 0,
        width: 264,
        height: 264,
    },
    SubTexture {
        name: "donut_2.png",
        x: 0,
        y: 528,
        width: 264,
        height: 264,
    },
    SubTexture {
        name: "donut_3.png",
        x: 0,
        y: 264,
        width: 264,
        height: 264,
    },
    SubTexture {
        name: "glazing_1.png",
        x: 226,
        y: 1668,
        width: 226,
        height: 226,
    },
    SubTexture {
        name: "glazing_2.png",
        x: 229,
        y: 1244,
        width: 226,
        height: 226,
    },
    SubTexture {
        name: "glazing_3.png",
        x: 229,
        y: 792,
        width: 226,
        height: 226,
    },
    SubTexture {
        name: "glazing_4.png",
        x: 264,
        y: 226,
        width: 226,
        height: 226,
    },
    SubTexture {
        name: "glazing_5.png",
        x: 229,
        y: 1018,
        width: 226,
        height: 226,
    },
    SubTexture {
        name: "glazing_6.png",
        x: 264,
        y: 452,
        width: 226,
        height: 226,
    },
    SubTexture {
        name: "glazing_zigzag_1.png",
        x: 0,
        y: 1230,
        width: 229,
        height: 219,
    },
    SubTexture {
        name: "glazing_zigzag_2.png",
        x: 0,
        y: 1011,
        width: 229,
        height: 219,
    },
    SubTexture {
        name: "glazing_zigzag_3.png",
        x: 0,
        y: 1449,
        width: 229,
        height: 219,
    },
    SubTexture {
        name: "glazing_zigzag_4.png",
        x: 0,
        y: 792,
        width: 229,
        height: 219,
    },
    SubTexture {
        name: "sprinkles_1.png",
        x: 455,
        y: 888,
        width: 195,
        height: 210,
    },
    SubTexture {
        name: "sprinkles_2.png",
        x: 455,
        y: 678,
        width: 195,
        height: 210,
    },
    SubTexture {
        name: "sprinkles_3.png",
        x: 452,
        y: 1664,
        width: 202,
        height: 221,
    },
    SubTexture {
        name: "sprinkles_4.png",
        x: 455,
        y: 1098,
        width: 195,
        height: 210,
    },
    SubTexture {
        name: "sprinkles_5.png",
        x: 490,
        y: 0,
        width: 195,
        height: 210,
    },
    SubTexture {
        name: "stripes_1.png",
        x: 435,
        y: 1470,
        width: 206,
        height: 194,
    },
    SubTexture {
        name: "stripes_2.png",
        x: 229,
        y: 1470,
        width: 206,
        height: 194,
    },
    SubTexture {
        name: "stripes_3.png",
        x: 264,
        y: 0,
        width: 226,
        height: 226,
    },
    SubTexture {
        name: "stripes_4.png",
        x: 0,
        y: 1668,
        width: 226,
        height: 226,
    },
];
