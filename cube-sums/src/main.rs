mod cube_sum_finder;
mod awful_cube_sum_finder;

use cube_sum_finder::CubeSumFinder;
use awful_cube_sum_finder::AwfulCubeSubFinder;

fn main() {
    let finder = AwfulCubeSubFinder {};
    for found in finder.find_cube_sums(100) {
        println!("{} + {} == {} + {} ... {}", found.a, found.b, found.c, found.d, found.sum);
    }
}
