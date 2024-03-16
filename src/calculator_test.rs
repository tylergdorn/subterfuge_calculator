use crate::odd_calculator::*;

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
    c.do_hero_combat(HeroCombatant::Units(0), HeroCombatant::Units(0));
}

#[test]
fn test_hero_damage() {
    let mut hc = HeroCombatant::Units(10);
    hc = hc.do_damage().do_damage();
    assert!(hc.get_health() == 8);
}
