#[allow(unused_imports)]
use std::{thread, vec};

// RNG
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
// Image creation
use image::ImageBuffer;

type Grid = Vec<Vec<Tile>>;
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum TileType {
    Floor,
    Wall,
    Start,
    Boss,
    Event,
    Water,
    Forest,
}
#[derive(Clone)]
struct FloorPattern {
    // odds: f32,
    rng_range_multiplicator_rectangle_size: (f32, f32),
    rng_range_number_of_direction_changes: (i32, i32),
    rng_range_direction_repeat: (i32, i32),
    allowed_directions: Vec<(i32, i32)>,
    generation_area_size: (i32, i32),
}
#[derive(PartialEq, Eq)]
struct Tile {
    tile_type: TileType,
}
#[derive(Clone)]
struct Map {
    name: String,
    oob_type: TileType,
    biomes: Vec<FloorPattern>,
    generation_init_center: (i32, i32),
}
// const DIRECTIONS_OUTDOOR: [(i32, i32); 8] = [
//     (0, -1), // ↑
//     (0, 1),  // ↓
//     (-1, 0), // ←
//     (1, 0),  // →
//     (1, -1),
//     (1, 1),
//     (-1, 1),
//     (1, 1),
// ];

// const DIRECTIONS4: [(i32, i32); 4] = [
//     (0, -1), // ↑
//     (0, 1),  // ↓
//     (-1, 0), // ←
//     (1, 0),  // →
// ];

fn main() {
    // Create random generator from seed
    // fixed seed
    // let seed: u64 = 142857;
    // random seed
    let seed: u64 = rand::random();
    println!("New seed is {}", seed);

    //------------------------------------------------------//
    //                Define all Floor Patterns             //
    //------------------------------------------------------//
    let large_all_dir = FloorPattern {
        // odds: 1.0,
        rng_range_multiplicator_rectangle_size: (0.1, 0.2),
        rng_range_number_of_direction_changes: (4, 5),
        rng_range_direction_repeat: (1, 3),
        allowed_directions: vec![
            (0, -1),
            (0, 1),
            (-1, 0),
            (1, 0),
            (1, -1),
            (1, 1),
            (-1, 1),
            (1, 1),
        ],
        generation_area_size: (230, 230),
    };
    let small_cross_dir = FloorPattern {
        rng_range_multiplicator_rectangle_size: (0.02, 0.06),
        rng_range_number_of_direction_changes: (20, 30),
        rng_range_direction_repeat: (5, 10),
        allowed_directions: vec![(0, -1), (0, 1), (-1, 0), (1, 0)],
        generation_area_size: (230, 230),
    };
    let small_all_dir = FloorPattern {
        rng_range_multiplicator_rectangle_size: (0.02, 0.04),
        rng_range_number_of_direction_changes: (15, 25),
        rng_range_direction_repeat: (10, 15),
        allowed_directions: vec![
            (0, -1),
            (0, 1),
            (-1, 0),
            (1, 0),
            (1, -1),
            (1, 1),
            (-1, 1),
            (-1, -1),
        ],
        generation_area_size: (230, 230),
    };
    let many_tiny_all_dir = FloorPattern {
        rng_range_multiplicator_rectangle_size: (0.01, 0.020),
        rng_range_number_of_direction_changes: (30, 40),
        rng_range_direction_repeat: (10, 15),
        allowed_directions: vec![(1, -1), (1, 1), (-1, 1), (-1, -1)],
        generation_area_size: (230, 230),
    };
    let long_path_bottom_right_dir = FloorPattern {
        rng_range_multiplicator_rectangle_size: (0.01, 0.020),
        rng_range_number_of_direction_changes: (20, 30),
        rng_range_direction_repeat: (10, 15),
        allowed_directions: vec![(1, -1), (1, 1), (-1, 1)],
        generation_area_size: (230, 230),
    };
    let short_path_bottom_right_dir = FloorPattern {
        rng_range_multiplicator_rectangle_size: (0.01, 0.020),
        rng_range_number_of_direction_changes: (10, 15),
        rng_range_direction_repeat: (5, 8),
        allowed_directions: vec![(1, -1), (1, 1), (-1, 1)],

        generation_area_size: (230, 230),
    };
    //------------------------------------------------------//
    //                Define Maps Content                   //
    //------------------------------------------------------//
    let mut maps: Vec<Map> = vec![
        Map {
            name: String::from("Island"),
            oob_type: TileType::Water,
            biomes: vec![many_tiny_all_dir.clone(), small_all_dir.clone()],
            generation_init_center: (250, 250),
        },
        Map {
            name: String::from("Ledge"),
            oob_type: TileType::Wall,
            biomes: vec![
                long_path_bottom_right_dir.clone(),
                long_path_bottom_right_dir.clone(),
            ],
            generation_init_center: (20, 20),
        },
        Map {
            name: String::from("Desert"),
            oob_type: TileType::Wall,
            biomes: vec![long_path_bottom_right_dir.clone(), large_all_dir.clone()],
            generation_init_center: (150, 150),
        },
        Map {
            name: String::from("Forest"),
            oob_type: TileType::Forest,
            biomes: vec![
                short_path_bottom_right_dir.clone(),
                small_cross_dir.clone(),
                small_cross_dir.clone(),
            ],
            generation_init_center: (250, 250),
        },
        Map {
            name: String::from("Quarry"),
            oob_type: TileType::Wall,
            biomes: vec![
                short_path_bottom_right_dir.clone(),
                many_tiny_all_dir.clone(),
                many_tiny_all_dir.clone(),
                short_path_bottom_right_dir.clone(),
            ],
            generation_init_center: (250, 250),
        },
    ];
    //------------------------------------------------------//
    //               Generate maps                          //
    //------------------------------------------------------//

    // for map in maps {
    //     generate_map(seed, map);
    // }
    let mut handlers = Vec::new();
    while let Some(map) = maps.pop() {
        handlers.push(thread::spawn(move || {
            generate_map(seed, map);
        }));
    }
    for handler in handlers {
        handler.join().unwrap();
    }
}

