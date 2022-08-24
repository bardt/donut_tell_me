use crate::component::*;
use bevy::prelude::*;

pub fn change_cooking_donut(
    keys: Res<Input<KeyCode>>,

    mut cooking_donut: Query<
        (&mut Base, &mut Glazing, &mut Sprinkles),
        (With<CookingDonut>, With<Donut>),
    >,
) {
    for (mut base, mut glazing, mut sprinkles) in cooking_donut.iter_mut() {
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

        if keys.just_pressed(KeyCode::S) {
            glazing.cycle_right();
        }
        if keys.just_pressed(KeyCode::A) {
            glazing.cycle_left();
        }

        if keys.just_pressed(KeyCode::X) {
            sprinkles.cycle_right();
        }
        if keys.just_pressed(KeyCode::Z) {
            sprinkles.cycle_left();
        }
    }
}

pub fn add_donut_sprites(
    mut commands: Commands,
    added_donuts: Query<(Entity, &Base, &Glazing, &Sprinkles), Added<Donut>>,
    handles: Res<crate::Handles>,
) {
    for (entity, base, glazing, sprinkles) in added_donuts.iter() {
        commands.entity(entity).with_children(|parent| {
            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: handles.donuts_atlas.clone(),
                    sprite: TextureAtlasSprite {
                        index: base.to_sprite_index(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(*base);

            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: handles.donuts_atlas.clone(),
                    sprite: TextureAtlasSprite {
                        index: glazing.to_sprite_index(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(*glazing);

            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: handles.donuts_atlas.clone(),
                    sprite: TextureAtlasSprite {
                        index: sprinkles.to_sprite_index(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(*sprinkles);
        });
    }
}

pub fn update_donut_sprites(
    changed_donuts: Query<
        (&Children, &Base, &Glazing, &Sprinkles),
        (
            With<Donut>,
            Or<(Changed<Base>, Changed<Glazing>, Changed<Sprinkles>)>,
        ),
    >,
    mut set: ParamSet<(
        Query<(&mut Base, &mut TextureAtlasSprite), Without<Donut>>,
        Query<(&mut Glazing, &mut TextureAtlasSprite), Without<Donut>>,
        Query<(&mut Sprinkles, &mut TextureAtlasSprite), Without<Donut>>,
    )>,
) {
    for (children, parent_base, parent_glazing, parent_sprinkles) in changed_donuts.iter() {
        for &child in children.iter() {
            if let Ok((mut child_base, mut sprite)) = set.p0().get_mut(child) {
                *child_base = *parent_base;
                sprite.index = child_base.to_sprite_index();
            }

            if let Ok((mut child_glazing, mut sprite)) = set.p1().get_mut(child) {
                *child_glazing = *parent_glazing;
                sprite.index = child_glazing.to_sprite_index();
            }

            if let Ok((mut child_sprinkles, mut sprite)) = set.p2().get_mut(child) {
                *child_sprinkles = *parent_sprinkles;
                sprite.index = child_sprinkles.to_sprite_index();
            }
        }
    }
}

pub fn cook_another_donut(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    cooking_donut: Query<Entity, With<CookingDonut>>,
) {
    if keys.just_pressed(KeyCode::N) {
        if let Ok(cooking_donut) = cooking_donut.get_single() {
            commands.entity(cooking_donut).despawn_recursive();
        }

        commands
            .spawn_bundle(DonutBundle {
                spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(
                    0., -150., 0.,
                ))),
                ..Default::default()
            })
            .insert(CookingDonut);
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
                    ..Default::default()
                };
                let donut_rank = taste.rank(&donut);

                println!("I rate this donut as {}", "⭐️".repeat(donut_rank));
            }
        }
    }
}
