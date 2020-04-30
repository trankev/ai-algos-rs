const EXPLORATION_CONSTANT: f32 = 1.41;

pub fn uct_value(total_visits: f32, node_visits: f32, node_wins: f32) -> f32 {
    debug_assert!(total_visits != 0.0);
    if node_visits == 0.0 {
        return f32::INFINITY;
    }
    node_wins / node_visits + EXPLORATION_CONSTANT * (total_visits.ln() / node_visits).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use more_asserts::assert_gt;

    #[test]
    fn test_no_visits() {
        let result = uct_value(10.0, 0.0, 0.0);
        assert_eq!(result, f32::INFINITY);
    }

    #[test]
    fn test_favour_not_visited() {
        let value_a = uct_value(10.0, 9.0, 8.0);
        let value_b = uct_value(10.0, 1.0, 0.0);
        assert_gt!(value_b, value_a);
    }

    #[test]
    fn test_favour_wins() {
        let value_a = uct_value(100.0, 50.0, 75.0);
        let value_b = uct_value(100.0, 50.0, 45.0);
        assert_gt!(value_a, value_b);
    }
}
