use bevy::prelude::*;

pub trait ToSpriteIndex {
    const START_SPRITE_INDEX: usize = 0;
    const SPRITES_COUNT: usize = 1;

    fn to_sprite_index(self: &Self) -> usize {
        Self::START_SPRITE_INDEX
    }

    fn cycle_right(self: &mut Self) -> &mut Self;
    fn cycle_left(self: &mut Self) -> &mut Self;
}

#[derive(Component, PartialEq, Debug)]
pub struct Base(pub i32);

impl ToSpriteIndex for Base {
    const START_SPRITE_INDEX: usize = 0;
    const SPRITES_COUNT: usize = 3;

    fn to_sprite_index(self: &Self) -> usize {
        Self::START_SPRITE_INDEX + (self.0 as usize)
    }

    fn cycle_right(self: &mut Self) -> &mut Self {
        self.0 = (self.0 + Self::SPRITES_COUNT as i32 + 1) % Self::SPRITES_COUNT as i32;
        self
    }

    fn cycle_left(self: &mut Self) -> &mut Self {
        self.0 = (self.0 + Self::SPRITES_COUNT as i32 - 1) % Self::SPRITES_COUNT as i32;
        self
    }
}

#[test]
fn test_base_cycling() {
    let mut base = Base(0);

    assert_eq!(base.cycle_left(), &Base(2));
    assert_eq!(base.cycle_left(), &Base(1));
    assert_eq!(base.cycle_right(), &Base(2));
    assert_eq!(base.cycle_right(), &Base(0));
}

#[derive(Component)]
pub struct Glazing(pub i32);

impl ToSpriteIndex for Glazing {
    const START_SPRITE_INDEX: usize = 3;
    const SPRITES_COUNT: usize = 10;

    fn to_sprite_index(self: &Self) -> usize {
        Self::START_SPRITE_INDEX + (self.0 as usize)
    }

    fn cycle_right(self: &mut Self) -> &mut Self {
        self.0 = (self.0 + Self::SPRITES_COUNT as i32 + 1) % Self::SPRITES_COUNT as i32;
        self
    }

    fn cycle_left(self: &mut Self) -> &mut Self {
        self.0 = (self.0 + Self::SPRITES_COUNT as i32 - 1) % Self::SPRITES_COUNT as i32;
        self
    }
}

#[derive(Component)]
pub struct Sprinkles(pub i32);

impl ToSpriteIndex for Sprinkles {
    const START_SPRITE_INDEX: usize = 13;
    const SPRITES_COUNT: usize = 9;

    fn to_sprite_index(self: &Self) -> usize {
        Self::START_SPRITE_INDEX + (self.0 as usize)
    }

    fn cycle_right(self: &mut Self) -> &mut Self {
        self.0 = (self.0 + Self::SPRITES_COUNT as i32 + 1) % Self::SPRITES_COUNT as i32;
        self
    }

    fn cycle_left(self: &mut Self) -> &mut Self {
        self.0 = (self.0 + Self::SPRITES_COUNT as i32 - 1) % Self::SPRITES_COUNT as i32;
        self
    }
}

#[derive(Component)]
pub struct Stripes(pub i32);

impl ToSpriteIndex for Stripes {
    const START_SPRITE_INDEX: usize = 15;
    const SPRITES_COUNT: usize = 4;

    fn to_sprite_index(self: &Self) -> usize {
        Self::START_SPRITE_INDEX + (self.0 as usize)
    }

    fn cycle_right(self: &mut Self) -> &mut Self {
        self.0 = (self.0 + Self::SPRITES_COUNT as i32 + 1) % Self::SPRITES_COUNT as i32;
        self
    }

    fn cycle_left(self: &mut Self) -> &mut Self {
        self.0 = (self.0 + Self::SPRITES_COUNT as i32 - 1) % Self::SPRITES_COUNT as i32;
        self
    }
}

#[derive(Bundle)]
struct DonutBundle {
    base: Base,
    glazing: Glazing,
    sprinkles: Sprinkles,
    stripes: Stripes,
}
