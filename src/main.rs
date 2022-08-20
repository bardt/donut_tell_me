use bevy::prelude::*;
mod components;

use components::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(load_assets)
        .add_system(change_donut)
        .add_system(update_base_sprite)
        .add_system(update_glazing_sprite)
        .add_system(update_sprinkles_sprite)
        .add_system(update_stripes_sprite)
        .run();
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
                        index: Base::START_SPRITE_INDEX,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Base(0));

            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: donuts_atlas_handle.clone(),
                    sprite: TextureAtlasSprite {
                        index: Glazing::START_SPRITE_INDEX,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Glazing(0));

            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: donuts_atlas_handle.clone(),
                    sprite: TextureAtlasSprite {
                        index: Sprinkles::START_SPRITE_INDEX,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Sprinkles(0));
        });
}

fn change_donut(
    keys: Res<Input<KeyCode>>,
    mut bases: Query<&mut Base>,
    mut glazings: Query<&mut Glazing>,
    mut sprinkles: Query<&mut Sprinkles>,
    mut stripes: Query<&mut Stripes>,
) {
    for mut base in bases.iter_mut() {
        if keys.just_pressed(KeyCode::Right) {
            base.cycle_right();
        }

        if keys.just_pressed(KeyCode::Left) {
            base.cycle_left();
        }

        if keys.just_pressed(KeyCode::Key2) {
            base.cycle_right();
        }

        if keys.just_pressed(KeyCode::Key1) {
            base.cycle_left();
        }
    }

    for mut glazing in glazings.iter_mut() {
        if keys.just_pressed(KeyCode::W) {
            glazing.cycle_right();
        }

        if keys.just_pressed(KeyCode::Q) {
            glazing.cycle_left();
        }
    }

    for mut sprinkle in sprinkles.iter_mut() {
        if keys.just_pressed(KeyCode::S) {
            sprinkle.cycle_right();
        }

        if keys.just_pressed(KeyCode::A) {
            sprinkle.cycle_left();
        }
    }

    for mut stripe in stripes.iter_mut() {
        if keys.just_pressed(KeyCode::X) {
            stripe.cycle_right();
        }

        if keys.just_pressed(KeyCode::Z) {
            stripe.cycle_left();
        }
    }
}

fn update_base_sprite(mut query: Query<(&Base, &mut TextureAtlasSprite), Changed<Base>>) {
    for (base, mut sprite) in query.iter_mut() {
        sprite.index = base.to_sprite_index();
    }
}

fn update_glazing_sprite(mut query: Query<(&Glazing, &mut TextureAtlasSprite), Changed<Glazing>>) {
    for (glazing, mut sprite) in query.iter_mut() {
        sprite.index = glazing.to_sprite_index();
    }
}

fn update_sprinkles_sprite(
    mut query: Query<(&Sprinkles, &mut TextureAtlasSprite), Changed<Sprinkles>>,
) {
    for (sprinkles, mut sprite) in query.iter_mut() {
        sprite.index = sprinkles.to_sprite_index();
    }
}

fn update_stripes_sprite(mut query: Query<(&Stripes, &mut TextureAtlasSprite), Changed<Stripes>>) {
    for (stripes, mut sprite) in query.iter_mut() {
        sprite.index = stripes.to_sprite_index();
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
