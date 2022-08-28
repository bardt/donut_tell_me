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
    const SPRITES_COUNT: usize = 6;

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
    const SPRITES_COUNT: usize = 7;

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
        let stars = [
            self.bases[base.0],
            self.glazing[glazing.0],
            self.sprinkles[sprinkles.0],
        ];

        // Returns 0.0..1.0
        let weight = |r| ((r as f32 - 4.) * 2. + 7.) / 9.;

        let average_weight: f32 = stars.into_iter().map(weight).sum::<f32>() / stars.len() as f32;

        (average_weight * 5.).round() as usize
    }

    #[allow(dead_code)]
    pub fn all() -> Self {
        Self {
            bases: [5; Base::SPRITES_COUNT],
            glazing: [5; Glazing::SPRITES_COUNT],
            sprinkles: [5; Sprinkles::SPRITES_COUNT],
        }
    }

    #[allow(dead_code)]
    pub fn random() -> Self {
        use rand::distributions::WeightedIndex;
        use rand::prelude::*;

        let choices = [1, 2, 3, 4, 5];
        let weights = [1, 2, 5, 6, 3];
        let dist = WeightedIndex::new(&weights).unwrap();
        let mut rng = rand::thread_rng();

        Taste {
            bases: [0; Base::SPRITES_COUNT].map(|_| choices[dist.sample(&mut rng)]),
            glazing: [0; Glazing::SPRITES_COUNT].map(|_| choices[dist.sample(&mut rng)]),
            sprinkles: [0; Sprinkles::SPRITES_COUNT].map(|_| choices[dist.sample(&mut rng)]),
        }
    }
}

#[test]
fn test_donut_ranking() {
    let donut = DonutBundle::default();

    assert_eq!(
        Taste::default().rank(&donut.base, &donut.glazing, &donut.sprinkles),
        0
    );
    assert_eq!(
        Taste::all().rank(&donut.base, &donut.glazing, &donut.sprinkles),
        5
    );

    let mut taste = Taste::default();
    taste.bases[0] = 5;
    taste.glazing[0] = 5;
    taste.sprinkles[0] = 5;
    assert_eq!(taste.rank(&donut.base, &donut.glazing, &donut.sprinkles), 5);

    taste.bases[0] = 4;
    taste.glazing[0] = 5;
    taste.sprinkles[0] = 5;
    assert_eq!(taste.rank(&donut.base, &donut.glazing, &donut.sprinkles), 5);

    taste.bases[0] = 4;
    taste.glazing[0] = 4;
    taste.sprinkles[0] = 5;
    assert_eq!(taste.rank(&donut.base, &donut.glazing, &donut.sprinkles), 4);

    taste.bases[0] = 2;
    taste.glazing[0] = 5;
    taste.sprinkles[0] = 5;
    assert_eq!(taste.rank(&donut.base, &donut.glazing, &donut.sprinkles), 4);

    taste.bases[0] = 3;
    taste.glazing[0] = 4;
    taste.sprinkles[0] = 3;
    assert_eq!(taste.rank(&donut.base, &donut.glazing, &donut.sprinkles), 3);

    taste.bases[0] = 2;
    taste.glazing[0] = 4;
    taste.sprinkles[0] = 4;
    assert_eq!(taste.rank(&donut.base, &donut.glazing, &donut.sprinkles), 3);

    taste.bases[0] = 1;
    taste.glazing[0] = 4;
    taste.sprinkles[0] = 5;
    assert_eq!(taste.rank(&donut.base, &donut.glazing, &donut.sprinkles), 3);

    taste.bases[0] = 1;
    taste.glazing[0] = 2;
    taste.sprinkles[0] = 3;
    assert_eq!(taste.rank(&donut.base, &donut.glazing, &donut.sprinkles), 2);

    taste.bases[0] = 1;
    taste.glazing[0] = 2;
    taste.sprinkles[0] = 2;
    assert_eq!(taste.rank(&donut.base, &donut.glazing, &donut.sprinkles), 1);
}

#[derive(Component)]
pub struct CurrentCustomer;

#[derive(Component, Default)]
pub struct TransactionLog;

#[derive(Bundle)]
pub struct SalesLogBundle {
    pub sales_log: TransactionLog,
    pub children: Children,

    #[bundle]
    pub spatial: SpatialBundle,
}

impl Default for SalesLogBundle {
    fn default() -> Self {
        Self {
            sales_log: TransactionLog,
            children: Children::with(&[]),
            spatial: SpatialBundle::default(),
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum Emo {
    Angry = 15,
    Happy = 16,
    Sad = 17,
    Love = 18,
    Heartbroken = 19,
}

#[derive(Component)]
pub struct DisappearingTimer(pub Timer);

#[derive(Component)]
pub struct Photo(pub Handle<Image>);

#[derive(Component)]
pub struct PhotoCamera;
pub struct PhotosTakenEvent;

#[derive(Component)]
pub struct Regular;

#[derive(Component)]
pub struct PlayAgainButton;

#[derive(Component)]
pub struct Layout;
