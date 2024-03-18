use std::str::FromStr;

use crate::odd_calculator::{calculator::Hero, *};

#[test]
fn test_test() {
    let c = Calculator::new(false, false, false);
    // let mut rng = rand::thread_rng();
    // let nums = vec![0..6];
    // nums.choose(&mut rng);
    // println!("{:#?}", c.get_dice(3, D6));
    println!("{:#?}", c.do_unit_combat(10, 8));
}

#[test]
#[should_panic]
fn test_hero_combat_invalid() {
    let c = Calculator::new(false, false, false);
    c.do_hero_combat(Combatant::Units(0), Combatant::Units(0));
}

#[test]
fn test_hero_damage() {
    let mut hc = Combatant::Units(10);
    hc = hc.do_damage().do_damage();
    assert!(hc.get_health() == 8);
}

#[test]
fn test_deser_combatant() {
    let unit_input = "123";
    let hero_input = "12, 10";
    let hero = Combatant::from_str(hero_input).unwrap();
    let units = Combatant::from_str(unit_input).unwrap();
    assert_eq!(units, Combatant::Units(123));
    assert_eq!(
        hero,
        Combatant::Hero(Hero {
            attack: 12,
            health: 10
        })
    );
}
