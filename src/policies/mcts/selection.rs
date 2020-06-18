use super::nodes;
use super::uct_value;
use crate::interface::rulesets;
use petgraph::graph;

pub fn select<State: rulesets::StateTrait, Edge>(
    tree: &graph::Graph<nodes::Node<State>, Edge>,
    node: graph::NodeIndex<u32>,
) -> graph::NodeIndex<u32> {
    let weight = tree.node_weight(node).unwrap();
    if !weight.is_visited() {
        return node;
    }
    let best_neighbour = tree
        .neighbors(node)
        .filter_map(|child_index| {
            let child_weight = tree.node_weight(child_index).unwrap();
            if child_weight.expanding {
                return None;
            }
            let value =
                uct_value::uct_value(weight.visits, child_weight.visits, child_weight.score());
            Some((child_index, value))
        })
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
    match best_neighbour {
        Some((child_index, _)) => select(tree, child_index),
        None => match tree.neighbors(node).next() {
            Some(child_index) => child_index,
            None => node,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests;

    type EmptyStateGraph = graph::Graph<nodes::Node<tests::EmptyState>, ()>;

    #[test]
    fn test_no_children() {
        let mut tree = EmptyStateGraph::new();
        let root = tree.add_node(nodes::Node::new(
            tests::EmptyState::new(),
            rulesets::Status::Ongoing,
            0,
        ));
        let result = select(&tree, root);
        assert_eq!(result, root);
    }

    #[test]
    fn test_no_visits() {
        let mut tree = EmptyStateGraph::new();

        let root_weight = nodes::Node::new_visited(tests::EmptyState::new(), 10, 0, 0, 0);
        let root_index = tree.add_node(root_weight);

        let first_weight = nodes::Node::new_visited(tests::EmptyState::new(), 10, 10, 0, 0);
        let first_index = tree.add_node(first_weight);
        tree.add_edge(root_index, first_index, ());

        let second_index = tree.add_node(nodes::Node::new(
            tests::EmptyState::new(),
            rulesets::Status::Ongoing,
            0,
        ));
        tree.add_edge(root_index, second_index, ());

        let result = select(&tree, root_index);
        assert_eq!(result, second_index);
    }

    #[test]
    fn test_few_visits() {
        let mut tree = EmptyStateGraph::new();

        let root_weight = nodes::Node::new_visited(tests::EmptyState::new(), 10, 0, 0, 0);
        let root_index = tree.add_node(root_weight);

        let first_weight = nodes::Node::new_visited(tests::EmptyState::new(), 1, 0, 0, 0);
        let first_index = tree.add_node(first_weight);
        tree.add_edge(root_index, first_index, ());

        let second_weight = nodes::Node::new_visited(tests::EmptyState::new(), 9, 8, 0, 0);
        let second_index = tree.add_node(second_weight);
        tree.add_edge(root_index, second_index, ());

        let result = select(&tree, root_index);
        assert_eq!(result, first_index);
    }

    #[test]
    fn test_several_visits() {
        let mut tree = EmptyStateGraph::new();

        let root_weight = nodes::Node::new_visited(tests::EmptyState::new(), 100, 50, 0, 0);
        let root_index = tree.add_node(root_weight);

        let first_weight = nodes::Node::new_visited(tests::EmptyState::new(), 50, 40, 0, 0);
        let first_index = tree.add_node(first_weight);
        tree.add_edge(root_index, first_index, ());

        let second_weight = nodes::Node::new_visited(tests::EmptyState::new(), 50, 60, 0, 0);
        let second_index = tree.add_node(second_weight);
        tree.add_edge(root_index, second_index, ());

        let result = select(&tree, root_index);
        assert_eq!(result, first_index);
    }
}
