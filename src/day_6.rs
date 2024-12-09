use std::io;
use std::collections::HashSet;
use std::collections::HashMap;

const UP: (i32, i32) = (-1, 0);
const DOWN: (i32, i32) = (1, 0);
const LEFT: (i32, i32) = (0, -1);
const RIGHT: (i32, i32) = (0, 1);
const UP_SYMBOL: char = '^';
const DOWN_SYMBOL: char = 'v';
const LEFT_SYMBOL: char = '<';
const RIGHT_SYMBOL: char = '>';

#[derive(Debug)]
struct Maze {
    directions: HashMap<char, (i32,i32)>,
    maze: Vec<Vec<char>>,
    guard_position: (i32, i32), 
    guard_direction: (i32, i32)
}

impl Maze {
    fn new(maze: Vec<Vec<char>> ) -> Self {
        let (guard_position,guard_direction) = ((0,0),(0,0));
        let directions = [
            (UP_SYMBOL, UP),
            (DOWN_SYMBOL, DOWN),
            (RIGHT_SYMBOL, RIGHT),
            (LEFT_SYMBOL, LEFT)
        ].into_iter().collect();
        let mut me = Self {
            directions,
            maze,
            guard_position,
            guard_direction,
        };
        (me.guard_position,me.guard_direction) = me.get_guard_pos_and_direction();
        me
    }
    fn get_guard_pos_and_direction(&mut self) -> ((i32,i32),(i32,i32)) {
        for (pos_x,list) in self.maze.iter().enumerate() {
            for (pos_y,element) in list.iter().enumerate() {
                if let Some(value) = self.directions.get(element) {
                    return ((pos_x as i32,pos_y as i32),*value);
                }
            }
        }
        ((0,0),(0,0))
    }
    fn move_symbol(&mut self) -> Option<(i32,i32)> {
        let target = (self.guard_position.0 + self.guard_direction.0,self.guard_position.1 + self.guard_direction.1);
        if target.0 < 0 || target.0 >= self.maze.len().try_into().unwrap() || target.1 < 0 || target.1 >= self.maze[target.0 as usize].len().try_into().unwrap() {
            return None;
        }
        //we move here
        match self.maze[target.0 as usize][target.1 as usize] {
            '.' | UP_SYMBOL | DOWN_SYMBOL | RIGHT_SYMBOL | LEFT_SYMBOL => {
                self.guard_position = target;
                return Some(target);
            }
            _ => {
                match self.guard_direction {
                    UP => {
                        self.guard_direction = RIGHT;
                    }
                    RIGHT => {
                        self.guard_direction = DOWN;
                    }
                    DOWN => {
                        self.guard_direction = LEFT;
                    }
                    LEFT => {
                        self.guard_direction = UP;
                    }
                    _ => {
                        return None;
                    }
                }
                return self.move_symbol();
            }
        }
    }
    fn detect_loop(&mut self) -> bool {
        let mut all_positions: HashSet<((i32,i32),(i32,i32))> = HashSet::new();
        let guard_start_pos = self.guard_position;
        let guard_start_dir = self.guard_direction;
        all_positions.insert((self.guard_position,self.guard_direction));
        while let Some(tile) = self.move_symbol() {
            //println!("guard_pos: {:?}\n guard_dir{:?}\n",self.guard_position,self.guard_direction);
            if !all_positions.insert((tile,self.guard_direction)) {
                return true;
            }
        }
        return false;
    }
}


fn wait_for_enter() {
    let mut input = String::new();
    println!("Press Enter to continue...");
    io::stdin().read_line(&mut input).expect("Failed to read line");
}

pub fn solve_maze(maze_board: Vec<Vec<char>>) -> Option<i32> {
    let mut rtn = 0;
    return Some(0);
    for (pos_x,list) in maze_board.iter().enumerate() {
        for (pos_y,value) in list.iter().enumerate() {
            match *value {
                '#' | UP_SYMBOL | DOWN_SYMBOL | RIGHT_SYMBOL | LEFT_SYMBOL => {
                    continue;
                }
                _ => {
                    let mut maze_board_cpy=maze_board.clone();
                    maze_board_cpy[pos_x][pos_y]='#';
                    let mut maze = Maze::new(maze_board_cpy);
                    if maze.detect_loop() {
                        //println!("guard_pos: {:?}\n guard_dir{:?}\n new wall: {:?}",maze.guard_position,maze.guard_direction,(pos_x,pos_y));
                        //wait_for_enter();
                        rtn +=1;
                    }
                }
            }
        }
    }
    Some(rtn)
}
