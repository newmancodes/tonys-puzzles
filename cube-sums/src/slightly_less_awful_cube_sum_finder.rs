use crate::cube_sum_finder::{CubeSumFinder, CubeSumComponent};

pub struct SlightlyLessAwfulCubeSumFinder;

impl CubeSumFinder for SlightlyLessAwfulCubeSumFinder {
    fn find_cube_sums(&self, range :usize) -> Vec<CubeSumComponent> {
        let mut found_cube_sums = Vec::new();

        for a in 1..(range + 1) {
            let a_cubed = a * a * a;
            for b in 1..(range + 1) {
                let b_cubed = b * b * b;
                for c in 1..(range + 1) {
                    let c_cubed = c * c * c;
                    for d in 1..(range + 1) {
                        let d_cubed = d * d * d;

                        let lhs = a_cubed + b_cubed;
                        let rhs = c_cubed + d_cubed;
                        if lhs == rhs {
                            let component = CubeSumComponent::new(a, b, c, d, lhs);
                            found_cube_sums.push(component);
                            break;
                        }
                    }
                }
            }
        }

        found_cube_sums
    }
}