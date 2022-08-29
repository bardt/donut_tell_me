use std::collections::VecDeque;

use crate::assets::*;
use crate::component::*;
use crate::AppState;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::encase::rts_array::Length;
use bevy::render::render_resource::{Extent3d, TextureUsages};
use bevy::render::view::RenderLayers;
use bevy::sprite::Anchor;
use bevy_ninepatch::*;
use rand::prelude::*;

pub fn setup_game(
    mut commands: Commands,
    my_assets: Res<MyAssets>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,
) {
    let mut main_camera_bundle = Camera2dBundle::default();
    main_camera_bundle.transform.translation.x = 100.; // half of the side panel width
    commands
        .spawn_bundle(main_camera_bundle)
        .insert(RenderLayers::from_layers(&[0, 1]));

    commands.insert_resource(Line(VecDeque::new()));

    let nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins(44, 44, 44, 44));

    // Layout
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                direction: Direction::LeftToRight,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::SpaceEvenly,
                size: Size {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                },

                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(Layout)
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Stretch,
                        justify_content: JustifyContent::Center,
                        flex_grow: 1.,
                        flex_shrink: 1.,
                        size: Size {
                            width: Val::Auto,
                            height: Val::Percent(50.),
                        },
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Left buttons
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::ColumnReverse,
                                align_items: AlignItems::FlexEnd,
                                justify_content: JustifyContent::SpaceEvenly,
                                flex_grow: 1.,
                                flex_shrink: 1.,
                                size: Size {
                                    width: Val::Auto,
                                    height: Val::Percent(100.),
                                },
                                padding: UiRect::all(Val::Px(10.)),
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(ButtonBundle {
                                    image: UiImage(my_assets.ui_button_rectangle_wood.clone()),
                                    style: Style {
                                        padding: UiRect::all(Val::Px(10.)),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(BaseLeftButton)
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle {
                                        text: Text {
                                            sections: vec![TextSection {
                                                value: "< base".to_string(),
                                                style: TextStyle {
                                                    font_size: 20.,
                                                    font: my_assets.font_pixel.clone(),
                                                    color: Color::BLACK,
                                                },
                                            }],
                                            alignment: TextAlignment::CENTER,
                                        },
                                        style: Style {
                                            margin: UiRect::all(Val::Auto),

                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    });
                                });

                            parent
                                .spawn_bundle(ButtonBundle {
                                    image: UiImage(my_assets.ui_button_rectangle_wood.clone()),
                                    style: Style {
                                        padding: UiRect::all(Val::Px(10.)),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(GlazingLeftButton)
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle {
                                        text: Text {
                                            sections: vec![TextSection {
                                                value: "< glazing".to_string(),
                                                style: TextStyle {
                                                    font_size: 20.,
                                                    font: my_assets.font_pixel.clone(),
                                                    color: Color::BLACK,
                                                },
                                            }],
                                            alignment: TextAlignment::CENTER,
                                        },
                                        style: Style {
                                            margin: UiRect::all(Val::Auto),

                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    });
                                });

                            parent
                                .spawn_bundle(ButtonBundle {
                                    image: UiImage(my_assets.ui_button_rectangle_wood.clone()),
                                    style: Style {
                                        padding: UiRect::all(Val::Px(10.)),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(SprinklesLeftButton)
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle {
                                        text: Text {
                                            sections: vec![TextSection {
                                                value: "< top".to_string(),
                                                style: TextStyle {
                                                    font_size: 20.,
                                                    font: my_assets.font_pixel.clone(),
                                                    color: Color::BLACK,
                                                },
                                            }],
                                            alignment: TextAlignment::CENTER,
                                        },
                                        style: Style {
                                            margin: UiRect::all(Val::Auto),

                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    });
                                });
                        });

                    // Center placeholder
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::ColumnReverse,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceBetween,
                                flex_grow: 0.,
                                flex_shrink: 0.,
                                size: Size {
                                    width: Val::Px(256.),
                                    height: Val::Percent(100.),
                                },
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(ButtonBundle {
                                    image: UiImage(my_assets.ui_button_rectangle_wood.clone()),
                                    style: Style {
                                        padding: UiRect::all(Val::Px(10.)),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(OfferButton)
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle {
                                        text: Text {
                                            sections: vec![TextSection {
                                                value: "^ offer".to_string(),
                                                style: TextStyle {
                                                    font_size: 20.,
                                                    font: my_assets.font_pixel.clone(),
                                                    color: Color::BLACK,
                                                },
                                            }],
                                            alignment: TextAlignment::CENTER,
                                        },
                                        style: Style {
                                            margin: UiRect::all(Val::Auto),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    });
                                });

                            parent
                                .spawn_bundle(ButtonBundle {
                                    image: UiImage(my_assets.ui_button_rectangle_wood.clone()),
                                    style: Style {
                                        padding: UiRect::all(Val::Px(10.)),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(NewDonutButton)
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle {
                                        text: Text {
                                            sections: vec![TextSection {
                                                value: "new donut".to_string(),
                                                style: TextStyle {
                                                    font_size: 20.,
                                                    font: my_assets.font_pixel.clone(),
                                                    color: Color::BLACK,
                                                },
                                            }],
                                            alignment: TextAlignment::CENTER,
                                        },
                                        style: Style {
                                            margin: UiRect::all(Val::Auto),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    });
                                });
                        });

                    // Right buttons
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::ColumnReverse,
                                align_items: AlignItems::FlexStart,
                                justify_content: JustifyContent::SpaceEvenly,
                                flex_grow: 1.,
                                flex_shrink: 1.,
                                size: Size {
                                    width: Val::Auto,
                                    height: Val::Percent(100.),
                                },
                                padding: UiRect::all(Val::Px(10.)),
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(ButtonBundle {
                                    image: UiImage(my_assets.ui_button_rectangle_wood.clone()),
                                    style: Style {
                                        padding: UiRect::all(Val::Px(10.)),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(BaseRightButton)
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle {
                                        text: Text {
                                            sections: vec![TextSection {
                                                value: "base >".to_string(),
                                                style: TextStyle {
                                                    font_size: 20.,
                                                    font: my_assets.font_pixel.clone(),
                                                    color: Color::BLACK,
                                                },
                                            }],
                                            alignment: TextAlignment::CENTER,
                                        },
                                        style: Style {
                                            margin: UiRect::all(Val::Auto),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    });
                                });

                            parent
                                .spawn_bundle(ButtonBundle {
                                    image: UiImage(my_assets.ui_button_rectangle_wood.clone()),
                                    style: Style {
                                        padding: UiRect::all(Val::Px(10.)),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(GlazingRightButton)
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle {
                                        text: Text {
                                            sections: vec![TextSection {
                                                value: "glazing >".to_string(),
                                                style: TextStyle {
                                                    font_size: 20.,
                                                    font: my_assets.font_pixel.clone(),
                                                    color: Color::BLACK,
                                                },
                                            }],
                                            alignment: TextAlignment::CENTER,
                                        },
                                        style: Style {
                                            margin: UiRect::all(Val::Auto),

                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    });
                                });

                            parent
                                .spawn_bundle(ButtonBundle {
                                    image: UiImage(my_assets.ui_button_rectangle_wood.clone()),
                                    style: Style {
                                        padding: UiRect::all(Val::Px(10.)),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(SprinklesRightButton)
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle {
                                        text: Text {
                                            sections: vec![TextSection {
                                                value: "top >".to_string(),
                                                style: TextStyle {
                                                    font_size: 20.,
                                                    font: my_assets.font_pixel.clone(),
                                                    color: Color::BLACK,
                                                },
                                            }],
                                            alignment: TextAlignment::CENTER,
                                        },
                                        style: Style {
                                            margin: UiRect::all(Val::Auto),

                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    });
                                });
                        });
                });

            // Log UI
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        flex_grow: 0.,
                        flex_shrink: 0.,
                        align_self: AlignSelf::Center,
                        size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                        overflow: Overflow::Hidden,
                        position: UiRect {
                            right: Val::Px(0.),
                            ..default()
                        },
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Background
                    parent.spawn_bundle(NinePatchBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            position: UiRect::all(Val::Percent(0.)),
                            ..Default::default()
                        },
                        nine_patch_data: NinePatchData {
                            texture: my_assets.ui_panel_wood_wear.clone(),
                            nine_patch: nine_patch_handle.clone(),
                            ..default()
                        },
                        ..Default::default()
                    });

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
        });

    // Desk
    commands.spawn_bundle(SpriteBundle {
        texture: my_assets.ui_panel_wood_wear.clone(),
        transform: Transform::from_translation(Vec3::new(0., -150., 0.)).with_scale(Vec3::ONE * 2.),
        ..Default::default()
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

#[allow(clippy::type_complexity)]
pub fn change_cooking_donut_buttons(
    mut cooking_donut: Query<
        (&mut Base, &mut Glazing, &mut Sprinkles),
        (With<CookingDonut>, With<Donut>),
    >,
    mut set: ParamSet<(
        Query<&mut Interaction, With<BaseLeftButton>>,
        Query<&mut Interaction, With<BaseRightButton>>,
        Query<&mut Interaction, With<GlazingLeftButton>>,
        Query<&mut Interaction, With<GlazingRightButton>>,
        Query<&mut Interaction, With<SprinklesLeftButton>>,
        Query<&mut Interaction, With<SprinklesRightButton>>,
    )>,
) {
    for (mut base, mut glazing, mut sprinkles) in cooking_donut.iter_mut() {
        for mut base_left in set.p0().iter_mut() {
            if let Interaction::Clicked = *base_left {
                base.cycle_left();
                *base_left = Interaction::None;
            }
        }
        for mut base_right in set.p1().iter_mut() {
            if let Interaction::Clicked = *base_right {
                base.cycle_right();
                *base_right = Interaction::None;
            }
        }

        for mut glazing_left in set.p2().iter_mut() {
            if let Interaction::Clicked = *glazing_left {
                glazing.cycle_left();
                *glazing_left = Interaction::None;
            }
        }
        for mut glazing_right in set.p3().iter_mut() {
            if let Interaction::Clicked = *glazing_right {
                glazing.cycle_right();
                *glazing_right = Interaction::None;
            }
        }

        for mut sprinkles_left in set.p4().iter_mut() {
            if let Interaction::Clicked = *sprinkles_left {
                sprinkles.cycle_left();
                *sprinkles_left = Interaction::None;
            }
        }
        for mut sprinkles_right in set.p5().iter_mut() {
            if let Interaction::Clicked = *sprinkles_right {
                sprinkles.cycle_right();
                *sprinkles_right = Interaction::None;
            }
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
                .insert(*base)
                .insert(RenderLayers::layer(1));

            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: atlases.donuts_atlas.clone(),
                    sprite: TextureAtlasSprite {
                        index: glazing.to_sprite_index(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(*glazing)
                .insert(RenderLayers::layer(1));

            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: atlases.donuts_atlas.clone(),
                    sprite: TextureAtlasSprite {
                        index: sprinkles.to_sprite_index(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(*sprinkles)
                .insert(RenderLayers::layer(1));
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
    mut interactions: Query<&mut Interaction, With<NewDonutButton>>,
    cooking_donut: Query<Entity, With<CookingDonut>>,
) {
    let mut do_stuff = || {
        for cooking_donut in cooking_donut.iter() {
            commands.entity(cooking_donut).despawn_recursive();
        }

        commands
            .spawn_bundle(DonutBundle::new())
            .insert(CookingDonut);
    };

    for mut interaction in interactions.iter_mut() {
        if let Interaction::Clicked = *interaction {
            do_stuff();
            *interaction = Interaction::None;
        }
    }

    if keys.just_pressed(KeyCode::N) {
        do_stuff();
    }
}

#[allow(clippy::too_many_arguments)]
pub fn offer_cooked_donut(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut interactions: Query<&mut Interaction, With<OfferButton>>,
    cooking_donut: Query<(Entity, &Base, &Glazing, &Sprinkles), With<CookingDonut>>,
    customer: Query<(Entity, &Taste), With<CurrentCustomer>>,
    photo_cameras: Query<Entity, With<PhotoCamera>>,
    atlases: Res<Atlases>,
    mut ev_photos_taken: EventWriter<PhotosTakenEvent>,
    mut images: ResMut<Assets<Image>>,
) {
    let mut do_stuff = || {
        if let Ok((customer, taste)) = customer.get_single() {
            if let Ok((cooking_donut, base, glazing, sprinkles)) = cooking_donut.get_single() {
                let donut_rank = taste.rank(base, glazing, sprinkles);

                let emotion = match donut_rank {
                    5 => Emo::Love,
                    4 => Emo::Happy,
                    3 => Emo::Sad,
                    2 => Emo::Angry,
                    _ => Emo::Heartbroken,
                };

                if emotion == Emo::Love {
                    commands.entity(customer).insert(Regular);
                } else {
                    commands.entity(customer).remove::<Regular>();
                }

                commands
                    .entity(customer)
                    .insert(LeavingTimer(Timer::from_seconds(2.1, false)));

                for photo_camera in photo_cameras.iter() {
                    commands.entity(photo_camera).despawn_recursive();
                }

                let size = Extent3d {
                    width: 512,
                    height: 512,
                    ..default()
                };
                // This is the texture that will be rendered to.
                let mut image = Image::default();
                image.texture_descriptor.size = size;
                image.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT;

                // fill image.data with zeroes
                image.resize(size);

                let donut_image_handle = images.add(image.clone());
                let emo_image_handle = images.add(image);

                let donut_camera_bundle = Camera2dBundle {
                    camera_2d: Camera2d {
                        clear_color: ClearColorConfig::None,
                    },
                    camera: Camera {
                        target: RenderTarget::Image(donut_image_handle.clone()),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::new(0., -150., 1.))
                        .with_scale(Vec3::ONE * 0.3),
                    ..Default::default()
                };
                commands
                    .spawn_bundle(donut_camera_bundle)
                    .insert(PhotoCamera)
                    .insert(UiCameraConfig { show_ui: false })
                    .insert(RenderLayers::layer(1));

                commands
                    .entity(cooking_donut)
                    .insert(Photo(donut_image_handle));

                // Emo camera
                let emo_camera_bundle = Camera2dBundle {
                    camera_2d: Camera2d {
                        clear_color: ClearColorConfig::None,
                    },
                    camera: Camera {
                        target: RenderTarget::Image(emo_image_handle.clone()),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::new(0., 180., 10.))
                        .with_scale(Vec3::ONE * 0.5),
                    ..Default::default()
                };
                commands
                    .spawn_bundle(emo_camera_bundle)
                    .insert(PhotoCamera)
                    .insert(UiCameraConfig { show_ui: false });

                commands
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: atlases.emotes_atlas.clone(),
                        sprite: TextureAtlasSprite {
                            index: emotion as usize,
                            ..Default::default()
                        },
                        transform: Transform::from_translation(Vec3::new(100., 245., 0.))
                            .with_scale(Vec3::ONE * 1.5),
                        ..Default::default()
                    })
                    .insert(emotion)
                    .insert(Photo(emo_image_handle))
                    .insert(DisappearingTimer(Timer::from_seconds(2., false)));

                println!("I rate this donut as {}", "⭐️".repeat(donut_rank));

                ev_photos_taken.send(PhotosTakenEvent);
            }
        }
    };

    if keys.just_pressed(KeyCode::Return) {
        do_stuff();
    }

    for mut interaction in interactions.iter_mut() {
        if let Interaction::Clicked = *interaction {
            do_stuff();
            *interaction = Interaction::None
        }
    }
}

pub fn log_transaction(
    mut commands: Commands,
    mut ev_photos_taken: EventReader<PhotosTakenEvent>,
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
                            color: Color::NONE.into(),
                            style: Style {
                                flex_shrink: 0.,
                                padding: UiRect::all(Val::Px(20.)),

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
                                            width: Val::Px(80.),
                                            height: Val::Undefined,
                                        },
                                        aspect_ratio: Some(1.),
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
                                            width: Val::Px(80.),
                                            height: Val::Undefined,
                                        },
                                        aspect_ratio: Some(1.),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(Node::default());
                        })
                        .id();
                    commands.entity(log).insert_children(0, &[new_entry]);
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

pub fn leaving(
    mut commands: Commands,
    mut timers: Query<(Entity, &mut LeavingTimer)>,
    time: Res<Time>,
) {
    for (entity, mut timer) in timers.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            commands
                .entity(entity)
                .remove::<CurrentCustomer>()
                .insert(Visibility { is_visible: false });
        }
    }
}

pub fn winning(regulars: Query<Entity, With<Regular>>, mut app_state: ResMut<State<AppState>>) {
    if regulars.iter().count() >= 3 {
        app_state.set(AppState::GameOver).ok();
    }
}

pub fn setup_game_over(
    mut commands: Commands,
    my_assets: Res<MyAssets>,
    layout: Query<Entity, With<Layout>>,
) {
    let layout = layout.get_single().unwrap();

    let popup = commands.spawn_bundle(NodeBundle {
        style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                position_type: PositionType::Absolute,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
    }).with_children(|parent| {
        parent
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                align_self: AlignSelf::Center,
                size: Size::new(Val::Px(500.0), Val::Auto),
                position_type: PositionType::Relative,
                margin: UiRect::all(Val::Auto),
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::rgb(0.10, 0.10, 0.10).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "The game is over!".to_string(),
                        style: TextStyle {
                            font_size: 50.,
                            font: my_assets.font_blocks.clone(),
                            color: Color::WHITE,
                        },
                    }
                    ],
                    alignment: TextAlignment::CENTER,
                },
                style: Style {
                    size: Size::new(Val::Px(400.), Val::Auto),
                    ..Default::default()
                },
                ..Default::default()
            });

            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "You now have enough regular customers to sustain the business. Now it's time to lay back and chill. And instead of constantly guessing other people's wants, maybe ask yourself: What do I want?".to_string(),
                        style: TextStyle {
                            font_size: 20.,
                            font: my_assets.font_pixel.clone(),
                            color: Color::WHITE,
                        },
                    }
                    ],
                    alignment: TextAlignment::CENTER,
                },
                style: Style {
                    size: Size::new(Val::Px(400.), Val::Auto),
                    margin: UiRect::new(Val::Auto, Val::Auto, Val::Px(40.), Val::Px(40.)),
                    ..Default::default()
                },
                ..Default::default()
            });

            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Or don't.".to_string(),
                        style: TextStyle {
                            font_size: 20.,
                            font: my_assets.font_blocks.clone(),
                            color: Color::WHITE,
                        },
                    }
                    ],
                    alignment: TextAlignment::CENTER,
                },
                ..Default::default()
            });

            parent.spawn_bundle(ButtonBundle {
                color: Color::TEAL.into(),
                style: Style {
                    margin: UiRect::all(Val::Px(40.)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(PlayAgainButton)
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "Play again".to_string(),
                            style: TextStyle {
                                font_size: 40.,
                                font: my_assets.font_blocks.clone(),
                                color: Color::WHITE,
                            },
                        }
                        ],
                        alignment: TextAlignment::CENTER,
                    },
                    style: Style {
                        margin: UiRect::all(Val::Auto),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });
        });
    }).id();

    commands.entity(layout).push_children(&[popup]);
}

