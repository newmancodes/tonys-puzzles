use crate::cube_sum_finder::{CubeSumFinder, CubeSumComponent};
use num_integer::Roots;

pub struct LessAwfulCubeSumFinder;

impl CubeSumFinder for LessAwfulCubeSumFinder {
    fn find_cube_sums(&self, range :usize) -> Vec<CubeSumComponent> {
        let mut found_cube_sums = Vec::new();

        for a in 1..(range + 1) {
            let a_cubed = a * a * a;
            for b in 1..(range + 1) {
                let b_cubed = b * b * b;
                for c in 1..(range + 1) {
                    let c_cubed = c * c * c;

                    let lhs = a_cubed + b_cubed;
                    let potential_d = lhs - c_cubed;

                    let d = potential_d.nth_root(3);
                    let rhs = c_cubed + (d * d * d);
                    if lhs == rhs && d >= 1 && d <= range {
                        let component = CubeSumComponent::new(a, b, c, d, lhs);
                        found_cube_sums.push(component);
                        break;
                    }
                }
            }
        }

        found_cube_sums
    }
}