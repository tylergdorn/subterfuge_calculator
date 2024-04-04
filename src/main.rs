use clap::Parser;
use odd_calculator::{calculator::Calculator, calculator::Combatant, CombatSimulator};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::plotter::plot_result;

mod odd_calculator;
mod plotter;
fn main() {
    let args = Args::parse();
    let cs = CombatSimulator::new(args.simulations);
    let calculator = Calculator::new(args.fort, args.ark, args.log);
    if args.plot {
        if let Combatant::Units(attacker) = args.attacker {
            // let mut res_vec = Vec::new();
            let res: Vec<(f32, f32)> = (0..25)
                .into_par_iter()
                .map(|delta| {
                    (
                        (attacker + delta) as f32,
                        cs.simulate_combat_multi(
                            &calculator,
                            Combatant::Units(attacker + delta),
                            args.defender,
                        )
                        .to_percent_f32(),
                    )
                })
                .collect();
            plot_result(0.0..25.0, 0.0..100.0, res).unwrap();
        }
        // for i in 0..25 {
        //     if let Combatant::Units(attacker) = args.attacker {
        //         let res = cs.simulate_combat_multi(
        //             &calculator,
        //             Combatant::Units(attacker + i),
        //             args.defender,
        //         );
        //         res_vec.push(((attacker + i) as f32, res.to_percent_f32()))
        //     }
        // }
        return;
    }
    let res = match args.multi {
        true => cs.simulate_combat_multi(&calculator, args.attacker, args.defender),
        false => cs.simulate_combat(&calculator, args.attacker, args.defender),
    };
    println!("attacker has {}% chance of winning", res.to_percent());
    if args.log {
        println!("{}", calculator.get_log().unwrap())
    }
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    fort: bool,
    #[arg(short, long, default_value_t = false)]
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
    #[arg(short, long, default_value_t = false)]
    multi: bool,
    #[arg(short, long, default_value_t = false)]
    plot: bool,
}
