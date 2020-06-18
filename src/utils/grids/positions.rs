pub struct Positions {
    dimensions: Vec<isize>,
    pub current_position: Vec<isize>,
}

impl Positions {
    pub fn new(dimensions: Vec<isize>) -> Positions {
        let mut current_position = vec![0; dimensions.len()];
        if !current_position.is_empty() {
            let last_index = current_position.len() - 1;
            current_position[last_index] = -1;
        }
        Positions {
            dimensions,
            current_position,
        }
    }

    pub fn iterate(&mut self) -> Option<()> {
        let iterator = self
            .current_position
            .iter_mut()
            .rev()
            .zip(self.dimensions.iter().rev());
        for (value, dimension) in iterator {
            *value += 1;
            if *value < *dimension as isize {
                return Some(());
            }
            *value = 0;
        }
        None
    }
}

impl Iterator for Positions {
    type Item = Vec<isize>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterate() {
            Some(()) => Some(self.current_position.clone()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positions() {
        let iterator = Positions::new(vec![3, 3]);
        let result = iterator.collect::<Vec<_>>();
        let expected = vec![
            vec![0, 0],
            vec![0, 1],
            vec![0, 2],
            vec![1, 0],
            vec![1, 1],
            vec![1, 2],
            vec![2, 0],
            vec![2, 1],
            vec![2, 2],
        ];
        assert_eq!(result, expected);
    }
}
