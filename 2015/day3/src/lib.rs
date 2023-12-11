use std::collections::HashMap;

fn get_nextpos(pos: (i32, i32), dir: char) -> (i32, i32) {
    let (startx, starty) = pos;
    let nextpos = match dir {
        '^' => (startx, starty + 1),
        'v' => (startx, starty - 1),
        '>' => (startx + 1, starty),
        '<' => (startx - 1, starty),
        _   => unreachable!(),
    };
    return nextpos
}

pub fn solve_part1(input: &str) -> String {
    let mut grid = HashMap::new();
    let (mut startx, mut starty) = (0, 0);
    grid.entry((startx, starty)).or_insert(1);
    for dir in input.chars() {
        let nextpos = match dir {
            '^' => (startx, starty + 1),
            'v' => (startx, starty - 1),
            '>' => (startx + 1, starty),
            '<' => (startx - 1, starty),
            _   => unreachable!(),
        };
        *grid.entry(nextpos).or_insert(0) += 1;
        (startx, starty) = nextpos;
    }
    //for k in grid.keys() {
    //    println!("{:?}, {}", k, grid[k])
    //}
    grid.keys().count().to_string()
}

pub fn solve_part2(input: &str) -> String {
    let mut grid = HashMap::new();
    let (mut startx, mut starty) = (0, 0);
    grid.entry((startx, starty)).or_insert(2);
    let (even, odd): (Vec<_>, Vec<_>) = input.
                      chars().
                      enumerate()
                      .partition(|(i, _)| i % 2 == 0);

    for (_, dir) in even { 
        let nextpos = get_nextpos((startx, starty), dir);
        *grid.entry(nextpos).or_insert(0) += 1;
        (startx, starty) = nextpos;
    }
    (startx, starty) = (0, 0);
    for (_, dir) in odd {
        let nextpos = get_nextpos((startx, starty), dir);
        *grid.entry(nextpos).or_insert(0) += 1;
        (startx, starty) = nextpos;
    }
    grid.keys().count().to_string()
}
