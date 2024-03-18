use anyhow::anyhow;
use regex::Regex;
use std::{cmp::min, str::FromStr};

use super::calculator_logger::{CombatStart, Damage, SubterfugeLog, SubterfugeLogger};
use rand::Rng;

const MAX_ATTACKER_DIE: i32 = 3;
const MAX_DEFENDER_DIE: i32 = 2;
const D6: i32 = 6;
const D20: i32 = 20;

#[cfg(test)]
#[path = "./calculator_test.rs"]
mod calculator_test;

impl Calculator {
    pub fn new(fort: bool, ark: bool, log: bool) -> Self {
        Calculator {
            fort,
            ark,
            log: match log {
                true => Some(SubterfugeLogger::new()),
                false => None,
            },
        }
    }

    pub fn do_combat(&self, attacker: Combatant, defender: Combatant) -> bool {
        if let (Combatant::Units(attacker_units), Combatant::Units(defender_units)) =
            (attacker, defender)
        {
            self.do_unit_combat(attacker_units, defender_units)
        } else {
            self.do_hero_combat(attacker, defender)
        }
    }

    fn get_dice(&self, dice_count: i32, dice_sides: i32) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let mut res: Vec<i32> = (0..dice_count)
            .map(|_| rng.gen_range(0..dice_sides))
            .collect();

        res.sort();
        res.reverse();
        return res;
    }

    // returns true if attackers win
    fn do_unit_combat(&self, attackers: i32, defenders: i32) -> bool {
        let mut atk = attackers;
        let mut def = defenders;
        while atk > 0 && def > 0 {
            let dmg = self.get_unit_combat_damage(atk, def);
            atk -= dmg.attack_damage;
            def -= dmg.defense_damage;
        }
        if atk <= 0 {
            self.add_log(SubterfugeLog::AttackerVictory(false));
            return false;
        } else {
            self.add_log(SubterfugeLog::AttackerVictory(true));
            return true;
        }
    }

    fn get_unit_combat_damage(&self, attackers: i32, defenders: i32) -> Damage {
        let mut attacker_die = self.get_dice(min(attackers, MAX_ATTACKER_DIE), D6);
        let mut defender_die = self.get_dice(min(defenders, MAX_DEFENDER_DIE), D6);
        self.add_log(SubterfugeLog::AttackerRoll(attacker_die.clone()));
        self.add_log(SubterfugeLog::DefenderRoll(defender_die.clone()));

        let mut damage = Damage {
            attack_damage: 0,
            defense_damage: 0,
        };

        if self.fort && self.ark {
            attacker_die[0] += 1;
        } else if self.fort {
            defender_die[0] += 1;
        }

        defender_die
            .into_iter()
            .zip(attacker_die.into_iter())
            .for_each(|pair| {
                if pair.0 >= pair.1 {
                    damage.attack_damage += 1
                } else {
                    damage.defense_damage += 1
                }
            });
        self.add_log(SubterfugeLog::Damage(damage.clone()));
        return damage;
    }

    fn add_log(&self, log: SubterfugeLog) {
        if let Some(logger) = &self.log {
            logger.add_log(log)
        }
    }

    fn do_hero_combat(&self, mut attacker: Combatant, mut defender: Combatant) -> bool {
        if matches!(attacker, Combatant::Units(_)) && matches!(defender, Combatant::Units(_)) {
            panic!("units cannot fight units in hero combat")
        }
        self.add_log(SubterfugeLog::CombatStart(CombatStart {
            attacker: attacker.clone(),
            defender: defender.clone(),
        }));

        while attacker.get_health() > 0 && defender.get_health() > 0 {
            let attacker_die = self.get_dice(1, D20)[0] + attacker.get_attack();
            let defender_die = self.get_dice(1, D20)[0]
                + defender.get_attack()
                + if self.fort == true { 1 } else { 0 };
            self.add_log(SubterfugeLog::AttackerRoll(vec![attacker_die]));
            self.add_log(SubterfugeLog::DefenderRoll(vec![defender_die]));
            if defender_die >= attacker_die {
                attacker = attacker.do_damage();
                self.add_log(SubterfugeLog::Damage(Damage {
                    attack_damage: 1,
                    defense_damage: 0,
                }));
            } else {
                defender = defender.do_damage();
                self.add_log(SubterfugeLog::Damage(Damage {
                    attack_damage: 0,
                    defense_damage: 1,
                }));
            }
        }
        if attacker.get_health() == 0 {
            self.add_log(SubterfugeLog::AttackerVictory(false));
            return false;
        } else {
            self.add_log(SubterfugeLog::AttackerVictory(true));
            return true;
        }
    }

    pub fn get_log(&self) -> Option<&SubterfugeLogger> {
        match &self.log {
            Some(log) => Some(log),
            None => None,
        }
    }
}

#[derive(Debug)]
pub struct Calculator {
    fort: bool,
    ark: bool,
    log: Option<SubterfugeLogger>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Combatant {
    Units(i32),
    Hero(Hero),
}

// impl clap::ValueEnum for Combatant {
//     fn value_variants<'a>() -> &'a [Self] {
//         &[
//             Combatant::Units(0),
//             Combatant::Hero(Hero {
//                 attack: 0,
//                 health: 0,
//             }),
//         ]
//     }

//     fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
//         match self {
//             Self::Units(_) => Some(clap::builder::PossibleValue::new("units")),
//             Self::Hero(_) => Some(clap::builder::PossibleValue::new("hero")),
//         }
//     }
// }

impl FromStr for Combatant {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let combatant_regex = Regex::new(r"^(\d+)$|^(\d+), ?(\d+)$").unwrap();
        let res = match combatant_regex.captures(s) {
            Some(res) => res,
            None => return Err(anyhow!("Couldn't parse")),
        };
        // if we get a match in the first capture group, it's just units
        if let Some(unit_str) = res.get(1) {
            return Ok(Combatant::Units(unit_str.as_str().parse::<i32>()?));
        } else {
            let hero = Hero {
                // unwrapping because this shouldn't happen
                attack: res.get(2).unwrap().as_str().parse::<i32>()?,
                health: res.get(3).unwrap().as_str().parse::<i32>()?,
            };
            return Ok(Combatant::Hero(hero));
        }
    }
}

impl Combatant {
    fn get_health(&self) -> i32 {
        match self {
            Self::Units(units) => *units,
            Self::Hero(hero) => hero.health,
        }
    }
    fn get_attack(&self) -> i32 {
        match self {
            Self::Units(units) => *units,
            Self::Hero(hero) => hero.attack,
        }
    }

    // only one damage at a time
    fn do_damage(&self) -> Combatant {
        match self {
            Combatant::Units(units) => Combatant::Units(units - 1),
            Combatant::Hero(hero) => Combatant::Hero(Hero {
                health: hero.health - 1,
                attack: hero.attack,
            }),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Hero {
    pub attack: i32,
    pub health: i32,
}
