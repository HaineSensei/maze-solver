/// This module implements a grid-based maze with infinitely thin walls.
///
/// # Coordinate System
///
/// The maze is structured on a grid of dimensions WIDTH × HEIGHT.
/// Positions are represented by (x, y) coordinates where:
/// - x increases downward from 0 (to WIDTH-1)
/// - y increases rightward from 0 (to HEIGHT-1)
///
/// # Wall Representation
///
/// Walls are placed between adjacent cells and are defined by:
/// - An (x, y) coordinate of a cell
/// - An orientation (horizontal or vertical)
///
/// A vertical wall at position (x, y) lies between cell (x, y) and (x+1, y),
/// positioned to the right of the cell at (x, y).
///
/// A horizontal wall at position (x, y) lies between cell (x, y) and (x, y+1),
/// positioned below the cell at (x, y).
///
/// # Solvability Guarantee
///
/// The maze maintains a guarantee of solvability at all times:
/// - When adding a wall, the module verifies the maze remains solvable
/// - If a wall would make the maze unsolvable, the addition is automatically rejected
///
/// The maze provides functionality to find paths from start to end,
/// determine if positions are separated by walls, and move between adjacent positions.


use std::collections::HashMap;
#[cfg(test)]
mod tests;

/// Represents the orientation of a wall in the maze.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Orientation {
    Horizontal,
    Vertical,
}
use Orientation::*;

/// Represents a wall at a specific position and orientation in a maze.
///
/// A wall is defined by its (x, y) position and orientation (horizontal or vertical).
/// The position coordinates refer to the cell that the wall is adjacent to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Wall {
    x: usize,
    y: usize,
    orientation: Orientation,
}

impl Wall {
    /// Creates a new `Wall` with the specified coordinates and orientation.
    pub fn new(x: usize, y: usize, orientation: Orientation) -> Self {
        Self { x, y, orientation }
    }

    /// Returns the x-coordinate of this wall.
    pub fn get_x(self) -> usize {
        self.x
    }

    /// Returns the y-coordinate of this wall.
    pub fn get_y(self) -> usize {
        self.y
    }

    /// Returns the orientation of this wall.
    pub fn get_orientation(self) -> Orientation {
        self.orientation
    }
}

/// A wall that is guaranteed to be within the interior bounds of a maze of dimensions WIDTH × HEIGHT.
///
/// This struct wraps a `Wall` and ensures it is valid for the maze dimensions,
/// preventing walls that would lie on the exterior boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InteriorWall<const WIDTH: usize, const HEIGHT: usize> {
    wall: Wall
}

impl<const WIDTH: usize, const HEIGHT: usize> InteriorWall<WIDTH, HEIGHT> {

    /// Creates a new `InteriorWall<WIDTH, HEIGHT>` at the specified position and orientation.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - x >= WIDTH or y >= HEIGHT
    /// - x = WIDTH - 1 and orientation is Vertical
    /// - y = HEIGHT - 1 and orientation is Horizontal
    ///
    /// # Examples
    ///
    /// ```
    /// use maze_solver::wall_maze::{InteriorWall, Orientation};
    ///
    /// // Valid wall
    /// let wall = InteriorWall::<5, 5>::new(2, 3, Orientation::Horizontal);
    /// assert!(wall.is_ok());
    ///
    /// // Invalid wall (would be on the exterior boundary)
    /// let boundary_wall = InteriorWall::<5, 5>::new(4, 2, Orientation::Vertical);
    /// assert!(boundary_wall.is_err());
    /// ```
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

    /// Creates an `InteriorWall<WIDTH, HEIGHT>` from an existing `Wall` structure.
    ///
    /// # Errors
    ///
    /// Returns an error if the wall would be invalid for this maze's dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use maze_solver::wall_maze::{InteriorWall, Wall, Orientation};
    ///
    /// let raw_wall = Wall::new(2, 3, Orientation::Horizontal);
    /// let interior_wall = InteriorWall::<5, 5>::from_wall(raw_wall);
    /// assert!(interior_wall.is_ok());
    /// ```
    pub fn from_wall(wall: Wall) -> Result<Self, String> {
        Self::new(wall.x, wall.y, wall.orientation)
    }

