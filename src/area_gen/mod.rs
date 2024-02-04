// Custom
use floor_pattern::{FloorPattern, Map, Tile, TileType};
// RNG
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
// Image creation
use image::ImageBuffer;

mod floor_pattern;

type Grid = Vec<Vec<Tile>>;

const TILE_SIZE: i32 = 60;

pub struct AreaGenerationOutput {
    pub width: u32,
    pub height: u32,
    pub walkable_x: Vec<u32>,
    pub walkable_y: Vec<u32>,
    pub oob_polygons: Vec<Shape>, // bool is true when outer oob shape, false when inner
    pub player_spawn_position: (i32, i32),
}

pub struct Shape {
    pub points: Vec<(f32, f32)>,
    pub inner_if_true: bool,
}

pub fn generate_area(map_index: usize) -> AreaGenerationOutput {
    // Create random generator from seed
    // fixed seed
    // let seed: u64 = 142857;
    // random seed
    let seed: u64 = rand::random();

    let mut maps = floor_pattern::define_floor_patterns();
    //------------------------------------------------------//
    //               Generate maps                          //
    //------------------------------------------------------//

    // Pick a map
    let map = maps.remove(map_index);
    let map_name = map.name.clone();
    // Generate map grid
    let (mut grid, player_spawn_position) = generate_map(seed, map);

    //------------------------------------------------------//
    //               Find oob polygons                      //
    //------------------------------------------------------//
    let oob_polygons = find_oob_polygons(&mut grid);
    render_grid(&grid, map_name.clone() + "_outline", true);
    render_grid(&grid, map_name.clone(), false);

    // Initiate module outputf
    let mut walkable_x = Vec::new();
    let mut walkable_y = Vec::new();
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y].tile_type == TileType::Floor
                || grid[x][y].tile_type == TileType::Start
                || grid[x][y].tile_type == TileType::Boss
            {
                walkable_x.push(x as u32);
                walkable_y.push(y as u32);
            }
        }
    }
    println!(
        "----------------------------\nSeed : {} \n    Biome : {}\n    Size  : {} x {} tiles",
        seed,
        map_name,
        grid.len(),
        grid[0].len()
    );
    AreaGenerationOutput {
        oob_polygons,
        width: grid.len() as u32,
        height: grid[0].len() as u32,
        walkable_x,
        walkable_y,
        player_spawn_position,
    }
}

fn find_oob_polygons(grid: &mut Grid) -> Vec<Shape> {
    // Find a first point on the map contour
    let mut oob_polygons = Vec::new();
    let mut current_pos = (0, (grid[0].len() / 2) as i32);
    while grid[current_pos.0 as usize][current_pos.1 as usize].tile_type != TileType::Floor {
        current_pos.0 += 1;
    }
    // Take a step
    current_pos.0 -= 1;
    // Generate polygone of the outside of the map
    oob_polygons.push(Shape {
        points: find_oob_polygone(current_pos, grid, (0, 1)),
        inner_if_true: false,
    });
    // find inside map polygones
    // scan the grid and search for tiles that are not floor but next to floor, and not already scanned
    'outer: loop {
        for x in 1..grid.len() - 1 {
            for y in 1..grid[0].len() - 1 {
                if !grid[x][y].scanned
                    && grid[x][y].tile_type != TileType::Floor
                    && (grid[x + 1][y].tile_type == TileType::Floor
                        || grid[x - 1][y].tile_type == TileType::Floor
                        || grid[x][y + 1].tile_type == TileType::Floor
                        || grid[x][y - 1].tile_type == TileType::Floor)
                {
                    oob_polygons.push(Shape {
                        points: find_oob_polygone((x as i32, y as i32), grid, (0, -1)),
                        inner_if_true: true,
                    });
                    continue 'outer;
                }
            }
        }
        break;
    }

    oob_polygons
}