fn generate_map(seed: u64, map: Map) {
    // The rng instance is created from the seed
    let mut rng: ChaCha8Rng = ChaCha8Rng::seed_from_u64(seed);

    // Roll initial biome, which defines the out of bound tile type
    // let biome = biomes[rng.gen_range(0..biomes.len())];
    println!("-----For Biome {}", map.name);
    let oob_tiletype = map.oob_type;

    // Initialize map grid from initial biome and oob tile type
    let mut grid: Grid = init_grid(500, 500, oob_tiletype);

    // genrate walkable paths based on a random selection of possible biomes
    let mut center = map.generation_init_center;
    let map_start = center;

    for i in 0..map.biomes.len() {
        center =
            generate_walkable_layout(&mut grid, &map.biomes[i], &mut rng, oob_tiletype, center);
    }

    //TODO
    // Centering and cropping maps, retry if oob

    // remove small clusters of oob tiles
    remove_small_cluster(&mut grid, oob_tiletype, 3, false, true);
    remove_small_cluster(&mut grid, oob_tiletype, 3, true, false);
    remove_small_cluster(&mut grid, oob_tiletype, 3, false, true);

    // TODO
    // add Start of map, first center and last center
    draw_rectangle(&mut grid, TileType::Start, (2, 2), map_start);
    draw_rectangle(&mut grid, TileType::Boss, (2, 2), center);

    // TODO
    // add a map attribute bool, to remove or not the "inside shapes"
    // start by flagging all
    // Convert generated tile map oob to largest rectangle
    // render_grid(&grid, map.name.clone() + "_before");
    resize_grid(&mut grid, 4);

    println!(
        "final size for {} is {} {}",
        map.name,
        grid.len(),
        grid[0].len()
    );
    // print grid
    render_grid(&grid, map.name.clone());
}

fn resize_grid(grid: &mut Grid, border_size: usize) {
    // for each direction
    // left to right
    let height = grid[0].len();
    'outer: loop {
        for y in 0..height {
            if grid[border_size][y].tile_type == TileType::Floor {
                break 'outer;
            }
        }
        grid.remove(0);
    }
    //right to left
    let mut x = grid.len() - 1;

    'outer: loop {
        for y in 0..height {
            if grid[x - border_size][y].tile_type == TileType::Floor {
                break 'outer;
            }
        }
        x -= 1;
    }
    grid.truncate(x);
    // bottom to up
    let mut y = 0;
    let width = grid.len();
    'outer: loop {
        for x in 0..width {
            if grid[x][y + border_size].tile_type == TileType::Floor {
                break 'outer;
            }
        }
        y += 1;
    }
    for x in 0..width {
        for _ in 0..y {
            grid[x].remove(0);
        }
    }
    // Top to bottom
    y = grid[0].len() - 1;

    'outer: loop {
        for x in 0..width {
            if grid[x][y - border_size].tile_type == TileType::Floor {
                break 'outer;
            }
        }
        y -= 1;
    }
    for x in 0..width {
        grid[x].truncate(y);
    }
}