    /// Creates an `InteriorWall<WIDTH, HEIGHT>` at the given position with the specified orientation.
    ///
    /// # Errors
    ///
    /// Returns an error if the resulting wall would lie on the exterior boundary
    /// of the maze (a vertical wall at x = WIDTH - 1 or a horizontal wall at y = HEIGHT - 1).
    ///
    /// # Examples
    ///
    /// ```
    /// use maze_solver::wall_maze::{InteriorWall, InteriorPosition, Orientation};
    ///
    /// let pos1 = InteriorPosition::<5, 5>::new(2, 2).unwrap();
    /// let wall1 = InteriorWall::<5, 5>::from_position_and_orientation(pos1, Orientation::Horizontal);
    /// assert!(wall1.is_ok());
    /// 
    /// let pos2 = InteriorPosition::<5, 5>::new(3, 4).unwrap();
    /// let wall2 = InteriorWall::<5, 5>::from_position_and_orientation(pos2, Orientation::Vertical);
    /// assert!(wall2.is_ok());
    /// let invalid_wall = InteriorWall::<5, 5>::from_position_and_orientation(pos2, Orientation::Horizontal);
    /// assert!(invalid_wall.is_err());
    /// ```
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

    /// Returns the x-coordinate of this wall.
    pub fn get_x(self) -> usize {
        self.wall.x
    }

    /// Returns the y-coordinate of this wall.
    pub fn get_y(self) -> usize {
        self.wall.y
    }

    /// Returns the orientation of this wall.
    pub fn get_orientation(self) -> Orientation {
        self.wall.orientation
    }

    /// Returns the underlying `Wall` structure.
    pub fn get_wall(self) -> Wall {
        self.wall
    }
}

/// A position guaranteed to be within the interior bounds of a maze.
///
/// This struct represents a cell position (x, y) that is valid for a maze of 
/// dimensions WIDTH × HEIGHT. It provides methods for navigating between positions
/// and determining relationships between positions in the maze.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InteriorPosition<const WIDTH: usize, const HEIGHT: usize> {
    x: usize,
    y: usize,
}

impl<const WIDTH: usize, const HEIGHT: usize> InteriorPosition<WIDTH, HEIGHT> {

    /// Creates a new `InteriorPosition<WIDTH, HEIGHT>` at the specified coordinates.
    ///
    /// # Errors
    ///
    /// Returns an error if x >= WIDTH or y >= HEIGHT.
    ///
    /// # Examples
    ///
    /// ```
    /// use maze_solver::wall_maze::InteriorPosition;
    ///
    /// let pos: Result<InteriorPosition<5, 5>, String> = InteriorPosition::new(2, 3);
    /// assert!(pos.is_ok());
    ///
    /// let invalid_pos = InteriorPosition::<5, 5>::new(5, 3);
    /// assert!(invalid_pos.is_err());
    /// ```
    pub fn new(x: usize, y: usize) -> Result<Self, String> {
        if x >= WIDTH {
            Err(format!("x coordinate {} is out of bounds for width {}", x, WIDTH))
        } else if y >= HEIGHT {
            Err(format!("y coordinate {} is out of bounds for height {}", y, HEIGHT))
        } else {
            Ok(Self { x, y })
        }
    }

    /// Returns the x-coordinate of this position.
    pub fn get_x(self) -> usize {
        self.x
    }

    /// Returns the y-coordinate of this position.
    pub fn get_y(self) -> usize {
        self.y
    }

    /// Returns a `Vec<InteriorPosition<WIDTH, HEIGHT>>` of all positions adjacent to this position.
    ///
    /// This includes positions to the left, right, up, and down, but only
    /// those that are within the maze boundaries.
    ///
    /// # Examples
    ///
    /// ```
    /// use maze_solver::wall_maze::InteriorPosition;
    ///
    /// let pos = InteriorPosition::<3, 3>::new(1, 1).unwrap();
    /// let adjacent = pos.adjacent_positions();
    /// 
    /// // A position in the middle of the maze has 4 adjacent positions
    /// assert_eq!(adjacent.len(), 4);
    ///
    /// // A position at the edge has fewer adjacent positions
    /// let corner = InteriorPosition::<3, 3>::new(0, 0).unwrap();
    /// assert_eq!(corner.adjacent_positions().len(), 2);
    /// ```
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

    /// Returns the Manhattan distance between this position and another position.
    ///
    /// Manhattan distance is the sum of the absolute differences of the
    /// x and y coordinates. This represents the minimum number of steps 
    /// required to reach one position from another in a grid without obstacles.
    ///
    /// # Examples
    ///
    /// ```
    /// use maze_solver::wall_maze::InteriorPosition;
    ///
    /// let pos1 = InteriorPosition::<5, 5>::new(0, 0).unwrap();
    /// let pos2 = InteriorPosition::<5, 5>::new(2, 3).unwrap();
    /// 
    /// assert_eq!(pos1.min_distance(pos2), 5); // |0-2| + |0-3| = 5
    /// assert_eq!(pos2.min_distance(pos1), 5); // Distance is symmetric
    /// ```
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

