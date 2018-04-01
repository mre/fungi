use core::{Attacks, Strength};
use core::map::Distance;

/// Active ability.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Ability {
    Knockback,
    Club,
    Jump(Jump),
    Poison,
    Explode,
    ExplodeFire,
    ExplodePoison,
    Bomb(Bomb),
    BombFire(BombFire),
    BombPoison(BombPoison),
    Summon(Summon),
    Vanish,
    Dash,
    Rage(Rage),
    Heal(Heal),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Summon(pub i32); // TODO: i32 -> ???

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Jump(pub Distance);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Rage(pub Attacks);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Heal(pub Strength);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Bomb(pub Distance);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BombPoison(pub Distance);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BombFire(pub Distance);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Status {
    Ready,
    Cooldown(i32), // TODO: i32 -> Rounds
}

impl Status {
    pub fn update(&mut self) {
        if let Status::Cooldown(ref mut rounds) = *self {
            *rounds -= 1;
        }
        if *self == Status::Cooldown(0) {
            *self = Status::Ready;
        }
    }
}

fn default_status() -> Status {
    Status::Ready
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RechargeableAbility {
    pub ability: Ability,

    #[serde(default = "default_status")]
    pub status: Status,

    pub base_cooldown: i32, // TODO: i32 -> Rounds
}

impl Ability {
    pub fn to_string(&self) -> String {
        match *self {
            Ability::Knockback => format!("Knockback"),
            Ability::Club => format!("Club"),
            Ability::Jump(a) => format!("Jump-{}", (a.0).0),
            Ability::Poison => format!("Poison"),
            Ability::Explode => format!("Explode"),
            Ability::ExplodeFire => format!("Explode Fire"),
            Ability::ExplodePoison => format!("Explode Poison"),
            Ability::Bomb(a) => format!("Bomb-{}", (a.0).0),
            Ability::BombFire(a) => format!("Fire bomb-{}", (a.0).0),
            Ability::BombPoison(a) => format!("Poison bomb-{}", (a.0).0),
            Ability::Vanish => format!("Vanish"),
            Ability::Summon(a) => format!("Summon-{}", a.0),
            Ability::Dash => format!("Dash"),
            Ability::Rage(a) => format!("Rage-{}", (a.0).0),
            Ability::Heal(a) => format!("Heal-{}", (a.0).0),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PassiveAbility {
    HeavyImpact,
    SpawnPoisonCloudOnDeath,
    Burn,
    Poison,
    SpikeTrap,
    PoisonAttack,
    Regenerate(Regenerate),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Regenerate(pub Strength);

impl PassiveAbility {
    pub fn to_string(&self) -> String {
        match *self {
            PassiveAbility::HeavyImpact => format!("Heavy impact"),
            PassiveAbility::SpawnPoisonCloudOnDeath => format!("Spawn a poison cloud on death"),
            PassiveAbility::Burn => format!("Burn"),
            PassiveAbility::Poison => format!("Poison"),
            PassiveAbility::SpikeTrap => format!("SpikeTrap"),
            PassiveAbility::PoisonAttack => format!("Poison attack"),
            PassiveAbility::Regenerate(a) => format!("Regenerate-{}", (a.0).0),
        }
    }
}
