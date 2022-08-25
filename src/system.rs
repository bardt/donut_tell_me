use crate::assets::*;
use crate::component::*;
use bevy::input::mouse::MouseScrollUnit;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::prelude::*;

pub fn setup_game(
    mut commands: Commands,
    atlases: Res<Atlases>,
    my_assets: Res<MyAssets>,
    faces_metadata: Res<FacesMetadata>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    let mut rng = rand::thread_rng();

    commands.spawn_bundle(SalesLogBundle {
        spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(
            150., 0., 0.,
        ))),
        ..Default::default()
    });

    // Log UI
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                align_self: AlignSelf::Center,
                size: Size::new(Val::Px(300.0), Val::Percent(100.0)),
                overflow: Overflow::Hidden,
                position_type: PositionType::Absolute,
                position: UiRect {
                    right: Val::Px(0.),
                    ..default()
                },
                ..default()
            },
            color: Color::rgb(0.10, 0.10, 0.10).into(),
            ..default()
        })
        .with_children(|parent| {
            // Moving panel
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        flex_grow: 1.0,
                        max_size: Size::new(Val::Undefined, Val::Undefined),
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .insert(ScrollingList::default())
                .with_children(|parent| {
                    // List items
                    for i in 0..30 {
                        parent.spawn_bundle(
                            TextBundle::from_section(
                                format!("Item {i}"),
                                TextStyle {
                                    font: my_assets.font_blocks.clone(),
                                    font_size: 20.,
                                    color: Color::WHITE,
                                },
                            )
                            .with_style(Style {
                                flex_shrink: 0.,
                                size: Size::new(Val::Undefined, Val::Px(20.)),
                                margin: UiRect {
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    ..default()
                                },
                                ..default()
                            }),
                        );
                    }
                });
        });

    // @TODO: spawn customer somewhere else
    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform::from_translation(Vec3::new(0., 150., 0.)),
            ..default()
        })
        .insert(Taste::random())
        .insert(CurrentCustomer)
        .with_children(|parent| {
            parent.spawn_bundle(SpriteSheetBundle {
                texture_atlas: atlases.skin_atlas.clone(),
                sprite: TextureAtlasSprite {
                    index: 2,
                    anchor: Anchor::Center,
                    ..Default::default()
                },
                ..Default::default()
            });

            parent.spawn_bundle(SpriteSheetBundle {
                texture_atlas: atlases.face_atlas.clone(),
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
}

#[derive(Component, Default)]
pub struct ScrollingList {
    position: f32,
}

pub fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Children, &Node)>,
    query_item: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, children, uinode) in &mut query_list {
            let items_height: f32 = children
                .iter()
                .map(|entity| query_item.get(*entity).unwrap().size.y)
                .sum();
            let panel_height = uinode.size.y;
            let max_scroll = (items_height - panel_height).max(0.);
            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };
            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.position.top = Val::Px(scrolling_list.position);
        }
    }
}

#[allow(clippy::type_complexity)]
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
    atlases: Res<Atlases>,
) {
    for (entity, base, glazing, sprinkles) in added_donuts.iter() {
        commands.entity(entity).with_children(|parent| {
            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: atlases.donuts_atlas.clone(),
                    sprite: TextureAtlasSprite {
                        index: base.to_sprite_index(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(*base);

            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: atlases.donuts_atlas.clone(),
                    sprite: TextureAtlasSprite {
                        index: glazing.to_sprite_index(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(*glazing);

            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: atlases.donuts_atlas.clone(),
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

#[allow(clippy::type_complexity)]
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
                spatial: SpatialBundle::from_transform(
                    Transform::from_translation(Vec3::new(0., -150., 0.))
                        .with_scale(Vec3::ONE * 0.5),
                ),
                ..Default::default()
            })
            .insert(CookingDonut);
    }
}

pub fn offer_cooked_donut(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    cooking_donut: Query<(&Base, &Glazing, &Sprinkles), With<CookingDonut>>,
    customer: Query<&Taste, With<CurrentCustomer>>,
    log: Query<(Entity, &Children), With<SalesLog>>,
    atlases: Res<Atlases>,
) {
    if keys.just_pressed(KeyCode::Return) {
        if let Ok(taste) = customer.get_single() {
            if let Ok((base, glazing, sprinkles)) = cooking_donut.get_single() {
                let donut_rank = taste.rank(base, glazing, sprinkles);

                let emotion = match donut_rank {
                    5 => Emo::Love,
                    4 => Emo::Happy,
                    3 => Emo::Sad,
                    2 => Emo::Angry,
                    _ => Emo::Heartbroken,
                };

                commands
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: atlases.emotes_atlas.clone(),
                        sprite: TextureAtlasSprite {
                            index: emotion as usize,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(emotion)
                    .insert(DisappearingTimer(Timer::from_seconds(1., false)));

                println!("I rate this donut as {}", "⭐️".repeat(donut_rank));

                for (log, entries) in log.iter() {
                    let new_entry = commands
                        .spawn_bundle(DonutBundle {
                            base: *base,
                            glazing: *glazing,
                            sprinkles: *sprinkles,
                            spatial: SpatialBundle::from_transform(
                                Transform::from_xyz(0., 100. * entries.len() as f32, 0.)
                                    .with_scale(Vec3::ONE * 0.3),
                            ),
                            ..Default::default()
                        })
                        .id();
                    commands.entity(log).push_children(&[new_entry]);
                }
            }
        }
    }
}

pub fn disappearing(
    mut commands: Commands,
    mut timers: Query<(Entity, &mut DisappearingTimer)>,
    time: Res<Time>,
) {
    for (entity, mut timer) in timers.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
