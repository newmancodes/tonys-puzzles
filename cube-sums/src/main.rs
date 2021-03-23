mod cube_sum_finder;
mod awful_cube_sum_finder;
mod slightly_less_awful_cube_sum_finder;
mod less_awful_cube_sum_finder;
mod reasonable_cube_sum_finder;

use cube_sum_finder::CubeSumFinder;
use awful_cube_sum_finder::AwfulCubeSumFinder;
use slightly_less_awful_cube_sum_finder::SlightlyLessAwfulCubeSumFinder;
use less_awful_cube_sum_finder::LessAwfulCubeSumFinder;
use reasonable_cube_sum_finder::ReasonableCubeSumFinder;

extern crate num_integer;

use stopwatch::Stopwatch;

fn main() {
    let finder = ReasonableCubeSumFinder {};
    let mut sw = Stopwatch::start_new();
    let found_cube_sums = finder.find_cube_sums(1000);
    let elapsed = sw.elapsed();
    sw.stop();
    for found in found_cube_sums {
        println!("{} + {} == {} + {} ... {}", found.a, found.b, found.c, found.d, found.sum);
    }
    println!("");
    println!("Took {}ms.", elapsed.as_millis());
}