    /// Returns `true` if the other position is adjacent to this position ignoring walls.
    ///
    /// Two positions are adjacent if they share a side (left, right, up, or down),
    /// and both are within the maze boundaries.
    ///
    /// # Examples
    ///
    /// ```
    /// use maze_solver::wall_maze::InteriorPosition;
    ///
    /// let pos = InteriorPosition::<5, 5>::new(2, 2).unwrap();
    /// let adjacent = InteriorPosition::<5, 5>::new(2, 3).unwrap();
    /// let not_adjacent = InteriorPosition::<5, 5>::new(3, 3).unwrap();
    ///
    /// assert!(pos.adjacent_to(adjacent));
    /// assert!(!pos.adjacent_to(not_adjacent)); // Diagonals are not adjacent
    /// assert!(!pos.adjacent_to(pos)); // A position is not adjacent to itself
    /// ```
    pub fn adjacent_to(self, other: Self) -> bool {
        self.adjacent_positions().contains(&other)
    }

    /// Determines if this position is separated from another position by a wall in a given `WallMaze<WIDTH, HEIGHT>`.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The positions are not adjacent
    /// - The positions are the same
    ///
    /// # Examples
    ///
    /// ```
    /// use maze_solver::wall_maze::{InteriorPosition, WallMaze, Wall, Orientation};
    ///
    /// // Create a maze
    /// let start = InteriorPosition::<3, 3>::new(0, 0).unwrap();
    /// let end = InteriorPosition::<3, 3>::new(2, 2).unwrap();
    /// let mut maze = WallMaze::<3, 3>::new(start, end).unwrap();
    ///
    /// // Add a wall between (1,1) and (1,2)
    /// maze.add_wall(Wall::new(1, 1, Orientation::Horizontal)).unwrap();
    ///
    /// let pos1 = InteriorPosition::<3, 3>::new(1, 1).unwrap();
    /// let pos2 = InteriorPosition::<3, 3>::new(1, 2).unwrap();
    ///
    /// // Check if the positions are separated by a wall
    /// assert!(pos1.separated_by_wall(pos2, &maze).unwrap());
    /// 
    /// // Non-adjacent positions will return an error
    /// let pos3 = InteriorPosition::<3, 3>::new(0, 0).unwrap();
    /// assert!(pos1.separated_by_wall(pos3, &maze).is_err());
    /// ```
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

/// Represents a cardinal direction for movement within the maze.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    /// Toward the top of the maze (decreasing y-coordinate).
    Up,
    /// Toward the bottom of the maze (increasing y-coordinate).
    Down,
    /// Toward the left side of the maze (decreasing x-coordinate).
    Left,
    /// Toward the right side of the maze (increasing x-coordinate).
    Right
}
use Direction::{Up, Down, Left, Right};

impl Direction {
    /// Returns the opposite direction.
    ///
    /// # Examples
    ///
    /// ```
    /// use maze_solver::wall_maze::Direction;
    ///
    /// assert_eq!(Direction::Up.opposite(), Direction::Down);
    /// assert_eq!(Direction::Left.opposite(), Direction::Right);
    /// ```
    pub fn opposite(self) -> Self {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }

    /// An array containing all four cardinal directions.
    pub const ALL: [Direction; 4] = [Up, Down, Left, Right];
}

