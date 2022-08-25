use bevy::prelude::*;

pub trait ToSpriteIndex {
    const START_SPRITE_INDEX: usize = 0;
    const SPRITES_COUNT: usize = 1;

    fn to_sprite_index(&self) -> usize {
        Self::START_SPRITE_INDEX
    }

    fn cycle_right(&mut self) -> &mut Self;
    fn cycle_left(&mut self) -> &mut Self;
}

#[derive(Component, PartialEq, Eq, Debug, Copy, Clone, Default)]
pub struct Base(pub usize);

impl ToSpriteIndex for Base {
    const START_SPRITE_INDEX: usize = 0;
    const SPRITES_COUNT: usize = 3;

    fn to_sprite_index(&self) -> usize {
        Self::START_SPRITE_INDEX + (self.0 as usize)
    }

    fn cycle_right(&mut self) -> &mut Self {
        self.0 = (self.0 + Self::SPRITES_COUNT + 1) % Self::SPRITES_COUNT;
        self
    }

    fn cycle_left(&mut self) -> &mut Self {
        self.0 = (self.0 + Self::SPRITES_COUNT - 1) % Self::SPRITES_COUNT;
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

#[derive(Component, Copy, Clone, Default)]
pub struct Glazing(pub usize);

impl ToSpriteIndex for Glazing {
    const START_SPRITE_INDEX: usize = 3;
    const SPRITES_COUNT: usize = 10;

    fn to_sprite_index(&self) -> usize {
        Self::START_SPRITE_INDEX + (self.0)
    }

    fn cycle_right(&mut self) -> &mut Self {
        self.0 = (self.0 + Self::SPRITES_COUNT + 1) % Self::SPRITES_COUNT;
        self
    }

    fn cycle_left(&mut self) -> &mut Self {
        self.0 = (self.0 + Self::SPRITES_COUNT - 1) % Self::SPRITES_COUNT;
        self
    }
}

#[derive(Component, Copy, Clone, Default)]
pub struct Sprinkles(pub usize);

impl ToSpriteIndex for Sprinkles {
    const START_SPRITE_INDEX: usize = 13;
    const SPRITES_COUNT: usize = 9;

    fn to_sprite_index(&self) -> usize {
        Self::START_SPRITE_INDEX + (self.0 as usize)
    }

    fn cycle_right(&mut self) -> &mut Self {
        self.0 = (self.0 + Self::SPRITES_COUNT + 1) % Self::SPRITES_COUNT;
        self
    }

    fn cycle_left(&mut self) -> &mut Self {
        self.0 = (self.0 + Self::SPRITES_COUNT - 1) % Self::SPRITES_COUNT;
        self
    }
}

#[derive(Component, Default)]
pub struct Donut;

#[derive(Bundle, Default)]
pub struct DonutBundle {
    pub donut: Donut,
    pub base: Base,
    pub glazing: Glazing,
    pub sprinkles: Sprinkles,

    #[bundle]
    pub spatial: SpatialBundle,
}

#[derive(Component)]
pub struct CookingDonut;

#[derive(Component, Default)]
pub struct Taste {
    bases: [usize; Base::SPRITES_COUNT],
    glazing: [usize; Glazing::SPRITES_COUNT],
    sprinkles: [usize; Sprinkles::SPRITES_COUNT],
}

impl Taste {
    pub fn rank(&self, base: &Base, glazing: &Glazing, sprinkles: &Sprinkles) -> usize {
        let base_rank = self.bases[base.0];
        let glazing_rank = self.glazing[glazing.0];
        let sprinkles_rank = self.sprinkles[sprinkles.0];

        // @TODO: adjust the formula
        let total_rank: usize = (base_rank + glazing_rank + sprinkles_rank) / 3;
        total_rank
    }

    pub fn random() -> Self {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();

        // @TODO: generate taste somewhat systematically
        let bases = [0; Base::SPRITES_COUNT].map(|_| rng.gen_range(1_usize..5_usize));
        let glazing = [0; Glazing::SPRITES_COUNT].map(|_| rng.gen_range(1_usize..5_usize));
        let sprinkles = [0; Sprinkles::SPRITES_COUNT].map(|_| rng.gen_range(1_usize..5_usize));

        Taste {
            bases,
            glazing,
            sprinkles,
        }
    }
}

#[test]
fn test_donut_ranking() {
    let donut = DonutBundle::default();

    let no_taste = Taste::default();
    assert_eq!(
        no_taste.rank(&donut.base, &donut.glazing, &donut.sprinkles),
        0
    );

    let mut exact_taste = Taste::default();
    exact_taste.bases[0] = 5;
    exact_taste.glazing[0] = 5;
    exact_taste.sprinkles[0] = 5;
    assert_eq!(
        exact_taste.rank(&donut.base, &donut.glazing, &donut.sprinkles),
        5
    );
}

#[derive(Component)]
pub struct CurrentCustomer;

#[derive(Component, Default)]
pub struct SalesLog;

#[derive(Bundle)]
pub struct SalesLogBundle {
    pub sales_log: SalesLog,
    pub children: Children,

    #[bundle]
    pub spatial: SpatialBundle,
}

impl Default for SalesLogBundle {
    fn default() -> Self {
        Self {
            sales_log: SalesLog,
            children: Children::with(&[]),
            spatial: SpatialBundle::default(),
        }
    }
}

#[derive(Component, Clone, Copy)]
pub enum Emo {
    Angry = 15,
    Happy = 16,
    Sad = 17,
    Love = 18,
    Heartbroken = 19,
}

#[derive(Component)]
pub struct DisappearingTimer(pub Timer);
