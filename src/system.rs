use crate::assets::*;
use crate::component::*;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::sprite::Anchor;
use rand::prelude::*;

pub fn setup_game(
    mut commands: Commands,
    atlases: Res<Atlases>,
    faces_metadata: Res<FacesMetadata>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    let mut rng = rand::thread_rng();

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
                .insert(TransactionLog);
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
    donut_camera: Query<Entity, With<PhotoCamera>>,
    mut images: ResMut<Assets<Image>>,
) {
    if keys.just_pressed(KeyCode::N) {
        if let Ok(cooking_donut) = cooking_donut.get_single() {
            commands.entity(cooking_donut).despawn_recursive();
        }
        if let Ok(donut_camera) = donut_camera.get_single() {
            commands.entity(donut_camera).despawn_recursive();
        }

        let size = Extent3d {
            width: 512,
            height: 512,
            ..default()
        };
        // This is the texture that will be rendered to.
        let mut image = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size,
                dimension: TextureDimension::D2,
                format: TextureFormat::Bgra8UnormSrgb,
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT,
            },
            ..default()
        };

        // fill image.data with zeroes
        image.resize(size);

        let image_handle = images.add(image);
        let camera_bundle = Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
                ..Default::default()
            },
            camera: Camera {
                target: RenderTarget::Image(image_handle.clone()),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0., -150., 0.))
                .with_scale(Vec3::ONE * 0.3),
            ..Default::default()
        };
        commands.spawn_bundle(camera_bundle).insert(PhotoCamera);

        commands
            .spawn_bundle(DonutBundle {
                spatial: SpatialBundle::from_transform(
                    Transform::from_translation(Vec3::new(0., -150., 0.))
                        .with_scale(Vec3::ONE * 0.5),
                ),
                photo: Photo(image_handle),
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
    atlases: Res<Atlases>,
    mut ev_photos_taken: EventWriter<PhotosTaken>,
    mut images: ResMut<Assets<Image>>,
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

                let size = Extent3d {
                    width: 512,
                    height: 512,
                    ..default()
                };
                // This is the texture that will be rendered to.
                let mut image = Image {
                    texture_descriptor: TextureDescriptor {
                        label: None,
                        size,
                        dimension: TextureDimension::D2,
                        format: TextureFormat::Bgra8UnormSrgb,
                        mip_level_count: 1,
                        sample_count: 1,
                        usage: TextureUsages::TEXTURE_BINDING
                            | TextureUsages::COPY_DST
                            | TextureUsages::RENDER_ATTACHMENT,
                    },
                    ..default()
                };

                // fill image.data with zeroes
                image.resize(size);

                let image_handle = images.add(image);
                let camera_bundle = Camera2dBundle {
                    camera_2d: Camera2d {
                        clear_color: ClearColorConfig::None,
                        ..Default::default()
                    },
                    camera: Camera {
                        target: RenderTarget::Image(image_handle.clone()),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::new(0., 0., 0.))
                        .with_scale(Vec3::ONE * 0.1),
                    ..Default::default()
                };
                commands.spawn_bundle(camera_bundle).insert(PhotoCamera);

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
                    .insert(Photo(image_handle))
                    .insert(DisappearingTimer(Timer::from_seconds(1., false)));

                println!("I rate this donut as {}", "⭐️".repeat(donut_rank));

                ev_photos_taken.send(PhotosTaken);
            }
        }
    }
}

pub fn log_transaction(
    mut commands: Commands,
    mut ev_photos_taken: EventReader<PhotosTaken>,
    photo_cameras: Query<Entity, With<PhotoCamera>>,
    cooking_donut: Query<(Entity, &Photo), With<CookingDonut>>,
    emo_photo: Query<&Photo, With<Emo>>,
    log: Query<Entity, With<TransactionLog>>,
) {
    for _event in ev_photos_taken.iter() {
        if let Ok((cooking_donut, photo)) = cooking_donut.get_single() {
            if let Ok(emo_photo) = emo_photo.get_single() {
                for photo_camera in photo_cameras.iter() {
                    commands.entity(photo_camera).despawn_recursive();
                }
                commands.entity(cooking_donut).despawn_recursive();

                for log in log.iter() {
                    let new_entry = commands
                        .spawn_bundle(NodeBundle {
                            color: Color::TEAL.into(),
                            style: Style {
                                flex_shrink: 0.,
                                size: Size::new(Val::Auto, Val::Px(100.)),
                                margin: UiRect::all(Val::Px(20.)),
                                ..default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(ImageBundle {
                                    image: UiImage(photo.0.clone()),
                                    style: Style {
                                        size: Size {
                                            width: Val::Px(100.),
                                            height: Val::Px(100.),
                                        },
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(Node::default());

                            parent
                                .spawn_bundle(ImageBundle {
                                    image: UiImage(emo_photo.0.clone()),
                                    style: Style {
                                        size: Size {
                                            width: Val::Px(100.),
                                            height: Val::Px(100.),
                                        },
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(Node::default());
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