fn find_oob_polygone(
    start_point: (i32, i32),
    grid: &mut Grid,
    start_dir: (i32, i32),
) -> Vec<(f32, f32)> {
    let mut tile_polygone = Vec::new();
    let mut px_polygone: Vec<(f32, f32)> = Vec::new();
    let mut current_pos = start_point;

    let mut dir = start_dir;
    let mut next_dir = dir;
    // continue tracing until we come back where to the first corner
    while !tile_polygone.contains(&current_pos) {
        // if current dir is down
        if dir == (0, 1) {
            // right is floor
            if (grid[current_pos.0 as usize + 1][current_pos.1 as usize]).tile_type
                == TileType::Floor
            {
                // down is floor
                if grid[current_pos.0 as usize][current_pos.1 as usize + 1].tile_type
                    == TileType::Floor
                {
                    //found corner
                    tile_polygone.push(current_pos);
                    //keep bottom right point
                    px_polygone.push((
                        (current_pos.0 * TILE_SIZE) as f32,
                        (current_pos.1 * TILE_SIZE) as f32,
                    ));
                    next_dir = (-1, 0);
                } else {
                    next_dir = (0, 1);
                }
            } else {
                // found corner
                tile_polygone.push(current_pos);
                // keep up right point
                px_polygone.push((
                    (current_pos.0 * TILE_SIZE) as f32,
                    ((current_pos.1 * TILE_SIZE) - TILE_SIZE) as f32,
                ));
                next_dir = (1, 0);
            }
        }
        // if curent dir is left
        if dir == (-1, 0) {
            // bottom is floor
            if (grid[current_pos.0 as usize][current_pos.1 as usize + 1]).tile_type
                == TileType::Floor
            {
                // left is floor
                if grid[current_pos.0 as usize - 1][current_pos.1 as usize].tile_type
                    == TileType::Floor
                {
                    //found corner
                    tile_polygone.push(current_pos);
                    //keep bottom left
                    px_polygone.push((
                        ((current_pos.0 * TILE_SIZE) - TILE_SIZE) as f32,
                        (current_pos.1 * TILE_SIZE) as f32,
                    ));
                    next_dir = (0, -1);
                } else {
                    next_dir = (-1, 0);
                }
            } else {
                tile_polygone.push(current_pos);
                //keep bottom right
                px_polygone.push((
                    (current_pos.0 * TILE_SIZE) as f32,
                    (current_pos.1 * TILE_SIZE) as f32,
                ));
                next_dir = (0, 1);
            }
        }
        // if curent dir is right
        if dir == (1, 0) {
            // up is floor
            if (grid[current_pos.0 as usize][current_pos.1 as usize - 1]).tile_type
                == TileType::Floor
            {
                // right is floor
                if grid[current_pos.0 as usize + 1][current_pos.1 as usize].tile_type
                    == TileType::Floor
                {
                    //found corner
                    tile_polygone.push(current_pos);
                    // keep up right point
                    px_polygone.push((
                        (current_pos.0 * TILE_SIZE) as f32,
                        ((current_pos.1 * TILE_SIZE) - TILE_SIZE) as f32,
                    ));
                    next_dir = (0, 1);
                } else {
                    next_dir = (1, 0);
                }
            } else {
                tile_polygone.push(current_pos);
                // keep up left point
                px_polygone.push((
                    ((current_pos.0 * TILE_SIZE) - TILE_SIZE) as f32,
                    ((current_pos.1 * TILE_SIZE) - TILE_SIZE) as f32,
                ));
                next_dir = (0, -1);
            }
        }
        // if current dir is up
        if dir == (0, -1) {
            // left is floor
            if (grid[current_pos.0 as usize - 1][current_pos.1 as usize]).tile_type
                == TileType::Floor
            {
                // up is floor
                if grid[current_pos.0 as usize][current_pos.1 as usize - 1].tile_type
                    == TileType::Floor
                {
                    //found corner
                    tile_polygone.push(current_pos);
                    // keep up left point
                    px_polygone.push((
                        ((current_pos.0 * TILE_SIZE) - TILE_SIZE) as f32,
                        ((current_pos.1 * TILE_SIZE) - TILE_SIZE) as f32,
                    ));
                    next_dir = (1, 0);
                } else {
                    next_dir = (0, -1);
                }
            } else {
                // found corner
                tile_polygone.push(current_pos);
                // keep bottom left point
                px_polygone.push((
                    ((current_pos.0 * TILE_SIZE) - TILE_SIZE) as f32,
                    (current_pos.1 * TILE_SIZE) as f32,
                ));
                next_dir = (-1, 0);
            }
        }
        dir = next_dir;
        // flag current point to avoid scanning this polygon again later
        grid[current_pos.0 as usize][current_pos.1 as usize].scanned = true;
        // move to next point
        current_pos.0 += dir.0;
        current_pos.1 += dir.1;
    }

    // for debuging only
    for point in tile_polygone {
        grid[point.0 as usize][point.1 as usize].tile_type = TileType::Angle;
    }

    px_polygone
}

