use crate::odd_calculator::calculator::{Calculator, Combatant};

pub mod calculator;
pub mod calculator_logger;

pub struct CombatSimulator {
    iterations: i32,
}
impl CombatSimulator {
    pub fn new(iterations: i32) -> Self {
        Self { iterations }
    }

    pub fn simulate_combat(
        &self,
        calculator: &Calculator,
        attacker: Combatant,
        defender: Combatant,
    ) -> SimulationResults {
        let attacker_wins = (0..self.iterations)
            .map(|_| calculator.do_combat(attacker, defender))
            .fold(0, |acc, item| if item { acc + 1 } else { acc });
        SimulationResults {
            attacker_wins,
            total_iterations: self.iterations,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SimulationResults {
    pub attacker_wins: i32,
    pub total_iterations: i32,
}

impl SimulationResults {
    pub fn to_percent(self) -> String {
        format!(
            "{:.2}",
            (self.attacker_wins as f32 / self.total_iterations as f32) * 100.0
        )
    }
}
