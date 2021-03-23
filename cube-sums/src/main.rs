mod cube_sum_finder;
mod awful_cube_sum_finder;
mod slightly_less_awful_cube_sum_finder;
mod less_awful_cube_sum_finder;

use cube_sum_finder::CubeSumFinder;
use awful_cube_sum_finder::AwfulCubeSubFinder;
use slightly_less_awful_cube_sum_finder::SlightlyLessAwfulCubeSubFinder;
use less_awful_cube_sum_finder::LessAwfulCubeSubFinder;

extern crate num_integer;

use stopwatch::Stopwatch;

fn main() {
    let finder = LessAwfulCubeSubFinder {};
    let mut sw = Stopwatch::start_new();
    let found_cube_sums = finder.find_cube_sums(100);
    let elapsed = sw.elapsed();
    sw.stop();
    for found in found_cube_sums {
        println!("{} + {} == {} + {} ... {}", found.a, found.b, found.c, found.d, found.sum);
    }
    println!("");
    println!("Took {}ms.", elapsed.as_millis());
}