impl<const WIDTH: usize, const HEIGHT: usize> InteriorPosition<WIDTH, HEIGHT> {
    /// Creates a new `InteriorPosition<WIDTH, HEIGHT>` by shifting this position in the specified direction.
    ///
    /// # Errors
    ///
    /// Returns an error if moving in the specified direction would cause the position
    /// to leave the maze boundaries.
    ///
    /// # Examples
    ///
    /// ```
    /// use maze_solver::wall_maze::{InteriorPosition, Direction};
    ///
    /// let pos = InteriorPosition::<5, 5>::new(2, 2).unwrap();
    ///
    /// // Valid movements
    /// let down_pos = pos.shifted_by(Direction::Down);
    /// assert!(down_pos.is_ok());
    /// assert_eq!(down_pos.unwrap().get_y(), 3);
    /// assert_eq!(pos.get_y(), 2); // Original position is unchanged
    ///
    /// // Invalid movement - would leave the maze
    /// let corner = InteriorPosition::<5, 5>::new(0, 0).unwrap();
    /// let up_pos = corner.shifted_by(Direction::Up);
    /// assert!(up_pos.is_err());
    /// ```
    pub fn shifted_by(self, direction: Direction) -> Result<Self, String> {
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

    /// Mutates this position by moving it in the specified direction.
    ///
    /// # Errors
    ///
    /// Returns an error if moving in the specified direction would cause the position
    /// to leave the maze boundaries. In case of error, the position remains unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use maze_solver::wall_maze::{InteriorPosition, Direction};
    ///
    /// let mut pos = InteriorPosition::<5, 5>::new(2, 2).unwrap();
    /// 
    /// // Valid movement
    /// assert!(pos.move_by(Direction::Right).is_ok());
    /// assert_eq!(pos.get_x(), 3); // Position was changed
    ///
    /// // Invalid movement
    /// let mut corner = InteriorPosition::<5, 5>::new(0, 0).unwrap();
    /// assert!(corner.move_by(Direction::Left).is_err());
    /// assert_eq!(corner.get_x(), 0); // Position remains unchanged
    /// ```
    pub fn move_by(&mut self, direction: Direction) -> Result<(), String> {
        let new_pos = self.shifted_by(direction)?;
        *self = new_pos;
        Ok(())
    }
}

/// A maze with infinitely thin walls placed between grid cells.
///
/// The maze consists of a grid of positions with dimensions WIDTH × HEIGHT,
/// a start position, an end position, and a collection of walls. The maze
/// guarantees that there is always a path from the start to the end position.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WallMaze<const WIDTH: usize, const HEIGHT: usize> {
    start: InteriorPosition<WIDTH, HEIGHT>,
    end: InteriorPosition<WIDTH, HEIGHT>,
    walls: Vec<InteriorWall<WIDTH, HEIGHT>>
}

impl<const WIDTH: usize, const HEIGHT: usize> WallMaze<WIDTH, HEIGHT> {
    /// Creates a new empty `WallMaze<WIDTH, HEIGHT>` with the specified start and end positions.
    ///
    /// # Errors
    ///
    /// Returns an error if the start and end positions are the same.
    ///
    /// # Examples
    ///
    /// ```
    /// use maze_solver::wall_maze::{WallMaze, InteriorPosition};
    ///
    /// let start = InteriorPosition::<5, 5>::new(0, 0).unwrap();
    /// let end = InteriorPosition::<5, 5>::new(4, 4).unwrap();
    ///
    /// let maze = WallMaze::<5, 5>::new(start, end);
    /// assert!(maze.is_ok());
    ///
    /// // Start and end cannot be the same
    /// let same_pos = InteriorPosition::<5, 5>::new(2, 2).unwrap();
    /// let invalid_maze = WallMaze::<5, 5>::new(same_pos, same_pos);
    /// assert!(invalid_maze.is_err());
    /// ```
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

    /// Creates a new `WallMaze<WIDTH, HEIGHT>` with the specified start, end, and walls.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The start and end positions are the same
    /// - The maze is not solvable with the given walls
    ///
    /// # Examples
    ///
    /// ```
    /// use maze_solver::wall_maze::{WallMaze, InteriorPosition, InteriorWall, Orientation};
    ///
    /// let start = InteriorPosition::<5, 5>::new(0, 0).unwrap();
    /// let end = InteriorPosition::<5, 5>::new(4, 4).unwrap();
    ///
    /// // Create some walls that don't block all paths
    /// let walls = vec![
    ///     InteriorWall::<5, 5>::new(1, 1, Orientation::Horizontal).unwrap(),
    ///     InteriorWall::<5, 5>::new(2, 2, Orientation::Vertical).unwrap(),
    /// ];
    ///
    /// let maze = WallMaze::<5, 5>::from_walls(start, end, walls);
    /// assert!(maze.is_ok());
    /// ```
    pub fn from_walls(start: InteriorPosition<WIDTH, HEIGHT>, end: InteriorPosition<WIDTH, HEIGHT>, walls: Vec<InteriorWall<WIDTH, HEIGHT>>) -> Result<Self, String> {
        let maze = WallMaze {
            start,
            end,
            walls,
        };
        if !maze.solveable() {
            Err(format!("Maze is not solvable with the given walls"))
        } else {
            Ok(maze)
        }
    }