fn generate_map(seed: u64, map: Map) -> (Grid, (i32, i32)) {
    // The rng instance is created from the seed
    let mut rng: ChaCha8Rng = ChaCha8Rng::seed_from_u64(seed);

    let oob_tiletype = map.oob_type;

    // Initialize map grid from initial biome and oob tile type
    let mut grid: Grid = init_grid(750, 750, oob_tiletype);

    // genrate walkable paths based on a random selection of possible biomes
    let mut center = map.generation_init_center;
    let map_start = center;

    for i in 0..map.biomes.len() {
        center =
            generate_walkable_layout(&mut grid, &map.biomes[i], &mut rng, oob_tiletype, center);
    }

    // remove small clusters of oob tiles
    remove_small_cluster(&mut grid, oob_tiletype, 4, false, true);
    remove_small_cluster(&mut grid, oob_tiletype, 4, true, false);
    remove_small_cluster(&mut grid, oob_tiletype, 4, false, true);

    // add Start of map, first center and last center
    draw_rectangle(&mut grid, TileType::Start, (2, 2), map_start);
    draw_rectangle(&mut grid, TileType::Boss, (2, 2), center);

    // resize_grid to it's minimum size
    resize_grid(&mut grid, 4);

    let mut start_after_resize = (0, 0);
    'outer: for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y].tile_type == TileType::Start {
                start_after_resize = (x as i32, y as i32);
                break 'outer;
            }
        }
    }

    // // print grid
    // render_grid(&grid, map.name.clone());
    (
        grid,
        (
            (start_after_resize.0 * TILE_SIZE) - (TILE_SIZE / 2),
            (start_after_resize.1 * TILE_SIZE) - (TILE_SIZE / 2),
        ),
    )
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
                scanned: false,
            })
        }
        grid.push(row)
    }
    grid
}

fn render_grid(grid: &Grid, file_name: String, show_outline: bool) {
    // Construct a new RGB ImageBuffer with the specified width and height.
    let width = grid.len();
    let height = grid[0].len();

    // Construct a new by repeated calls to the supplied closure.
    let mut img = ImageBuffer::new(width.try_into().unwrap(), height.try_into().unwrap());

    for (i, x) in grid.iter().enumerate() {
        for (j, y) in x.iter().enumerate() {
            if y.scanned && show_outline {
                img.put_pixel(
                    i.try_into().unwrap(),
                    j.try_into().unwrap(),
                    image::Rgb([252u8, 40u8, 40u8]),
                )
            } else {
                img.put_pixel(
                    i.try_into().unwrap(),
                    j.try_into().unwrap(),
                    match y.tile_type {
                        TileType::Boss => image::Rgb([0u8, 0u8, 0u8]),
                        TileType::Floor => image::Rgb([230u8, 213u8, 168u8]),
                        TileType::Wall => image::Rgb([122u8, 97u8, 31u8]),
                        TileType::Start => image::Rgb([182u8, 51u8, 214u8]),
                        TileType::Angle => image::Rgb([182u8, 51u8, 214u8]),
                        TileType::Event => image::Rgb([181u8, 181u8, 181u8]),
                        TileType::Water => image::Rgb([51u8, 114u8, 214u8]),
                        TileType::Forest => image::Rgb([42u8, 117u8, 14u8]),
                    },
                )
            };
        }
    }
    img.save("output/".to_string() + &file_name + ".png")
        .unwrap();
}
