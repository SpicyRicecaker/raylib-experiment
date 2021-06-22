pub mod rotation_direction;

/// Tetrominos of type J, L, S, T or Z each have 5 tests, accounting for each of the 4 indices, each with a cartesion coord
pub const JLSTZ_OFFSET_DATA: [[[i32; 2]; 4]; 5] = [
    [[0, 0], [0, 0], [0, 0], [0, 0]],
    [[0, 0], [1, 0], [0, 0], [-1, 0]],
    [[0, 0], [1, -1], [0, 0], [-1, -1]],
    [[0, 0], [0, 2], [0, 0], [0, 2]],
    [[0, 0], [1, 2], [0, 0], [-1, 2]],
];

/// Tetromino of type  has 5 tests, each with 4 indices, each with a cartesion coord
pub const I_OFFSET_DATA: [[[i32; 2]; 4]; 5] = [
    [[0, 0], [-1, 0], [-1, 1], [0, 1]],
    [[-1, 0], [0, 0], [1, 1], [0, 1]],
    [[2, 0], [0, 0], [-2, 1], [0, -1]],
    [[-1, 0], [0, 1], [1, 0], [0, -1]],
    [[2, 0], [0, -2], [-2, 0], [0, 2]],
];
pub const O_OFFSET_DATA: [[[i32; 2]; 4]; 1] = [[[0, 0], [0, -1], [-1, -1], [-1, 0]]];