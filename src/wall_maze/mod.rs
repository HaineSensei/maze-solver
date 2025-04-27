/// This module defines a maze structure with infinitely thin walls placed in a grid.
/// The maze is defined by its width and height, and the walls are represented by their
/// orientation (horizontal or vertical) and their position in the grid — the x and y
/// coordinates of the grid cell they are down or to the right of.
/// Positions in the maze are represented by their x and y coordinates in the grid with
/// x going down from 0 and y going right from 0.
/// The maze is defined by its start and end positions, which are also represented by
/// their x and y coordinates in the grid.
/// 
/// The maze is guaranteed to be solvable since to add a wall, the maze checks its
/// solvability and if it is not solvable, the wall addition fails and the wall is not
/// added.
use std::collections::HashMap;
#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Orientation {
    Horizontal,
    Vertical,
}
use Orientation::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Wall {
    x: usize,
    y: usize,
    orientation: Orientation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InteriorWall<const WIDTH: usize, const HEIGHT: usize> {
    wall: Wall
}

impl<const WIDTH: usize, const HEIGHT: usize> InteriorWall<WIDTH, HEIGHT> {
    pub fn new(x: usize, y: usize, orientation: Orientation) -> Result<Self, String> {
        if x >= WIDTH {
            Err(format!("x coordinate {} is out of bounds for width {}", x, WIDTH))
        } else if y >= HEIGHT {
            Err(format!("y coordinate {} is out of bounds for height {}", y, HEIGHT))
        } else {
            if x == WIDTH - 1 && orientation == Vertical {
                Err(format!("x coordinate {} is out of bounds for width {} (the wall would be on the edge of the maze)", x, WIDTH))
            } else if y == HEIGHT - 1 && orientation == Horizontal {
                Err(format!("y coordinate {} is out of bounds for height {} (the wall would be on the edge of the maze)", y, HEIGHT))
            } else {
                Ok(Self { wall: Wall { x, y, orientation } })
            }
        }
    }

    pub fn from_wall(wall: Wall) -> Result<Self, String> {
        Self::new(wall.x, wall.y, wall.orientation)
    }

    pub fn from_position_and_orientation(pos: InteriorPosition<WIDTH, HEIGHT>, orientation: Orientation) -> Result<Self, String> {
        match orientation {
            Horizontal => {
                if pos.y == HEIGHT - 1 {
                    Err(format!("y coordinate {} is out of bounds for height {} (the wall would be on the edge of the maze)", pos.y, HEIGHT))
                } else {
                    Self::new(pos.x, pos.y, orientation)
                }
            },
            Vertical => {
                if pos.x == WIDTH - 1 {
                    Err(format!("x coordinate {} is out of bounds for width {} (the wall would be on the edge of the maze)", pos.x, WIDTH))
                } else {
                    Self::new(pos.x, pos.y, orientation)
                }
            }
        }
    }

    pub fn get_x(self) -> usize {
        self.wall.x
    }

    pub fn get_y(self) -> usize {
        self.wall.y
    }

    pub fn get_orientation(self) -> Orientation {
        self.wall.orientation
    }

    pub fn get_wall(self) -> Wall {
        self.wall
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InteriorPosition<const WIDTH: usize, const HEIGHT: usize> {
    x: usize,
    y: usize,
}

impl<const WIDTH: usize, const HEIGHT: usize> InteriorPosition<WIDTH, HEIGHT> {
    pub fn new(x: usize, y: usize) -> Result<Self, String> {
        if x >= WIDTH {
            Err(format!("x coordinate {} is out of bounds for width {}", x, WIDTH))
        } else if y >= HEIGHT {
            Err(format!("y coordinate {} is out of bounds for height {}", y, HEIGHT))
        } else {
            Ok(Self { x, y })
        }
    }

    pub fn get_x(self) -> usize {
        self.x
    }

    pub fn get_y(self) -> usize {
        self.y
    }

    pub fn adjacent_positions(self) -> Vec<Self> {
        let mut positions = Vec::new();
        if self.x > 0 {
            positions.push(Self::new(self.x - 1, self.y).unwrap());
        }
        if self.x < WIDTH - 1 {
            positions.push(Self::new(self.x + 1, self.y).unwrap());
        }
        if self.y > 0 {
            positions.push(Self::new(self.x, self.y - 1).unwrap());
        }
        if self.y < HEIGHT - 1 {
            positions.push(Self::new(self.x, self.y + 1).unwrap());
        }
        positions
    }

    pub fn min_distance(self, other: Self) -> usize {
        let dx = match self.x.cmp(&other.x) {
            std::cmp::Ordering::Less => other.x - self.x,
            std::cmp::Ordering::Greater => self.x - other.x,
            std::cmp::Ordering::Equal => 0,
        };
        let dy = match self.y.cmp(&other.y) {
            std::cmp::Ordering::Less => other.y - self.y,
            std::cmp::Ordering::Greater => self.y - other.y,
            std::cmp::Ordering::Equal => 0,
        };
        dx + dy
    }

    pub fn adjacent_to(self, other: Self) -> bool {
        self.adjacent_positions().contains(&other)
    }

    pub fn separated_by_wall(self, other: Self, maze: &WallMaze<WIDTH,HEIGHT>) -> Result<bool, String> {
        if !self.adjacent_to(other) {
            Err(format!("Positions {:?} and {:?} are not adjacent", self, other))
        } else {
            match self.x.cmp(&other.x) {
                std::cmp::Ordering::Less => {
                    let proposed_wall = InteriorWall::from_position_and_orientation(self, Vertical)?;
                    if maze.walls.contains(&proposed_wall) {
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                },
                std::cmp::Ordering::Greater => {
                    let proposed_wall = InteriorWall::from_position_and_orientation(other, Vertical)?;
                    if maze.walls.contains(&proposed_wall) {
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                },
                std::cmp::Ordering::Equal => {
                    match self.y.cmp(&other.y) {
                        std::cmp::Ordering::Less => {
                            let proposed_wall = InteriorWall::from_position_and_orientation(self, Horizontal)?;
                            if maze.walls.contains(&proposed_wall) {
                                Ok(true)
                            } else {
                                Ok(false)
                            }
                        },
                        std::cmp::Ordering::Greater => {
                            let proposed_wall = InteriorWall::from_position_and_orientation(other, Horizontal)?;
                            if maze.walls.contains(&proposed_wall) {
                                Ok(true)
                            } else {
                                Ok(false)
                            }
                        },
                        std::cmp::Ordering::Equal => Err(format!("Positions {:?} and {:?} are the same", self, other)),
                    }
                },
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}
use Direction::{Up, Down, Left, Right};

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }

    pub const ALL: [Direction; 4] = [Up, Down, Left, Right];
}

impl<const WIDTH: usize, const HEIGHT: usize> InteriorPosition<WIDTH, HEIGHT> {
    pub fn move_in_direction(self, direction: Direction) -> Result<Self, String> {
        match direction {
            Up => {
                if self.y == 0 {
                    Err(format!("Cannot move up from position ({}, {})", self.x, self.y))
                } else {
                    Self::new(self.x, self.y - 1)
                }
            },
            Down => {
                match Self::new(self.x, self.y + 1) {
                    Ok(x) => Ok(x),
                    Err(_) => Err(format!("Cannot move down from position ({}, {}). This would leave the bottom of the maze of height {}", self.x, self.y, HEIGHT)),
                }
            },
            Left => {
                if self.x == 0 {
                    Err(format!("Cannot move left from position ({}, {})", self.x, self.y))
                } else {
                    Self::new(self.x - 1, self.y)
                }
            },
            Right => {
                match Self::new(self.x + 1, self.y) {
                    Ok(x) => Ok(x),
                    Err(_) => Err(format!("Cannot move right from position ({}, {}). This would leave the right side of the maze of width {}", self.x, self.y, WIDTH)),
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WallMaze<const WIDTH: usize, const HEIGHT: usize> {
    pub start: InteriorPosition<WIDTH, HEIGHT>,
    pub end: InteriorPosition<WIDTH, HEIGHT>,
    walls: Vec<InteriorWall<WIDTH, HEIGHT>>
}

impl<const WIDTH: usize, const HEIGHT: usize> WallMaze<WIDTH, HEIGHT> {
    pub fn new(start: InteriorPosition<WIDTH,HEIGHT>, end: InteriorPosition<WIDTH,HEIGHT>) -> Result<Self, String> {
        if start == end {
            return Err(format!("Start position cannot be the same as end position"));
        }
        Ok(Self {
            start,
            end,
            walls: Vec::new(),
        })
    }

    pub fn solve(&self) -> Result<Vec<InteriorPosition<WIDTH, HEIGHT>>, String> {
        let mut unchecked = vec![self.start];
        let mut path_to = HashMap::new();
        path_to.insert(self.start, Vec::from([self.start]));
        loop {
            if path_to.contains_key(&self.end) {
                break;
            }
            if unchecked.is_empty() {
                return Err(format!("No path found from start to end"));
            }
            let value = |pos: InteriorPosition<WIDTH, HEIGHT>| {
                pos.min_distance(self.end) + &path_to.get(&pos).unwrap().len()
            };
            unchecked.sort_by(|&a, &b| {
                value(a).cmp(&value(b)).reverse()
            });
            let next = unchecked.pop().unwrap();
            let adjacents = next.adjacent_positions();
            for adj in adjacents {
                if path_to.contains_key(&adj) {
                    continue;
                }
                if adj.separated_by_wall(next, self).unwrap() {
                    continue;
                }
                unchecked.push(adj);
                let mut new_path = path_to.get(&next).unwrap().clone();
                new_path.push(adj);
                path_to.insert(adj, new_path);
            }
        }
        Ok(path_to.get(&self.end).unwrap().clone())
    }

    fn solveable(&self) -> bool {
        self.solve().is_ok()
    }

    pub fn remove_wall(&mut self, interior_wall: InteriorWall<WIDTH, HEIGHT>) -> Result<(), String> {
        if let Some(pos) = self.walls.iter().position(|w| *w == interior_wall) {
            self.walls.remove(pos);
            Ok(())
        } else {
            Err(format!("Wall {:?} not found in the maze", interior_wall))
        }
    }

    pub fn add_wall(&mut self, wall: Wall) -> Result<(), String> {
        let interior_wall = InteriorWall::from_wall(wall)?;
        self.add_interior_wall(interior_wall)
    }

    pub fn add_interior_wall(&mut self, interior_wall: InteriorWall<WIDTH, HEIGHT>) -> Result<(), String> {
        if self.walls.contains(&interior_wall) {
            Err(format!("Wall {:?} already exists in the maze", interior_wall))
        } else {
            self.walls.push(interior_wall);
            if self.solveable() {
                Ok(())
            } else {
                self.walls.pop();
                Err(format!("Wall {:?} would make the maze unsolvable", interior_wall))
            }
        }
    }

    pub fn move_start(&mut self, new_start: InteriorPosition<WIDTH, HEIGHT>) -> Result<(), String> {
        let start_to_new_start_maze = match WallMaze::new(new_start, self.start) {
            Ok(maze) => maze,
            Err(_) => {
                return Ok(());
            },
        };
        if start_to_new_start_maze.solveable() {
            self.start = new_start;
            Ok(())
        } else {
            Err(format!("New start position {:?} would make the maze unsolvable", new_start))
        }
    }
}
