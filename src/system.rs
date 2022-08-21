use crate::component::*;
use bevy::prelude::*;

pub fn change_cooking_donut(
    keys: Res<Input<KeyCode>>,
    mut bases: Query<(&mut Base, &Parent)>,
    mut glazings: Query<(&mut Glazing, &Parent)>,
    mut sprinkles: Query<(&mut Sprinkles, &Parent)>,

    cooking_donut: Query<Entity, With<CookingDonut>>,
) {
    for (mut base, parent) in bases.iter_mut() {
        if cooking_donut.contains(parent.get()) {
            if keys.just_pressed(KeyCode::Right) {
                base.cycle_right();
            }
            if keys.just_pressed(KeyCode::Left) {
                base.cycle_left();
            }
            if keys.just_pressed(KeyCode::W) {
                base.cycle_right();
            }
            if keys.just_pressed(KeyCode::Q) {
                base.cycle_left();
            }
        }
    }

    for (mut glazing, parent) in glazings.iter_mut() {
        if cooking_donut.contains(parent.get()) {
            if keys.just_pressed(KeyCode::S) {
                glazing.cycle_right();
            }
            if keys.just_pressed(KeyCode::A) {
                glazing.cycle_left();
            }
        }
    }

    for (mut sprinkle, parent) in sprinkles.iter_mut() {
        if cooking_donut.contains(parent.get()) {
            if keys.just_pressed(KeyCode::X) {
                sprinkle.cycle_right();
            }
            if keys.just_pressed(KeyCode::Z) {
                sprinkle.cycle_left();
            }
        }
    }
}

pub fn update_base_sprite(mut query: Query<(&Base, &mut TextureAtlasSprite), Changed<Base>>) {
    for (base, mut sprite) in query.iter_mut() {
        sprite.index = base.to_sprite_index();
    }
}

pub fn update_glazing_sprite(
    mut query: Query<(&Glazing, &mut TextureAtlasSprite), Changed<Glazing>>,
) {
    for (glazing, mut sprite) in query.iter_mut() {
        sprite.index = glazing.to_sprite_index();
    }
}

pub fn update_sprinkles_sprite(
    mut query: Query<(&Sprinkles, &mut TextureAtlasSprite), Changed<Sprinkles>>,
) {
    for (sprinkles, mut sprite) in query.iter_mut() {
        sprite.index = sprinkles.to_sprite_index();
    }
}

pub fn cook_another_donut(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    cooking_donut: Query<Entity, With<CookingDonut>>,
    handles: Res<crate::Handles>,
) {
    if keys.just_pressed(KeyCode::N) {
        if let Ok(cooking_donut) = cooking_donut.get_single() {
            commands.entity(cooking_donut).despawn_recursive();
        }

        commands
            .spawn_bundle(SpatialBundle::default())
            .insert(CookingDonut)
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: handles.donuts_atlas.clone(),
                        sprite: TextureAtlasSprite {
                            index: Base::START_SPRITE_INDEX,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Base(0));

                parent
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: handles.donuts_atlas.clone(),
                        sprite: TextureAtlasSprite {
                            index: Glazing::START_SPRITE_INDEX,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Glazing(0));

                parent
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: handles.donuts_atlas.clone(),
                        sprite: TextureAtlasSprite {
                            index: Sprinkles::START_SPRITE_INDEX,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Sprinkles(0));
            });
    }
}

pub fn offer_cooked_donut(
    keys: Res<Input<KeyCode>>,
    cooking_donut: Query<Entity, With<CookingDonut>>,
    customer: Query<&Taste, With<CurrentCustomer>>,
    bases: Query<(&Base, &Parent)>,
    glazings: Query<(&Glazing, &Parent)>,
    sprinkles: Query<(&Sprinkles, &Parent)>,
) {
    if keys.just_pressed(KeyCode::Return) {
        if let Ok(taste) = customer.get_single() {
            let base: Option<Base> = bases.iter().find_map(|(part, parent)| {
                if cooking_donut.contains(parent.get()) {
                    Some(*part)
                } else {
                    None
                }
            });

            let glazing: Option<Glazing> = glazings.iter().find_map(|(part, parent)| {
                if cooking_donut.contains(parent.get()) {
                    Some(*part)
                } else {
                    None
                }
            });

            let sprinkles: Option<Sprinkles> = sprinkles.iter().find_map(|(part, parent)| {
                if cooking_donut.contains(parent.get()) {
                    Some(*part)
                } else {
                    None
                }
            });

            if let (Some(base), Some(glazing), Some(sprinkles)) = (base, glazing, sprinkles) {
                let donut = DonutBundle {
                    base,
                    glazing,
                    sprinkles,
                };
                let donut_rank = taste.rank(&donut);

                println!("I rate this donut as {}", "⭐️".repeat(donut_rank));
            }
        }
    }
}
