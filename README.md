# WIP

- [x] left & right hit detection
  - [x] No panic moving tetromino to left
  - [x] No moving tetromino over right boundary
- [x] generation of 7 tetrominos randomly

  - `rand` crate?
    - [x] implement `tetrominos.rs` -> ln `224`, random enums please!
      - [x] replace all instances of `generate_tetromino_from_type`

- [x] Remove full lines
- [x] Long press go down
  - [x] Remove ability of tetris input to modify universe; universe should provide its own interface and accept keys from tetris input
- [ ] _Rotation_
  - WE DON'T NEED TO HOLD CENTER
  - [ ] Need to add the offset tables
- [ ] Game Over / Restart Screen
- [ ] colors
- [ ] sound
  - [ ] music in the background
  - [ ] TETRIS for !
  - [ ] bouncing against the side