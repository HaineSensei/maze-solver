use std::{collections::HashMap, hash::Hash};

pub mod wall_maze;

pub trait MazeCell {
    fn adjacent_cells(&self) -> impl Iterator<Item = &Self>;
}
pub fn are_adjacent<T: MazeCell + PartialEq>(cell1: &T, cell2: &T) -> bool {
    let adjacent = cell1.adjacent_cells();
    for adj in adjacent {
        if adj == cell2 {
            return true;
        }
    }
    false
}

pub trait PathHeuristic {
    fn heuristic(&self, other: &Self) -> f64;
}

pub trait Maze {
    type Cell: MazeCell;

    fn start(&self) -> Self::Cell;
    fn end(&self) -> Self::Cell;
    fn try_solve(&self) -> Option<Vec<Self::Cell>>;

    fn solve(&self) -> Result<Vec<Self::Cell>, String> {
        if let Some(path) = self.try_solve() {
            Ok(path)
        } else {
            Err("No solution found".to_string())
        }
    }
}

pub trait MazeWall {
    type Cell: MazeCell;

    fn surrounding_cells(&self) -> [Self::Cell; 2];
}

pub trait WallMaze: Maze {
    type Wall: MazeWall<Cell = Self::Cell>;

    fn add_wall(&mut self, wall: Self::Wall) -> Result<(), String>;
    fn remove_wall(&mut self, wall: Self::Wall) -> Result<(), String>;
    fn add_interior_wall(&mut self, wall: Self::Wall) -> Result<(), String>;
    fn remove_interior_wall(&mut self, wall: Self::Wall) -> Result<(), String>;
    fn separated_by_wall(&self, cell1: &Self::Cell, cell2: &Self::Cell) -> bool;

    fn try_solve(&self) -> Option<Vec<Self::Cell>> where Self::Cell: Hash + Eq + Clone {
        let mut unchecked = vec![self.start()];
        let mut path_to = HashMap::new();
        path_to.insert(self.start(), vec![self.start()]);
        loop {
            if path_to.contains_key(&self.end()) {
                break;
            }
            if unchecked.is_empty() {
                return None;
            }
            let current = unchecked.pop().unwrap();
            for adj in current.adjacent_cells() {
                if path_to.contains_key(&adj) {
                    continue;
                }
                if self.separated_by_wall(&current, &adj) {
                    continue;
                }
                unchecked.push(adj.clone());
                let mut new_path = path_to.get(&current).unwrap().clone();
                new_path.push(adj.clone());
                path_to.insert(adj.clone(), new_path);
            }
        }
        Some(path_to.get(&self.end()).unwrap().clone())
    }
}

pub trait HeuristicWallMaze: WallMaze where Self::Cell: PathHeuristic {
    fn try_solve(&self) -> Option<Vec<Self::Cell>> where Self::Cell: Hash + Eq + Clone {
        let mut unchecked = vec![self.start()];
        let mut path_to = HashMap::new();
        path_to.insert(self.start(), vec![self.start()]);
        loop {
            if path_to.contains_key(&self.end()) {
                break;
            }
            if unchecked.is_empty() {
                return None;
            }
            let value = |pos: &Self::Cell| {
                pos.heuristic(&self.end()) + path_to.get(pos).unwrap().len() as f64
            };
            unchecked.sort_by(|a, b| {
                value(a).total_cmp(&value(b)).reverse()
            });
            let current = unchecked.pop().unwrap();
            for adj in current.adjacent_cells() {
                if path_to.contains_key(&adj) {
                    continue;
                }
                if self.separated_by_wall(&current, &adj) {
                    continue;
                }
                unchecked.push(adj.clone());
                let mut new_path = path_to.get(&current).unwrap().clone();
                new_path.push(adj.clone());
                path_to.insert(adj.clone(), new_path);
            }
        }
        Some(path_to.get(&self.end()).unwrap().clone())
    }

}

impl<T: WallMaze> HeuristicWallMaze for T where T::Cell: PathHeuristic {}

pub trait MutSolubleMaze: Maze {
    fn move_start(&mut self, new_start: Self::Cell) -> Result<(), String>;
    fn flip_start_end(&mut self);

    fn move_end(&mut self, new_end: Self::Cell) -> Result<(), String> {
        self.flip_start_end();
        match self.move_start(new_end) {
            Ok(_) => {
                self.flip_start_end();
                Ok(())
            }
            Err(e) => {
                self.flip_start_end();
                Err(e)
            }
        }
    }
}

pub trait MutSolubleWallMaze: WallMaze + MutSolubleMaze {
    fn add_wall(&mut self, wall: Self::Wall) -> Result<(), String>;
    fn remove_wall(&mut self, wall: Self::Wall) -> Result<(), String>;
}

pub trait BlockMaze: Maze {
    fn blocks(&self) -> Vec<Self::Cell>;
}

pub trait MutSolubleBlockMaze: BlockMaze + MutSolubleMaze {
    fn add_block(&mut self, block: Self::Cell) -> Result<(), String>;
    fn remove_block(&mut self, block: Self::Cell) -> Result<(), String>;
}



#[cfg(test)]
mod tests {
    
}