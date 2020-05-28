use super::edges;
use super::nodes;
use crate::algos;
use crate::interface::rulesets;

use petgraph::graph;

pub fn play_scores<RuleSet: rulesets::RuleSetTrait>(
    tree: &graph::Graph<nodes::Node<RuleSet::State>, edges::Edge<RuleSet::Ply>>,
    parent: graph::NodeIndex<u32>,
) -> Vec<algos::PlyConsideration<RuleSet::Ply>>
where
    RuleSet::State: rulesets::TurnByTurnState,
{
    let mut scores = tree
        .neighbors(parent)
        .map(|node_index| {
            let node_weight = tree.node_weight(node_index).unwrap();
            let edge = tree.find_edge(parent, node_index).unwrap();
            let edge_weight = tree.edge_weight(edge).unwrap();
            let follow_up = best_play::<RuleSet>(tree, node_index);
            algos::PlyConsideration {
                ply: edge_weight.ply,
                score: node_weight.score(),
                win_rate: node_weight.win_rate(),
                draw_rate: node_weight.draw_rate(),
                follow_up,
            }
        })
        .collect::<Vec<_>>();
    scores.sort_by(|consideration_a, consideration_b| {
        consideration_a
            .score
            .partial_cmp(&consideration_b.score)
            .unwrap()
            .reverse()
    });
    scores
}

fn best_play<RuleSet: rulesets::RuleSetTrait>(
    tree: &graph::Graph<nodes::Node<RuleSet::State>, edges::Edge<RuleSet::Ply>>,
    mut current_node: graph::NodeIndex<u32>,
) -> Vec<RuleSet::Ply>
where
    RuleSet::State: rulesets::TurnByTurnState,
{
    let mut result = Vec::new();
    loop {
        let neighbours = tree.neighbors(current_node).map(|node_index| {
            let node_weight = tree.node_weight(node_index).unwrap();
            let edge = tree.find_edge(current_node, node_index).unwrap();
            let edge_weight = tree.edge_weight(edge).unwrap();
            (node_weight.score(), node_index, edge_weight.ply)
        });
        let best_neighbour = {
            neighbours
                .max_by(|(score_a, _, _), (score_b, _, _)| score_a.partial_cmp(score_b).unwrap())
        };
        current_node = match best_neighbour {
            Some((_, node_index, ply)) => {
                result.push(ply);
                node_index
            }
            None => break,
        }
    }
    result
}
