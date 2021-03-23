use crate::cube_sum_finder::{CubeSumFinder, CubeSumComponent};
use std::collections::HashMap;

pub struct ReasonableCubeSumFinder;

impl CubeSumFinder for ReasonableCubeSumFinder {
    fn find_cube_sums(&self, range :usize) -> Vec<CubeSumComponent> {
        let mut map = HashMap::new();

        for a in 1..(range + 1) {
            let a_cubed = a * a * a;
            for b in 1..(range + 1) {
                let b_cubed = b * b * b;
                let result = a_cubed + b_cubed;

                if let Some(list) = map.insert(result, vec![(a, b)]) {
                    let mut new_list = Vec::from(list);
                    new_list.push((a, b));
                    map.insert(result, new_list);
                }
            }
        }

        let mut found_cube_sums = Vec::new();
        for (result, list) in map {
            for pair1 in &list {
                for pair2 in &list {
                    let component = CubeSumComponent::new(pair1.0, pair1.1, pair2.0, pair2.1, result);
                    found_cube_sums.push(component);
                }
            }
        }
        found_cube_sums
    }
}