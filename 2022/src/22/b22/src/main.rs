/**
 * Based on /u/Kentzo's solution: https://www.reddit.com/r/adventofcode/comments/zsct8w/2022_day_22_solutions/j25n56r/
 * Python code: https://gist.githubusercontent.com/Kentzo/2319c054832f2ff2befc7cfacf914976/raw/24c62150f20a7940d121c0e9a3f78513f5c5a1e6/aoc22.py
 */
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

mod matrix;
use matrix::*;

// Mapping between cardinal directions along the input.
const EAST: Vec3 = [1, 0, 0];
const SOUTH: Vec3 = [0, -1, 0];
const WEST: Vec3 = [-1, 0, 0];
const NORTH: Vec3 = [0, 1, 0];

/**
 * Cube faces.
 */
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct CubeRotation {
    canonical: Rot3,
    z_rotation: Rot3,
}

impl Default for CubeRotation {
    fn default() -> Self {
        Self {
            canonical: IDENTITY,
            z_rotation: IDENTITY,
        }
    }
}

impl CubeRotation {
    fn rotation(&self) -> Rot3 {
        matmul(self.z_rotation, self.canonical)
    }
}

/**
 * Coordinates to a location in the flattened input text.
 */
type InputCoord = (usize, usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
    Forward(i64),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Facing {
    East,
    South,
    West,
    North,
}

impl Facing {
    fn score(&self) -> usize {
        match self {
            Facing::East => 0,
            Facing::South => 1,
            Facing::West => 2,
            Facing::North => 3,
        }
    }

    fn left(&self) -> Self {
        match self {
            Facing::East => Facing::North,
            Facing::South => Facing::East,
            Facing::West => Facing::South,
            Facing::North => Facing::West,
        }
    }

    fn right(&self) -> Self {
        match self {
            Facing::North => Facing::East,
            Facing::East => Facing::South,
            Facing::South => Facing::West,
            Facing::West => Facing::North,
        }
    }
}

impl Into<u8> for Facing {
    fn into(self) -> u8 {
        match self {
            Facing::East => b'>',
            Facing::South => b'v',
            Facing::West => b'<',
            Facing::North => b'^',
        }
    }
}

impl From<Vec3> for Facing {
    fn from(v: Vec3) -> Self {
        match v {
            EAST => Facing::East,
            SOUTH => Facing::South,
            WEST => Facing::West,
            NORTH => Facing::North,
            _ => panic!("Unexpected Vec3 {:?}", v),
        }
    }
}

impl Into<Vec3> for Facing {
    fn into(self) -> Vec3 {
        match self {
            Facing::East => EAST,
            Facing::South => SOUTH,
            Facing::West => WEST,
            Facing::North => NORTH,
        }
    }
}

#[test]
fn test_cardinal_z_rotations() {
    assert_eq!(vecmul(Z_90, EAST), NORTH);
    assert_eq!(vecmul(Z_90, NORTH), WEST);
    assert_eq!(vecmul(Z_90, WEST), SOUTH);
    assert_eq!(vecmul(Z_90, SOUTH), EAST);

    assert_eq!(vecmul(Z_270, EAST), SOUTH);
    assert_eq!(vecmul(Z_270, SOUTH), WEST);
    assert_eq!(vecmul(Z_270, WEST), NORTH);
    assert_eq!(vecmul(Z_270, NORTH), EAST);
}

struct Input {
    rows: Vec<Vec<u8>>,
    directions: Vec<Direction>,
}

impl Input {
    fn from(lines: Vec<Vec<u8>>) -> Self {
        let mut making_grid = true;
        let mut rows = vec![];
        let mut directions = vec![];
        for line in lines {
            if line.is_empty() {
                making_grid = false;
            } else if making_grid {
                let mut grid_row = vec![];
                for c in line {
                    match c {
                        b' ' | b'#' | b'.' => {
                            grid_row.push(c);
                        }
                        _ => panic!("Unexpected character: {}", c),
                    }
                }
                rows.push(grid_row);
            } else {
                let mut steps = 0;
                for c in line {
                    match c {
                        b'L' => {
                            if steps != 0 {
                                directions.push(Direction::Forward(steps));
                                steps = 0;
                            }
                            directions.push(Direction::Left);
                        }
                        b'R' => {
                            if steps != 0 {
                                directions.push(Direction::Forward(steps));
                                steps = 0;
                            }
                            directions.push(Direction::Right);
                        }
                        b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                            steps *= 10;
                            steps += (c - b'0') as i64;
                        }
                        _ => panic!("Unexpected direction character: {}", c),
                    }
                }
                if steps != 0 {
                    directions.push(Direction::Forward(steps));
                    steps = 0;
                }
            }
        }
        Self { rows, directions }
    }

    /**
     * Return a node with the given coordinates, if it exists.
     */
    fn get(&self, coords: InputCoord) -> Option<u8> {
        let (x, y) = coords;
        if y >= self.rows.len() {
            return None;
        }
        if x >= self.rows[y].len() {
            return None;
        }
        if self.rows[y][x] == b' ' {
            return None;
        }
        Some(self.rows[y][x])
    }

    /**
     * Top-left nonempty node. Where the player begins.
     */
    fn starting_point(&self) -> (usize, usize) {
        let x = self.rows[0]
            .iter()
            .enumerate()
            .find(|(_, c)| **c != b' ')
            .unwrap()
            .0;
        let y = 0;
        (x, y)
    }

    fn side_len(&self) -> usize {
        // Every flat cube map on a grid will have either four rows or four columns
        self.rows
            .len()
            .max(self.rows.iter().map(|row| row.len()).max().unwrap_or(0))
            / 4
    }

    /**
     * Example input given in the puzzle
     */
    fn example() -> Self {
        let lines: Vec<&[u8]> = vec![
            b"        ...#",
            b"        .#..",
            b"        #...",
            b"        ....",
            b"...#.......#",
            b"........#...",
            b"..#....#....",
            b"..........#.",
            b"        ...#....",
            b"        .....#..",
            b"        .#......",
            b"        ......#.",
            b"",
            b"10R5L5R10L4R5L5",
        ];
        Self::from(
            lines
                .into_iter()
                .map(|byte_str| byte_str.to_vec())
                .collect(),
        )
    }
}

