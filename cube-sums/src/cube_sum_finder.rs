pub trait CubeSumFinder {
    fn find_cube_sums(&self, range :usize) -> Vec<CubeSumComponent>;
}

#[derive(Copy, Clone)]
pub struct CubeSumComponent {
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub d: usize,
    pub sum: usize,
}

impl CubeSumComponent {
    pub fn new(a: usize, b: usize, c: usize, d: usize, sum: usize) -> Self {
        CubeSumComponent {
            a,
            b,
            c,
            d,
            sum,
        }
    }
}
