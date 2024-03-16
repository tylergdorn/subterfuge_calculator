use std::{fmt::Display, sync::Mutex};

#[derive(Debug, Clone)]
pub enum SubterfugeLog {
    AttackerRoll(Vec<i32>),
    DefenderRoll(Vec<i32>),
    Damage(Damage),
    AttackerVictory(bool),
}

#[derive(Debug)]
pub struct SubterfugeLogger {
    log: Mutex<Vec<SubterfugeLog>>,
}

impl SubterfugeLogger {
    pub fn new() -> Self {
        SubterfugeLogger {
            log: Mutex::new(Vec::new()),
        }
    }
    pub fn add_log(&self, log: SubterfugeLog) {
        self.log.lock().unwrap().push(log);
    }
}

impl Display for SubterfugeLogger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v: String = self
            .log
            .lock()
            .unwrap()
            .iter()
            .map(|log| format!("{}\n", log))
            .collect();
        f.write_str(&v)
    }
}

#[derive(Debug, Clone)]
pub struct Damage {
    pub attack_damage: i32,
    pub defense_damage: i32,
}

impl Display for SubterfugeLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            SubterfugeLog::AttackerRoll(die) => format!("attacker rolled: {:#?}", die),
            SubterfugeLog::DefenderRoll(die) => format!("defender rolled: {:#?}", die),
            SubterfugeLog::Damage(damage) => format!(
                "attacker dealt: {} damage, defender dealt: {} damage",
                damage.attack_damage, damage.defense_damage
            ),
            SubterfugeLog::AttackerVictory(win) => match win {
                true => "Attacker wins.".into(),
                false => "Defender wins.".into(),
            },
        };
        f.write_str(&str)
    }
}