fn remove_small_cluster(
    grid: &mut Grid,
    oob_tiletype: TileType,
    cluster_size: usize,
    check_x: bool,
    check_y: bool,
) {
    let mut tiles_to_fill = Vec::new();
    // for all tiles
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            // if we are on a oob tile type
            if grid[x][y].tile_type == oob_tiletype
                && (x + cluster_size) < grid.len()
                && (x as i32 - cluster_size as i32) > 0
                && (y + cluster_size) < grid[0].len()
                && (y as i32 - cluster_size as i32) > 0
            {
                // check in each direction if there is a walkable tile next to it
                let mut is_floor_up: bool = false;
                let mut is_floor_bottom: bool = false;
                let mut is_floor_left: bool = false;
                let mut is_floor_right: bool = false;
                let mut floor_up_at = 0;
                let mut floor_bottom_at = 0;
                let mut floor_left_at = 0;
                let mut floor_right_at = 0;
                for i in 1..cluster_size {
                    if grid[x + i][y].tile_type == TileType::Floor {
                        is_floor_right = true;
                        floor_right_at = i;
                    }
                    if grid[x - i][y].tile_type == TileType::Floor {
                        is_floor_left = true;
                        floor_left_at = i;
                    }
                    if grid[x][y + i].tile_type == TileType::Floor {
                        is_floor_bottom = true;
                        floor_bottom_at = i;
                    }
                    if grid[x][y - i].tile_type == TileType::Floor {
                        is_floor_up = true;
                        floor_up_at = i;
                    }
                }
                if check_x && check_y {
                    if is_floor_right && is_floor_left && is_floor_bottom && is_floor_up {
                        for i in 1..floor_bottom_at {
                            tiles_to_fill.push((x, y + i));
                        }
                        for i in 1..floor_up_at {
                            tiles_to_fill.push((x, y - i));
                        }
                        for i in 1..floor_left_at {
                            tiles_to_fill.push((x - i, y));
                        }
                        for i in 1..floor_right_at {
                            tiles_to_fill.push((x + i, y));
                        }
                        tiles_to_fill.push((x, y));
                    }
                } else {
                    if (check_x) && is_floor_right && is_floor_left {
                        for i in 1..floor_left_at {
                            tiles_to_fill.push((x - i, y));
                        }
                        for i in 1..floor_right_at {
                            tiles_to_fill.push((x + i, y));
                        }
                        tiles_to_fill.push((x, y));
                    }

                    if (check_y) && is_floor_bottom && is_floor_up {
                        for i in 1..floor_bottom_at {
                            tiles_to_fill.push((x, y + i));
                        }
                        for i in 1..floor_up_at {
                            tiles_to_fill.push((x, y - i));
                        }
                        tiles_to_fill.push((x, y));
                    }
                }
            }
        }
    }
    // after full scan, update tileset
    for tile in tiles_to_fill {
        add_tile(grid, tile.0, tile.1, TileType::Floor);
    }
}
fn generate_walkable_layout(
    grid: &mut Grid,
    biome: &FloorPattern,
    rng: &mut ChaCha8Rng,
    oob_tiletype: TileType,
    start_center: (i32, i32),
) -> (i32, i32) {
    draw_rectangle(
        grid,
        TileType::Floor,
        (
            (biome.generation_area_size.0 as f32
                * rng.gen_range(
                    biome.rng_range_multiplicator_rectangle_size.0
                        ..biome.rng_range_multiplicator_rectangle_size.1,
                ))
            .round() as i32,
            (biome.generation_area_size.0 as f32
                * rng.gen_range(
                    biome.rng_range_multiplicator_rectangle_size.0
                        ..biome.rng_range_multiplicator_rectangle_size.1,
                ))
            .round() as i32,
        ),
        start_center,
    );

    let mut center: (i32, i32) = start_center;
    for _ in 0..rng.gen_range(
        biome.rng_range_number_of_direction_changes.0
            ..biome.rng_range_number_of_direction_changes.1,
    ) {
        let iterrations: i32 =
            rng.gen_range(biome.rng_range_direction_repeat.0..biome.rng_range_direction_repeat.1);

        for _ in 0..iterrations {
            let direction: (i32, i32) =
                biome.allowed_directions[rng.gen_range(0..biome.allowed_directions.len())];

            center = find_point_on_edge(grid, center, oob_tiletype, direction);
            draw_rectangle(
                grid,
                TileType::Floor,
                (
                    (biome.generation_area_size.0 as f32
                        * rng.gen_range(
                            biome.rng_range_multiplicator_rectangle_size.0
                                ..biome.rng_range_multiplicator_rectangle_size.1,
                        ))
                    .round() as i32,
                    (biome.generation_area_size.0 as f32
                        * rng.gen_range(
                            biome.rng_range_multiplicator_rectangle_size.0
                                ..biome.rng_range_multiplicator_rectangle_size.1,
                        ))
                    .round() as i32,
                ),
                center,
            );
        }
    }
    center
}

