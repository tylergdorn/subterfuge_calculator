use std::{cmp::min, sync::Mutex};

use rand::{seq::SliceRandom, Rng};

#[cfg(test)]
#[path = "./calculator_test.rs"]
mod calculator_test;

const MAX_ATTACKER_DIE: i32 = 3;
const MAX_DEFENDER_DIE: i32 = 2;
const D6: i32 = 6;
const D20: i32 = 20;

pub fn calculate_odds(attackers: i32, defenders: i32, defense: bool, arc: bool) -> f64 {
    rand::random::<i32>();
    let c = Calculator {
        fort: defense,
        arc,
        log: Vec::new().into(),
    };
    let iter = 100_000;
    let x = (0..iter)
        .map(|_| c.do_unit_combat(attackers, defenders))
        .fold(0, |acc, item| if item { acc + 1 } else { acc });
    let final_percent = (x as f32 / iter as f32) * 100.0;
    println!("{}%", final_percent);
    // println!("{:#?}", c.do_unit_combat(attackers, defenders);
    // println!("{:#?}", c.log.lock().unwrap());
    return 0.0;
}

impl Calculator {
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
            // println!(
            //     "atk {} def {} atk > 0 {} def > 0 {} atk > 0 && def > 0 {}",
            //     atk,
            //     def,
            //     atk > 0,
            //     def > 0,
            //     atk > 0 && def > 0
            // );
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

        if self.fort && self.arc {
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
        self.log.lock().unwrap().push(log)
    }

    fn do_hero_combat(&self, mut attacker: HeroCombatant, mut defender: HeroCombatant) -> Damage {
        if matches!(attacker, HeroCombatant::Units(_))
            && matches!(defender, HeroCombatant::Units(_))
        {
            panic!("units cannot fight units in hero combat")
        }

        let attacker_die = self.get_dice(D20, 1)[0];
        let defender_die = self.get_dice(D20, 1)[0];

        Damage {
            attack_damage: attacker_die,
            defense_damage: defender_die,
        }
    }
}

#[derive(Debug)]
pub struct Calculator {
    fort: bool,
    arc: bool,
    log: Mutex<Vec<SubterfugeLog>>,
}

#[derive(Debug, Clone)]
struct Damage {
    attack_damage: i32,
    defense_damage: i32,
}

#[derive(Debug)]
enum SubterfugeLog {
    AttackerRoll(Vec<i32>),
    DefenderRoll(Vec<i32>),
    Damage(Damage),
    AttackerVictory(bool),
}

#[derive(Debug, PartialEq)]
enum HeroCombatant {
    Units(i32),
    Hero(Hero),
}

impl HeroCombatant {
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
    fn do_damage(&self) -> HeroCombatant {
        match self {
            HeroCombatant::Units(units) => HeroCombatant::Units(units - 1),
            HeroCombatant::Hero(hero) => HeroCombatant::Hero(Hero {
                health: hero.health - 1,
                attack: hero.attack,
            }),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Hero {
    attack: i32,
    health: i32,
}