/**
 * Represents the relationship between a cube's face and rotation, and the InputCoord it corresponds to
 */
#[derive(Clone, Copy, Debug)]
struct CubeProjection {
    rotation: CubeRotation,
    min_coords: InputCoord,
}

impl CubeProjection {
    fn initial(min_coords: InputCoord) -> Self {
        Self {
            rotation: CubeRotation::default(),
            min_coords,
        }
    }
}

/**
 * There are 24 ways to view a cube (six faces * four rotations per face).
 *
 * For each of the six sides, one of its four Z-rotations is "canonical", meaning it corresponds to the view of the cube that's actually on the flattened map.
 */
fn make_rotation_to_canonical(input: &Input) -> HashMap<Rot3, CubeProjection> {
    let mut rotation_to_canonical = HashMap::new();
    let side_len = input.side_len();

    let mut seen = HashSet::new();
    let mut nodes = vec![(IDENTITY, input.starting_point())];
    while let Some((canonical_rotation, (x, y))) = nodes.pop() {
        // Make sure we are on the map
        assert!(input.get((x, y)).is_some());

        // Skip if we've already visited these coordinates, otherwise mark as visited
        if !seen.insert((x, y)) {
            continue;
        }

        // Record each given rotation, its Z-rotation, and its upper-left coordinates on the flattened input map
        for z_rotation in [IDENTITY, Z_90, Z_180, Z_270] {
            rotation_to_canonical.insert(
                matmul(z_rotation, canonical_rotation),
                CubeProjection {
                    rotation: CubeRotation {
                        canonical: canonical_rotation,
                        z_rotation,
                    },
                    min_coords: (x, y),
                },
            );
        }

        // East
        if input.get((x.wrapping_add(side_len), y)).is_some() {
            nodes.push((matmul(Y_270, canonical_rotation), (x + side_len, y)));
        }
        // South
        if input.get((x, y.wrapping_add(side_len))).is_some() {
            nodes.push((matmul(X_270, canonical_rotation), (x, y + side_len)));
        }
        // West
        if input.get((x.wrapping_sub(side_len), y)).is_some() {
            nodes.push((matmul(Y_90, canonical_rotation), (x - side_len, y)));
        }
        // North
        if input.get((x, y.wrapping_sub(side_len))).is_some() {
            nodes.push((matmul(X_90, canonical_rotation), (x, y - side_len)));
        }
    }

    assert_eq!(rotation_to_canonical.len(), 24);
    rotation_to_canonical
}

struct Board {
    input: Input,
    rotation_to_canonical: HashMap<Rot3, CubeProjection>,
    current_projection: CubeProjection,
    current_coords: InputCoord,
    facing: Facing,
    t: usize,
}

impl Board {
    fn new(input: Input) -> Self {
        let rotation_to_canonical = make_rotation_to_canonical(&input);
        let current_coords = input.starting_point();
        let current_projection = CubeProjection::initial(current_coords);
        let facing = Facing::East;
        let t = 0;
        let mut me = Self {
            input,
            rotation_to_canonical,
            current_projection,
            current_coords,
            facing,
            t,
        };
        me.mark();
        me
    }

    fn run(&mut self) {
        while self.t < self.input.directions.len() {
            self.step();
        }
    }

