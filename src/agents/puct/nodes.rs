use crate::interface::rulesets;

pub enum NodeStatus<RuleSet: rulesets::RuleSetTrait> {
    Terminal,
    Ongoing { children: Vec<Child<RuleSet>> },
}

pub struct Node<RuleSet: rulesets::RuleSetTrait> {
    pub status: NodeStatus<RuleSet>,
    pub value: f32,
    pub visits: f32,
}

pub struct Child<RuleSet: rulesets::RuleSetTrait> {
    pub ply: RuleSet::Ply,
    pub initial_prob: f32,
    pub visits: f32,
    pub qvalue: f32,
}

impl<RuleSet: rulesets::RuleSetTrait> Child<RuleSet> {
    pub fn new(ply: RuleSet::Ply, initial_prob: f32) -> Child<RuleSet> {
        Child {
            ply,
            initial_prob,
            visits: 0.0,
            qvalue: 0.0,
        }
    }

    pub fn ucb(&self, parent_visits: f32, cpuct: f32) -> f32 {
        self.qvalue + cpuct * self.initial_prob * (parent_visits / (self.visits + 1.0)).sqrt()
    }

    pub fn update(&mut self, target_value: f32) {
        self.qvalue = (self.visits * self.qvalue + target_value) / (self.visits + 1.0);
        self.visits += 1.0;
    }
}
