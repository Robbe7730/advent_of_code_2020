use crate::day::Day;

use itertools::iproduct;
use itertools::Itertools;

use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
pub struct Tile {
    id: usize,
    pixels: Vec<Vec<bool>>,
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let mut lines_iter = content.lines();
        let first = lines_iter.next().expect("Invalid input");
        Ok(Self::new(
            first
                .split(" ")
                .nth(1)
                .expect("Invalid input")
                .strip_suffix(":")
                .expect("Invalid input")
                .parse()
                .expect("Invalid input"),
            lines_iter
                .map(|x| x.bytes().map(|p| p == b'#').collect())
                .collect(),
        ))
    }
}

impl Tile {
    pub fn new(id: usize, pixels: Vec<Vec<bool>>) -> Self {
        Self { id, pixels }
    }

    pub fn edges(&self) -> HashSet<Vec<bool>> {
        let mut edges = HashSet::new();
        edges.insert(self.pixels[0].clone());
        edges.insert(self.pixels.last().expect("Invalid input").clone());
        edges.insert(self.pixels.iter().map(|x| x[0]).collect());
        edges.insert(
            self.pixels
                .iter()
                .map(|x| *x.last().expect("Invalid input"))
                .collect(),
        );
        edges.insert(self.pixels[0].iter().rev().cloned().collect());
        edges.insert(
            self.pixels
                .last()
                .expect("Invalid input")
                .iter()
                .rev()
                .cloned()
                .collect(),
        );
        edges.insert(self.pixels.iter().map(|x| x[0]).rev().collect());
        edges.insert(
            self.pixels
                .iter()
                .map(|x| *x.last().expect("Invalid input"))
                .rev()
                .collect(),
        );
        edges
    }

    pub fn fits(&self, other: &Tile) -> usize {
        self.edges().intersection(&other.edges()).count()
    }

    pub fn all_stripped(&self) -> HashSet<StrippedTile> {
        let mut ret = HashSet::new();

        let mut pixels;
        for i in 0..4 {
            pixels = self.pixels.clone();
            if i & 2 != 0 {
                pixels = Self::flip_horizontal(&pixels);
            }

            if i & 1 != 0 {
                pixels = Self::flip_vertical(&pixels);
            }
            let mut curr_top = Self::to_num(&pixels[0]);
            let mut curr_right = Self::to_num(&pixels.iter().map(|x| *x.last().unwrap()).collect());
            let mut curr_bottom =
                Self::to_num(&pixels.last().unwrap().iter().rev().cloned().collect());
            let mut curr_left = Self::to_num(&pixels.iter().rev().map(|x| x[0]).collect());

            for _ in 0..4 {
                ret.insert(StrippedTile {
                    id: self.id,
                    pixels: pixels.clone(),
                    top: curr_top,
                    right: curr_right,
                    bottom: curr_bottom,
                    left: curr_left,
                });
                pixels = Self::rotate(&pixels);
                let temp = curr_top;
                curr_top = curr_left;
                curr_left = curr_bottom;
                curr_bottom = curr_right;
                curr_right = temp;
            }
        }

        ret
    }

    pub fn to_num(list: &Vec<bool>) -> usize {
        list.iter().fold(0, |a, x| (a << 1) | (*x as usize))
    }

    pub fn rotate(pixels: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
        let num_rows = pixels.len();
        let num_cols = pixels[0].len();
        let mut ret: Vec<Vec<bool>> = vec![vec![false; num_rows]; num_cols];
        for (i, row) in pixels.iter().enumerate() {
            for (j, pixel) in row.iter().enumerate() {
                ret[j][num_rows - i - 1] = *pixel;
            }
        }
        ret
    }

    pub fn flip_horizontal(pixels: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
        let num_rows = pixels.len();
        let num_cols = pixels[0].len();
        let mut ret: Vec<Vec<bool>> = vec![vec![false; num_cols]; num_rows];
        for (i, row) in pixels.iter().enumerate() {
            ret[num_rows - i - 1] = row.clone();
        }
        ret
    }

