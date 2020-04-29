use crate::utils::grids::strips;

pub struct CellRuns {
    size: usize,
    strips: strips::StripIterator,
    current_strip: Vec<usize>,
    current_index: isize,
}

impl CellRuns {
    pub fn new(dimensions: Vec<isize>, size: usize) -> CellRuns {
        let strips = strips::StripIterator::new(dimensions.clone());
        CellRuns {
            size,
            strips,
            current_strip: vec![],
            current_index: 0,
        }
    }
}

impl Iterator for CellRuns {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_index += 1;
        if self.current_index as usize + self.size <= self.current_strip.len() {
            let slice = (self.current_index as usize)..(self.current_index as usize + self.size);
            return Some(self.current_strip[slice].into());
        }
        if let Some(strip) = self.strips.next() {
            self.current_strip = strip.collect();
            self.current_index = -1;
            return self.next();
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections;

    #[test]
    fn test_iteration() {
        let iterator = CellRuns::new(vec![3, 3], 3);
        let cell_runs = iterator.collect::<collections::HashSet<_>>();
        let expected = [
            vec![0, 1, 2],
            vec![3, 4, 5],
            vec![6, 7, 8],
            vec![0, 3, 6],
            vec![1, 4, 7],
            vec![2, 5, 8],
            vec![0, 4, 8],
            vec![2, 4, 6],
        ]
        .iter()
        .cloned()
        .collect::<collections::HashSet<_>>();
        assert_eq!(cell_runs, expected);
    }
}