    /// Finds a path from the start to the end position in the maze.
    ///
    /// Returns a vector of positions representing the path, including both start and end positions.
    /// The path is guaranteed to be valid, moving only between adjacent positions that
    /// are not separated by walls.
    ///
    /// # Errors
    ///
    /// Returns an error if no path exists from start to end.
    ///
    /// # Examples
    ///
    /// ```
    /// use maze_solver::wall_maze::{WallMaze, InteriorPosition, Wall, Orientation};
    ///
    /// let start = InteriorPosition::<3, 3>::new(0, 0).unwrap();
    /// let end = InteriorPosition::<3, 3>::new(2, 2).unwrap();
    /// let mut maze = WallMaze::<3, 3>::new(start, end).unwrap();
    ///
    /// // Add some walls
    /// maze.add_wall(Wall::new(0, 0, Orientation::Vertical)).unwrap();
    /// maze.add_wall(Wall::new(0, 1, Orientation::Horizontal)).unwrap();
    ///
    /// // Solve the maze
    /// let path = maze.solve().unwrap();
    ///
    /// // Path starts at the start position and ends at the end position
    /// assert_eq!(path.first(), Some(&start));
    /// assert_eq!(path.last(), Some(&end));
    /// ```
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

    /// Returns whether the maze can be solved from start to end.
    ///
    /// This is a utility method used internally to ensure the maze remains solvable
    /// when adding or removing walls or changing start/end positions.
    fn solveable(&self) -> bool {
        self.solve().is_ok()
    }

    /// Removes an existing wall from the maze.
    ///
    /// # Errors
    ///
    /// Returns an error if the specified wall doesn't exist in the maze.
    ///
    /// # Examples
    ///
    /// ```
    /// use maze_solver::wall_maze::{WallMaze, InteriorPosition, InteriorWall, Orientation};
    ///
    /// let start = InteriorPosition::<5, 5>::new(0, 0).unwrap();
    /// let end = InteriorPosition::<5, 5>::new(4, 4).unwrap();
    /// let mut maze = WallMaze::<5, 5>::new(start, end).unwrap();
    ///
    /// // Add a wall
    /// let wall = InteriorWall::<5, 5>::new(1, 1, Orientation::Horizontal).unwrap();
    /// maze.add_interior_wall(wall).unwrap();
    ///
    /// // Remove the wall
    /// assert!(maze.remove_wall(wall).is_ok());
    ///
    /// // Trying to remove it again fails
    /// assert!(maze.remove_wall(wall).is_err());
    /// ```
    pub fn remove_wall(&mut self, interior_wall: InteriorWall<WIDTH, HEIGHT>) -> Result<(), String> {
        if let Some(pos) = self.walls.iter().position(|w| *w == interior_wall) {
            self.walls.remove(pos);
            Ok(())
        } else {
            Err(format!("Wall {:?} not found in the maze", interior_wall))
        }
    }

    /// Adds a wall to the maze.
    ///
    /// The wall is first validated to ensure it's within the maze boundaries.
    /// Then, the method verifies that adding this wall won't make the maze unsolvable.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The wall is outside the maze boundaries or on the exterior boundary
    /// - The wall already exists in the maze
    /// - Adding the wall would make the maze unsolvable
    ///
    /// # Examples
    ///
    /// ```
    /// use maze_solver::wall_maze::{WallMaze, InteriorPosition, Wall, Orientation};
    ///
    /// let start = InteriorPosition::<5, 5>::new(0, 0).unwrap();
    /// let end = InteriorPosition::<5, 5>::new(4, 4).unwrap();
    /// let mut maze = WallMaze::<5, 5>::new(start, end).unwrap();
    ///
    /// // Add a valid wall
    /// assert!(maze.add_wall(Wall::new(1, 1, Orientation::Horizontal)).is_ok());
    ///
    /// // Adding the same wall again fails
    /// assert!(maze.add_wall(Wall::new(1, 1, Orientation::Horizontal)).is_err());
    /// ```
    pub fn add_wall(&mut self, wall: Wall) -> Result<(), String> {
        let interior_wall = InteriorWall::from_wall(wall)?;
        self.add_interior_wall(interior_wall)
    }