    pub fn flip_vertical(pixels: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
        let num_rows = pixels.len();
        let num_cols = pixels[0].len();
        let mut ret: Vec<Vec<bool>> = vec![vec![false; num_cols]; num_rows];
        for (i, row) in pixels.iter().enumerate() {
            for (j, pixel) in row.iter().enumerate() {
                ret[i][num_cols - j - 1] = *pixel;
            }
        }
        ret
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct StrippedTile {
    id: usize,
    pixels: Vec<Vec<bool>>,
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
}

impl StrippedTile {
    #[allow(dead_code)]
    pub fn display(&self) {
        for line in self.pixels.iter() {
            println!(
                "{}",
                line.iter().map(|x| if *x { "#" } else { "." }).join("")
            );
        }
    }

    pub fn fits_right_of(&self, other: &StrippedTile) -> bool {
        self.left == Self::reverse(other.right) && self.id != other.id
    }

    pub fn fits_below(&self, other: &StrippedTile) -> bool {
        self.top == Self::reverse(other.bottom) && self.id != other.id
    }

    pub fn reverse(num: usize) -> usize {
        let mut mut_num = num;
        let mut ret = 0;
        for _ in 0..10 {
            ret = (ret << 1) | (mut_num & 1);
            mut_num >>= 1;
        }
        ret
    }

    pub fn stripped_rows(&self) -> Vec<Vec<bool>> {
        self.pixels
            .iter()
            .skip(1)
            .map(|x| x.iter().skip(1).take(8).cloned().collect())
            .take(8)
            .collect()
    }
}

pub struct Day20 {}

impl Day for Day20 {
    type InputElement = Tile;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Vec<Self::InputElement>) -> Self::Output1 {
        let mut ret = 1;
        for first in input {
            let mut num = 0;
            for second in input {
                let count = first.fits(second);
                if count > 0 {
                    num += 1;
                }
            }
            if num == 3 {
                ret *= first.id;
            }
        }
        ret
    }

    fn solve_part2(&self, input: &Vec<Self::InputElement>) -> Self::Output2 {
        // Find all options of tile rotations/flips
        let entire_pool: Vec<StrippedTile> =
            input.iter().map(|x| x.all_stripped()).flatten().collect();

        // Count how often each side occurs
        let mut edges_count: HashMap<usize, usize> = HashMap::new();
        for edge in entire_pool.iter().map(|x| x.top) {
            if !edges_count.contains_key(&edge) {
                edges_count.insert(edge, 1);
            } else {
                let value = edges_count.get(&edge).unwrap().clone();
                edges_count.insert(edge, value + 1);
            }
        }

        // Find a possible top-left tile
        let mut left_tile_opt = entire_pool
            .iter()
            .filter(|x| edges_count[&x.top] == 1 && edges_count[&x.left] == 1)
            .next();

        // Construct the grid
        let mut grid: Vec<Vec<bool>> = vec![];
        let mut starting_row = 0;
        while left_tile_opt.is_some() {
            // Add room for the tiles
            for _ in 0..8 {
                grid.push(vec![]);
            }

            let left_tile = left_tile_opt.unwrap();
            let mut curr_tile_opt = Some(left_tile);

            // Find the tiles in the row
            while curr_tile_opt.is_some() {
                let curr_tile = curr_tile_opt.unwrap();

                for (i, row) in curr_tile.stripped_rows().iter().enumerate() {
                    grid[starting_row + i].append(&mut row.clone());
                }

                curr_tile_opt = entire_pool
                    .iter()
                    .filter(|x| x.fits_right_of(curr_tile))
                    .next();
            }
            left_tile_opt = entire_pool
                .iter()
                .filter(|x| x.fits_below(left_tile))
                .next();
            starting_row += 8;
        }

        // Find the monsters
        //  01234567890123456789
        // 0                  #
        // 1#    ##    ##    ###
        // 2 #  #  #  #  #  #
        let monster_orig = vec![
            vec![
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, true, false,
            ],
            vec![
                true, false, false, false, false, true, true, false, false, false, false, true,
                true, false, false, false, false, true, true, true,
            ],
            vec![
                false, true, false, false, true, false, false, true, false, false, true, false,
                false, true, false, false, true, false, false, false,
            ],
        ];

        let mut monster_positions: HashSet<(usize, usize)> = HashSet::new();

        for i in 0..4 {
            let mut monster = monster_orig.clone();
            if i & 2 != 0 {
                monster = Tile::flip_horizontal(&monster);
            }

            if i & 1 != 0 {
                monster = Tile::flip_vertical(&monster);
            }
            for _ in 0..4 {
                for start_x in 0..=(grid.len() - monster.len()) {
                    for start_y in 0..=(grid[0].len() - monster[0].len()) {
                        if iproduct!(0..monster.len(), 0..monster[0].len())
                            .all(|(dx, dy)| !monster[dx][dy] || grid[start_x + dx][start_y + dy])
                        {
                            monster_positions.insert((start_x, start_y));
                        }
                    }
                }
                monster = Tile::rotate(&monster);
            }
        }

        // 2242 -> too high
        // 2227 -> too high
        // 2212 -> too high
        // 2197 -> not right
        // 2182 -> not right
        grid.iter().flatten().filter(|x| **x).count() - (monster_positions.len() * 15)
    }

    fn parse_input(&self, content: String) -> Vec<Self::InputElement> {
        content
            .split("\n\n")
            .map(|x| x.parse().expect("Invalid input"))
            .collect()
    }
}
