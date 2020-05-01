use super::nodes;
use super::uct_value;
use petgraph::stable_graph;

pub fn select<State, Edge>(
    tree: &stable_graph::StableGraph<nodes::Node<State>, Edge>,
    node: stable_graph::NodeIndex<u32>,
) -> stable_graph::NodeIndex<u32> {
    let weight = tree.node_weight(node).unwrap();
    if weight.visits == 0.0 {
        return node;
    }
    let neighbors = tree
        .neighbors(node)
        .map(|child_index| {
            let child_weight = tree.node_weight(child_index).unwrap();
            let value = uct_value::uct_value(
                weight.visits,
                child_weight.visits,
                child_weight.wins,
                child_weight.draws,
            );
            (child_index, value)
        })
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()); // assumes no NaN values
    match neighbors {
        Some((child_index, _)) => child_index,
        None => node,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc;

    #[test]
    fn test_no_children() {
        let mut tree = stable_graph::StableDiGraph::<nodes::Node<()>, ()>::new();
        let root = tree.add_node(nodes::Node::new(rc::Rc::new(())));
        let result = select(&tree, root);
        assert_eq!(result, root);
    }

    #[test]
    fn test_no_visits() {
        let mut tree = stable_graph::StableDiGraph::<nodes::Node<()>, ()>::new();

        let mut root_weight = nodes::Node::new(rc::Rc::new(()));
        root_weight.visits = 10.0;
        let root_index = tree.add_node(root_weight);

        let mut first_weight = nodes::Node::new(rc::Rc::new(()));
        first_weight.visits = 10.0;
        first_weight.wins = 10.0;
        let first_index = tree.add_node(first_weight);
        tree.add_edge(root_index, first_index, ());

        let second_index = tree.add_node(nodes::Node::new(rc::Rc::new(())));
        tree.add_edge(root_index, second_index, ());

        let result = select(&tree, root_index);
        assert_eq!(result, second_index);
    }

    #[test]
    fn test_few_visits() {
        let mut tree = stable_graph::StableDiGraph::<nodes::Node<()>, ()>::new();

        let mut root_weight = nodes::Node::new(rc::Rc::new(()));
        root_weight.visits = 10.0;
        let root_index = tree.add_node(root_weight);

        let mut first_weight = nodes::Node::new(rc::Rc::new(()));
        first_weight.visits = 1.0;
        first_weight.wins = 0.0;
        let first_index = tree.add_node(first_weight);
        tree.add_edge(root_index, first_index, ());

        let mut second_weight = nodes::Node::new(rc::Rc::new(()));
        second_weight.visits = 9.0;
        second_weight.wins = 8.0;
        let second_index = tree.add_node(second_weight);
        tree.add_edge(root_index, second_index, ());

        let result = select(&tree, root_index);
        assert_eq!(result, first_index);
    }

    #[test]
    fn test_several_visits() {
        let mut tree = stable_graph::StableDiGraph::<nodes::Node<()>, ()>::new();

        let mut root_weight = nodes::Node::new(rc::Rc::new(()));
        root_weight.visits = 100.0;
        let root_index = tree.add_node(root_weight);

        let mut first_weight = nodes::Node::new(rc::Rc::new(()));
        first_weight.visits = 50.0;
        first_weight.wins = 40.0;
        let first_index = tree.add_node(first_weight);
        tree.add_edge(root_index, first_index, ());

        let mut second_weight = nodes::Node::new(rc::Rc::new(()));
        second_weight.visits = 50.0;
        second_weight.wins = 60.0;
        let second_index = tree.add_node(second_weight);
        tree.add_edge(root_index, second_index, ());

        let result = select(&tree, root_index);
        assert_eq!(result, second_index);
    }
}