    /// Adds a pre-validated interior wall to the maze.
    ///
    /// Unlike `add_wall`, this method takes an `InteriorWall<WIDTH, HEIGHT>` which has 
    /// already been validated to be within the maze boundaries.
    /// The method still verifies that adding this wall won't make the maze unsolvable.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The wall already exists in the maze
    /// - Adding the wall would make the maze unsolvable
    ///
    /// # Examples
    ///
    /// ```
    /// use maze_solver::wall_maze::{WallMaze, InteriorPosition, InteriorWall, Orientation};
    ///
    /// let start = InteriorPosition::<5, 5>::new(0, 0).unwrap();
    /// let end = InteriorPosition::<5, 5>::new(4, 4).unwrap();
    /// let mut maze = WallMaze::<5, 5>::new(start, end).unwrap();
    ///
    /// // Create a valid interior wall
    /// let wall = InteriorWall::<5, 5>::new(1, 1, Orientation::Horizontal).unwrap();
    /// 
    /// // Add the wall to the maze
    /// assert!(maze.add_interior_wall(wall).is_ok());
    ///
    /// // Adding the same wall again fails
    /// assert!(maze.add_interior_wall(wall).is_err());
    /// ```
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

    /// Changes the start position of the maze.
    ///
    /// This method verifies that the maze remains solvable after changing the
    /// start position by checking if there's a valid path from the new start 
    /// to the current start.
    ///
    /// # Errors
    ///
    /// Returns an error if changing the start would make the maze unsolvable.
    ///
    /// # Examples
    ///
    /// ```
    /// use maze_solver::wall_maze::{WallMaze, InteriorPosition, Wall, Orientation};
    ///
    /// let start = InteriorPosition::<5, 5>::new(0, 0).unwrap();
    /// let end = InteriorPosition::<5, 5>::new(4, 4).unwrap();
    /// let mut maze = WallMaze::<5, 5>::new(start, end).unwrap();
    ///
    /// // Add some walls but keep the maze solvable
    /// maze.add_wall(Wall::new(1, 0, Orientation::Horizontal)).unwrap();
    /// maze.add_wall(Wall::new(0, 1, Orientation::Vertical)).unwrap();
    ///
    /// // Change to a valid start position
    /// let new_start = InteriorPosition::<5, 5>::new(1, 1).unwrap();
    /// assert!(maze.move_start(new_start).is_ok());
    /// ```
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

    /// Swaps the start and end positions of the maze.
    ///
    /// This operation maintains maze solvability as it only exchanges the positions.
    ///
    /// # Examples
    ///
    /// ```
    /// use maze_solver::wall_maze::{WallMaze, InteriorPosition};
    ///
    /// let start = InteriorPosition::<5, 5>::new(0, 0).unwrap();
    /// let end = InteriorPosition::<5, 5>::new(4, 4).unwrap();
    /// let mut maze = WallMaze::<5, 5>::new(start, end).unwrap();
    ///
    /// // Before flipping
    /// let solution = maze.solve().unwrap();
    /// assert_eq!(solution.first(), Some(&start));
    /// assert_eq!(solution.last(), Some(&end));
    ///
    /// maze.flip_start_end();
    ///
    /// // After flipping
    /// let new_solution = maze.solve().unwrap();
    /// assert_eq!(new_solution.first(), Some(&end));
    /// assert_eq!(new_solution.last(), Some(&start));
    /// ```
    pub fn flip_start_end(&mut self) {
        (self.start, self.end) = (self.end, self.start);
    }

    /// Changes the end position of the maze.
    ///
    /// This method verifies that the maze remains solvable after changing the
    /// end position.
    ///
    /// # Errors
    ///
    /// Returns an error if changing the end position would make the maze unsolvable.
    ///
    /// # Examples
    ///
    /// ```
    /// use maze_solver::wall_maze::{WallMaze, InteriorPosition, Wall, Orientation};
    ///
    /// let start = InteriorPosition::<5, 5>::new(0, 0).unwrap();
    /// let end = InteriorPosition::<5, 5>::new(4, 4).unwrap();
    /// let mut maze = WallMaze::<5, 5>::new(start, end).unwrap();
    ///
    /// // Change to a valid end position
    /// let new_end = InteriorPosition::<5, 5>::new(3, 3).unwrap();
    /// assert!(maze.move_end(new_end).is_ok());
    /// ```
    pub fn move_end(&mut self, new_end: InteriorPosition<WIDTH, HEIGHT>) -> Result<(), String> {
        self.flip_start_end();
        match self.move_start(new_end) {
            Ok(_) => {
                self.flip_start_end();
                Ok(())
            },
            Err(_) => {
                self.flip_start_end();
                Err(format!("New end position {:?} would make the maze unsolvable", new_end))
            }
        }
    }
}
