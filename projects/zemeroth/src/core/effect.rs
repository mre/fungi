use core::{Phase, PosHex, Strength};
use core::component::Component;

#[derive(Clone, Debug, Copy, PartialEq, Serialize, Deserialize)]
pub enum Duration {
    Forever,
    Rounds(i32),
}

impl Duration {
    pub fn is_over(&self) -> bool {
        match *self {
            Duration::Rounds(n) => n <= 0,
            Duration::Forever => false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TimedEffect {
    pub duration: Duration,
    pub phase: Phase,
    pub effect: LastingEffect,
}

/// Instant effects
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub enum Effect {
    Create(Create),
    Kill,
    Vanish,
    Stun,
    Heal(Heal),
    Wound(Wound),
    Knockback(Knockback),
    FlyOff(FlyOff), // TODO: flying boulders should make some damage
    Throw(Throw),
    Miss,
}

impl Effect {
    pub fn to_str(&self) -> &str {
        match *self {
            Effect::Create(_) => "Create",
            Effect::Kill => "Kill",
            Effect::Vanish => "Vanish",
            Effect::Stun => "Stun",
            Effect::Heal(_) => "Heal",
            Effect::Wound(_) => "Wound",
            Effect::Knockback(_) => "Knockback",
            Effect::FlyOff(_) => "Fly off",
            Effect::Throw(_) => "Throw",
            Effect::Miss => "Miss",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum LastingEffect {
    Poison,
    Stun,
}

impl LastingEffect {
    pub fn to_str(&self) -> &str {
        match *self {
            LastingEffect::Poison => "Poison",
            LastingEffect::Stun => "Stun",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Wound {
    pub damage: Strength,
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct Heal {
    pub strength: Strength,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Create {
    pub pos: PosHex,
    pub prototype: String,
    pub components: Vec<Component>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct FlyOff {
    pub from: PosHex,
    pub to: PosHex,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Throw {
    pub from: PosHex,
    pub to: PosHex,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Knockback {
    pub from: PosHex,
    pub to: PosHex,
}