pub fn play_again_button(
    button: Query<&Interaction, With<PlayAgainButton>>,
    mut app_state: ResMut<State<AppState>>,
) {
    for interaction in button.iter() {
        if let Interaction::Clicked = interaction {
            app_state.set(AppState::InGame).ok();
        }
    }
}

pub fn cleanup(mut commands: Commands, query: Query<Entity, Without<Parent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn next_customer(
    mut commands: Commands,
    mut line: ResMut<Line>,
    current_customer: Query<Entity, With<CurrentCustomer>>,
) {
    if current_customer.is_empty() && !line.0.is_empty() {
        let current_first = line.0.pop_front().unwrap();
        line.0.push_back(current_first);

        let new_first = line.0.front().unwrap();
        commands
            .entity(*new_first)
            .insert(CurrentCustomer)
            .insert(Visibility { is_visible: true });
    }
}

pub fn fill_line(
    mut commands: Commands,
    mut line: ResMut<Line>,
    atlases: Res<Atlases>,
    faces_metadata: Res<FacesMetadata>,
    hair_metadata: Res<HairMetadata>,
) {
    let mut rng = rand::thread_rng();
    if line.0.length() < 4 {
        let new_customer = commands
            .spawn_bundle(SpatialBundle {
                transform: Transform::from_translation(Vec3::new(0., 150., 0.)),
                visibility: Visibility { is_visible: false },
                ..default()
            })
            .insert(Taste::random())
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

                // Pick random hair style
                let index = rng.gen_range(0..hair_metadata.names.len());
                let anchor = hair_metadata.anchor(index);
                parent.spawn_bundle(SpriteSheetBundle {
                    texture_atlas: atlases.hair_atlas.clone(),
                    sprite: TextureAtlasSprite {
                        index,
                        anchor,
                        ..Default::default()
                    },
                    ..Default::default()
                });
            })
            .id();

        line.0.push_back(new_customer);
    }
}
