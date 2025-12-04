pub mod template;

pub type Coord = (usize, usize);

pub fn ortho_neighbors(
    (x, y): (usize, usize),
    w: usize,
    h: usize,
) -> impl Iterator<Item = (usize, usize)> {
    const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    DIRS.into_iter().filter_map(move |(dx, dy)| {
        let nx = x.checked_add_signed(dx)?;
        let ny = y.checked_add_signed(dy)?;
        (nx < w && ny < h).then_some((nx, ny))
    })
}

pub fn all_neighbors(
    (x, y): (usize, usize),
    w: usize,
    h: usize,
) -> impl Iterator<Item = (usize, usize)> {
    const DIRS: [(isize, isize); 8] = [
        (-1, 1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (0, 1),
        (1, 1),
    ];

    DIRS.into_iter().filter_map(move |(dx, dy)| {
        let nx = x.checked_add_signed(dx)?;
        let ny = y.checked_add_signed(dy)?;
        (nx < w && ny < h).then_some((nx, ny))
    })
}
