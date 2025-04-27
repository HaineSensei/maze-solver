use super::*;

#[test]
fn test_interior_position_creation() {
    const WIDTH: usize = 5;
    const HEIGHT: usize = 5;
    
    assert!(InteriorPosition::<WIDTH, HEIGHT>::new(0, 0).is_ok());
    assert!(InteriorPosition::<WIDTH, HEIGHT>::new(4, 4).is_ok());
    assert!(InteriorPosition::<WIDTH, HEIGHT>::new(5, 0).is_err());
    assert!(InteriorPosition::<WIDTH, HEIGHT>::new(0, 5).is_err());
}

#[test]
fn test_wall_creation() {
    const WIDTH: usize = 5;
    const HEIGHT: usize = 5;

    assert!(InteriorWall::<WIDTH, HEIGHT>::new(0, 0, Horizontal).is_ok());
    assert!(InteriorWall::<WIDTH, HEIGHT>::new(0, 0, Vertical).is_ok());
    assert!(InteriorWall::<WIDTH, HEIGHT>::new(2, 4, Horizontal).is_err());
    assert!(InteriorWall::<WIDTH, HEIGHT>::new(4, 2, Vertical).is_err());
}

#[test]
fn test_maze_solvability() {
    const WIDTH: usize = 3;
    const HEIGHT: usize = 3;
    
    let start = InteriorPosition::<WIDTH, HEIGHT>::new(0, 0).unwrap();
    let end = InteriorPosition::<WIDTH, HEIGHT>::new(2, 2).unwrap();
    let mut maze = WallMaze::<WIDTH, HEIGHT>::new(start, end).unwrap();
    
    assert!(maze.add_wall(Wall { x: 0, y: 0, orientation: Vertical }).is_ok());
    assert!(maze.add_wall(Wall { x: 0, y: 1, orientation: Horizontal }).is_ok());
    
    // This wall would make maze unsolvable
    assert!(maze.add_wall(Wall { x: 0, y: 1, orientation: Vertical }).is_err());
}

#[test]
fn test_adjacent_positions() {
    const WIDTH: usize = 3;
    const HEIGHT: usize = 3;
    
    let pos = InteriorPosition::<WIDTH, HEIGHT>::new(1, 1).unwrap();
    let adjacent = pos.adjacent_positions();
    
    assert_eq!(adjacent.len(), 4);
    assert!(adjacent.contains(&InteriorPosition::new(0, 1).unwrap()));
    assert!(adjacent.contains(&InteriorPosition::new(2, 1).unwrap()));
    assert!(adjacent.contains(&InteriorPosition::new(1, 0).unwrap()));
    assert!(adjacent.contains(&InteriorPosition::new(1, 2).unwrap()));
}

#[test]
fn test_min_distance() {
    const WIDTH: usize = 5;
    const HEIGHT: usize = 5;
    
    let pos1 = InteriorPosition::<WIDTH, HEIGHT>::new(0, 0).unwrap();
    let pos2 = InteriorPosition::<WIDTH, HEIGHT>::new(2, 3).unwrap();
    
    assert_eq!(pos1.min_distance(pos2), 5);
    assert_eq!(pos2.min_distance(pos1), 5);
}

#[test] 
fn test_solve_maze() {
    const WIDTH: usize = 5;
    const HEIGHT: usize = 5;

    let start = InteriorPosition::<WIDTH, HEIGHT>::new(0, 0).unwrap();
    let end = InteriorPosition::<WIDTH, HEIGHT>::new(4, 4).unwrap();
    let mut maze = WallMaze::<WIDTH, HEIGHT>::new(start, end).unwrap();

    let walls = [
        InteriorWall::new(0,0, Vertical).unwrap(),
        InteriorWall::new(0,1, Vertical).unwrap(),
        InteriorWall::new(0,3, Vertical).unwrap(),
        InteriorWall::new(1,1, Horizontal).unwrap(),
        InteriorWall::new(1,3, Vertical).unwrap(),
        InteriorWall::new(1,3, Horizontal).unwrap(),
        InteriorWall::new(2,0, Horizontal).unwrap(),
        InteriorWall::new(2,1, Vertical).unwrap(),
        InteriorWall::new(2,2, Vertical).unwrap(),
        InteriorWall::new(2,2, Horizontal).unwrap(),
        InteriorWall::new(2,3, Vertical).unwrap(),
        InteriorWall::new(2,4, Vertical).unwrap(),
        InteriorWall::new(3,1, Vertical).unwrap(),
        InteriorWall::new(3,2, Vertical).unwrap(),
        InteriorWall::new(3,3, Horizontal).unwrap(),
        InteriorWall::new(4,2, Horizontal).unwrap()
    ];

    for wall in walls.iter() {
        maze.add_interior_wall(wall.clone()).unwrap();
    }
    let path = maze.solve().unwrap();
    
    assert_eq!(path, vec![
        InteriorPosition::new(0,0).unwrap(),
        InteriorPosition::new(0,1).unwrap(),
        InteriorPosition::new(0,2).unwrap(),
        InteriorPosition::new(1,2).unwrap(),
        InteriorPosition::new(2,2).unwrap(),
        InteriorPosition::new(2,1).unwrap(),
        InteriorPosition::new(1,1).unwrap(),
        InteriorPosition::new(1,0).unwrap(),
        InteriorPosition::new(2,0).unwrap(),
        InteriorPosition::new(3,0).unwrap(),
        InteriorPosition::new(3,1).unwrap(),
        InteriorPosition::new(3,2).unwrap(),
        InteriorPosition::new(3,3).unwrap(),
        InteriorPosition::new(4,3).unwrap(),
        InteriorPosition::new(4,4).unwrap()
    ]);
}