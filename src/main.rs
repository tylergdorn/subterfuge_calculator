use clap::Parser;
use odd_calculator::{
    calculator::Calculator, calculator::Combatant, calculator::Hero, CombatSimulator,
};

mod odd_calculator;
fn main() {
    let args = Args::parse();
    let cs = CombatSimulator::new(args.simulations);
    let calculator = Calculator::new(args.fort, args.ark, args.log);
    let res = cs.simulate_combat(&calculator, args.attacker, args.defender);
    // println!("{:#?}{:#?}", args.attacker, args.defender);
    println!("attacker has {}% chance of winning", res.to_percent());
    if args.log {
        println!("{}", calculator.get_log().unwrap())
    }
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value_t = false)]
    fort: bool,
    #[arg(long, default_value_t = false)]
    ark: bool,
    #[arg(short, long, default_value_t = 100_000)]
    simulations: i32,
    #[arg(short, long, default_value_t = false)]
    log: bool,
    #[arg(
        index = 1,
        help = "`attack,health` for hero, or just a number for units"
    )]
    attacker: Combatant,
    #[arg(
        index = 2,
        help = "`attack,health` for hero, or just a number for units"
    )]
    defender: Combatant,
}