fn find_point_on_edge(
    grid: &Grid,
    previous_center: (i32, i32),
    oob_tiletype: TileType,
    direction: (i32, i32),
) -> (i32, i32) {
    let mut current_position = previous_center;
    while (current_position.0 + 1) < grid.len() as i32
        && (current_position.1 + 1) < grid.len() as i32
        && (current_position.0 - 1) > 0
        && (current_position.1 - 1) > 0
        && grid[(current_position.0 + direction.0) as usize]
            [(current_position.1 + direction.1) as usize]
            .tile_type
            != oob_tiletype
    {
        current_position = (
            (current_position.0 + direction.0),
            (current_position.1 + direction.1),
        )
    }
    current_position
}

// fn roll_direction(direction: &Vec<((i32, i32), i32)>, rng: &mut ChaCha8Rng) -> (i32, i32) {
//     let total_probability = direction
//         .iter()
//         .copied()
//         .reduce(|a, b| ((a.0 .0 + b.0 .0, a.0 .1 + b.0 .1), (a.1 + b.1)))
//         .unwrap();
//     let coin_toss = rng.gen_range(0..total_probability.1);
//     let mut cumul = 0;
//     let mut chosen_direction = (0, 0);
//     for odd in direction {
//         let old_cumul = cumul;
//         cumul += odd.1;
//         if coin_toss <= cumul && coin_toss > old_cumul {
//             chosen_direction = odd.0;
//         }
//     }
//     chosen_direction
// }

fn draw_rectangle(grid: &mut Grid, tiletype: TileType, size: (i32, i32), center: (i32, i32)) {
    for x in 0..size.0 {
        for y in 0..size.1 {
            add_tile(
                grid,
                ((center.0 - (size.0 / 2)) + x) as usize,
                ((center.1 - (size.1 / 2)) + y) as usize,
                tiletype,
            )
        }
    }
}
// input taille, coordonees du centre

// fn draw_circle(
//     grid: &mut Grid,
//     tiletype: TileType,
//     inbound: bool,
//     center: (f32, f32),
//     radius: f32,
// ) {
//     let width = grid.len();
//     let height = grid[0].len();

//     // println!("Maps is {} wide and {} tall ", center_x, center_y);
//     for x in 0..width {
//         for y in 0..height {
//             let dx = f32::abs(x as f32 - center.0);
//             let dy = f32::abs(y as f32 - center.1);
//             if dx * dx + dy * dy <= radius * radius {
//                 if inbound {
//                     add_tile(grid, x, y, tiletype)
//                 }
//             } else if !inbound {
//                 add_tile(grid, x, y, tiletype)
//             }
//         }
//     }
// }

fn add_tile(grid: &mut Grid, x: usize, y: usize, tile_type: TileType) {
    if x < grid.len() && y < grid.len() {
        grid[x][y].tile_type = tile_type;
    }
}

fn init_grid(height: i32, width: i32, oob_tiletype: TileType) -> Grid {
    let mut grid: Grid = Vec::new();
    for _ in 0..width {
        let mut row = Vec::new();
        for _ in 0..height {
            row.push(Tile {
                tile_type: oob_tiletype,
            })
        }
        grid.push(row)
    }
    grid
}

fn render_grid(grid: &Grid, file_name: String) {
    // Construct a new RGB ImageBuffer with the specified width and height.
    let width = grid.len();
    let height = grid[0].len();

    // Construct a new by repeated calls to the supplied closure.
    let mut img = ImageBuffer::new(width.try_into().unwrap(), height.try_into().unwrap());

    for (i, x) in grid.iter().enumerate() {
        for (j, y) in x.iter().enumerate() {
            img.put_pixel(
                i.try_into().unwrap(),
                j.try_into().unwrap(),
                match y.tile_type {
                    TileType::Boss => image::Rgb([0u8, 0u8, 0u8]),
                    TileType::Floor => image::Rgb([230u8, 213u8, 168u8]),
                    TileType::Wall => image::Rgb([122u8, 97u8, 31u8]),
                    TileType::Start => image::Rgb([182u8, 51u8, 214u8]),
                    TileType::Event => image::Rgb([181u8, 181u8, 181u8]),
                    TileType::Water => image::Rgb([51u8, 114u8, 214u8]),
                    TileType::Forest => image::Rgb([42u8, 117u8, 14u8]),
                },
            );
        }
    }
    img.save("output/".to_string() + &file_name + ".png")
        .unwrap();
}