    fn step(&mut self) {
        match self.input.directions[self.t] {
            Direction::Left => {
                self.facing = self.facing.left();
                self.mark();
            }
            Direction::Right => {
                self.facing = self.facing.right();
                self.mark();
            }
            Direction::Forward(n) => {
                for _ in 0..n {
                    if !self.forward() {
                        break;
                    }
                    self.mark();
                }
            }
        }
        self.t += 1;
        // println!("{}", self);
    }

    fn forward(&mut self) -> bool {
        let new_rel_coords = self.forward_relative_coords();
        let (new_projection, new_coords, new_facing) =
            self.wrap_coordinate(new_rel_coords, self.facing);
        let new_tile = self.input.get(new_coords).unwrap();
        assert_ne!(new_tile, b' ');
        if new_tile != b'#' {
            self.current_projection = new_projection;
            // We don't care about our z-rotation anymore, and leaving it on would cause us to reapply the rotation with every timestep.
            // TODO: It feels hacky to clear it this way. I'm p sure Z-rotations are only really useful during canonicalize_coordinates.
            self.current_projection.rotation.z_rotation = IDENTITY;
            self.current_coords = new_coords;
            self.facing = new_facing;
            true
        } else {
            false
        }
    }

    /**
     * Return signed coordinates, relative to the current face of the cube we're on, which may have gone off the edge of the cube.
     */
    fn forward_relative_coords(&self) -> (i64, i64) {
        let x = (self.current_coords.0 - self.current_projection.min_coords.0) as i64;
        let y = (self.current_coords.1 - self.current_projection.min_coords.1) as i64;
        match self.facing {
            Facing::North => (x, y - 1),
            Facing::South => (x, y + 1),
            Facing::East => (x + 1, y),
            Facing::West => (x - 1, y),
        }
    }

    fn mark(&mut self) {
        let (x, y) = self.current_coords;
        let row = &mut self.input.rows[y];
        assert_ne!(row[x], b' ');
        assert_ne!(row[x], b'#');
        row[x] = self.facing.into();
    }

    /**
     * Transform coordinates, which may not represent legal locations on the input, into coordinates which are definitely legal locations on the input.
     */
    fn wrap_coordinate(
        &self,
        mut relative_coords: (i64, i64),
        facing: Facing,
    ) -> (CubeProjection, InputCoord, Facing) {
        let src = &self.current_projection;
        let side_len = self.input.side_len() as i64;
        let mut dst_rot = src.rotation.rotation();
        loop {
            let (rel_x, rel_y) = relative_coords;
            if rel_x < 0 {
                relative_coords = (rel_x + side_len, rel_y);
                dst_rot = matmul(Y_90, dst_rot);
            } else if rel_y < 0 {
                relative_coords = (rel_x, rel_y + side_len);
                dst_rot = matmul(X_90, dst_rot);
            } else if rel_x >= side_len {
                relative_coords = (rel_x - side_len, rel_y);
                dst_rot = matmul(Y_270, dst_rot);
            } else if rel_y >= side_len {
                relative_coords = (rel_x, rel_y - side_len);
                dst_rot = matmul(X_270, dst_rot);
            } else {
                break;
            }
        }
        assert!(relative_coords.0 >= 0);
        assert!(relative_coords.0 < side_len);
        assert!(relative_coords.1 >= 0);
        assert!(relative_coords.1 < side_len);
        // We know which face of the cube we're on now, but we don't yet know where we are on the flattened input map.
        // Use rotation_to_canonical to find where we are on the flattened input map.
        let dst = self.rotation_to_canonical.get(&dst_rot).unwrap();
        let (new_coords, new_facing) = self.canonicalize_coords(dst, relative_coords, facing);
        (*dst, new_coords, new_facing)
    }

    /**
     * Given coordinates and a Facing relative to the given face and rotation, return a real InputCoord that corresponds to the face's location on the unfolded input.
     */
    fn canonicalize_coords(
        &self,
        projection: &CubeProjection,
        relative_coords: (i64, i64),
        facing: Facing,
    ) -> (InputCoord, Facing) {
        let side_len = self.input.side_len() as i64;
        assert!(relative_coords.0 >= 0);
        assert!(relative_coords.0 < side_len);
        assert!(relative_coords.1 >= 0);
        assert!(relative_coords.1 < side_len);

        // We need to rotate the coordinates around the center of the current face, so convert them to centered coordinates
        let centered_coords_vec = [
            2 * relative_coords.0 - side_len + 1,
            2 * relative_coords.1 - side_len + 1,
            0,
        ];

        // Take this projection's Z-rotation and undo it, to convert to the canonical rotation we saw in the unfolded input
        let inv_z_rotation = transpose(projection.rotation.z_rotation);
        let transformed_centered_coords_vec =
            vecmul(projection.rotation.z_rotation, centered_coords_vec);
        assert!(transformed_centered_coords_vec[0] >= -side_len);
        assert!(transformed_centered_coords_vec[0] <= side_len);
        assert!(transformed_centered_coords_vec[1] >= -side_len);
        assert!(transformed_centered_coords_vec[1] <= side_len);
        let transformed_relative_coords = (
            (transformed_centered_coords_vec[0] + side_len - 1) / 2,
            (transformed_centered_coords_vec[1] + side_len - 1) / 2,
        );
        assert!(transformed_relative_coords.0 >= 0);
        assert!(transformed_relative_coords.0 < side_len);
        assert!(transformed_relative_coords.1 >= 0);
        assert!(transformed_relative_coords.1 < side_len);
        let transformed_coords = (
            transformed_relative_coords.0 as usize + projection.min_coords.0,
            transformed_relative_coords.1 as usize + projection.min_coords.1,
        );
        // Also undo the given Facing's Z-rotation
        let transformed_facing = Facing::from(vecmul(inv_z_rotation, facing.into()));
        (transformed_coords, transformed_facing)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.input.rows {
            write!(f, "{}\n", std::str::from_utf8(row.as_slice()).unwrap())?;
        }
        write!(f, "\n{:?}\n", self.input.directions)?;
        Ok(())
    }
}

fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(|line| line.unwrap().as_bytes().to_vec())
        .collect();
    let mut board = Board::new(Input::from(lines));
    // let mut board = Board::new(Input::example());
    println!("{}", board);
    board.run();
    println!("{}", board);
    let (x, y) = board.current_coords;
    println!(
        "Row: {}, Col: {}, facing: {} ({:?})",
        y + 1,
        x + 1,
        board.facing.score(),
        board.facing,
    );
    println!(
        "Score: {}",
        1000 * (y + 1) + 4 * (x + 1) + board.facing.score()
    );
}

#[test]
fn test_example_start() {
    /* Structure of the example input:
     *   A
     * BCD
     *   EF
     */
    let ex = Input::example();
    let side_len = ex.side_len();
    assert_eq!(side_len, 4);
    let (x, y) = ex.starting_point();
    assert_eq!(x, side_len * 2);
    assert_eq!(y, 0);
}

#[test]
fn test_example() {
    /* Structure of the example input:
     *   A
     * BCD
     *   EF
     */
    let mut board = Board::new(Input::example());
    let (x, y) = board.current_coords;
    let side_len = board.input.side_len();
    assert_eq!(x, side_len * 2);
    assert_eq!(y, 0);
    board.input.directions = vec![
        Direction::Forward(2),
        Direction::Right,
        Direction::Forward(5),
        Direction::Left,
        Direction::Forward(3),
    ];
    // Take two steps East
    board.step();
    assert_eq!(board.facing, Facing::East);
    assert_eq!(board.current_coords, (side_len * 2 + 2, 0));
    // Turn Right to go South
    board.step();
    assert_eq!(board.facing, Facing::South);
    // Now take five steps south, off the south edge of A. This puts you on the north edge of D, facing south.
    board.step();
    assert_eq!(board.current_projection.rotation.rotation(), X_270);
    let (x, y) = board.current_coords;
    assert_eq!(x, side_len * 2 + 2);
    assert_eq!(y, 5);
    assert_eq!(board.facing, Facing::South);
    // Now make a left turn. You are now facing East.
    board.step();
    assert_eq!(board.facing, Facing::East);
    // Now walk three steps off the east edge of D. You are now on F, facing South.
    board.step();
    assert_eq!(
        board.current_projection.rotation.rotation(),
        matmul(Y_270, X_180)
    );
    let (x, y) = board.current_coords;
    assert_eq!(x, side_len * 3 + 2);
    assert_eq!(y, side_len * 2 + 1);
    assert_eq!(vecmul(Z_270, EAST), SOUTH);
    assert_eq!(board.facing, Facing::South);
}

#[test]
fn test_wrap_coordinate() {
    /*   A
     * BCD
     *   EF
     */
    let mut board = Board::new(Input::example());
    let (x, y) = board.current_coords;
    let side_len = board.input.side_len();
    assert_eq!(x, side_len * 2);
    board.input.directions = vec![
        Direction::Forward(1),
        Direction::Left,
        Direction::Forward(1),
    ];
    // Take one step East
    board.step();
    assert_eq!(board.facing, Facing::East);
    assert_eq!(board.current_coords, (side_len * 2 + 1, 0));
    // Turn Left to go North
    board.step();
    assert_eq!(board.facing, Facing::North);
    // Now take one step north off north edge of A. This puts you on the north edge of B, facing south.
    board.step();
    assert_eq!(
        board.current_projection.rotation.rotation(),
        matmul(Y_180, X_270)
    );
    let (x, y) = board.current_coords;
    assert_eq!(x, side_len - 2);
    assert_eq!(y, side_len);
    assert_eq!(board.facing, Facing::South);
}
