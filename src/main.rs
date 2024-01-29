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

struct Biome {
    name: String,
    // odds: f32,
    oob_type: TileType,
    rng_range_multiplicator_rectangle_size: (f32, f32),
    rng_range_number_of_direction_changes: (i32, i32),
    rng_range_direction_repeat: (i32, i32),
    allowed_directions: Vec<(i32, i32)>,
    generation_init_center: (i32, i32),
    generation_area_size: (i32, i32),
}
#[derive(PartialEq, Eq)]
struct Tile {
    tile_type: TileType,
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

// const DIRECTIONS_INTDOOR: [(i32, i32); 4] = [
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
    let mut biomes = vec![
        Biome {
            name: String::from("Desert"),
            // odds: 1.0,
            oob_type: TileType::Wall,
            rng_range_multiplicator_rectangle_size: (0.1, 0.15),
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
            generation_init_center: (500, 500),
            generation_area_size: (1000, 1000),
        },
        Biome {
            name: String::from("Forest"),
            // odds: 1.0,
            oob_type: TileType::Forest,
            rng_range_multiplicator_rectangle_size: (0.02, 0.06),
            rng_range_number_of_direction_changes: (20, 30),
            rng_range_direction_repeat: (5, 10),
            allowed_directions: vec![(0, -1), (0, 1), (-1, 0), (1, 0)],
            generation_init_center: (400, 400),
            generation_area_size: (800, 800),
        },
        Biome {
            name: String::from("Island"),
            // odds: 1.0,
            oob_type: TileType::Water,
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
            generation_init_center: (400, 400),
            generation_area_size: (800, 800),
        },
        Biome {
            name: String::from("Quarry"),
            // odds: 1.0,
            oob_type: TileType::Wall,
            rng_range_multiplicator_rectangle_size: (0.01, 0.020),
            rng_range_number_of_direction_changes: (30, 40),
            rng_range_direction_repeat: (10, 15),
            allowed_directions: vec![(1, -1), (1, 1), (-1, 1), (-1, -1)],
            generation_init_center: (400, 400),
            generation_area_size: (800, 800),
        },
        Biome {
            name: String::from("Ledge"),
            // odds: 1.0,
            oob_type: TileType::Wall,
            rng_range_multiplicator_rectangle_size: (0.01, 0.020),
            rng_range_number_of_direction_changes: (20, 30),
            rng_range_direction_repeat: (10, 15),
            allowed_directions: vec![(1, -1), (1, 1), (-1, 1)],
            generation_init_center: (100, 100),
            generation_area_size: (800, 800),
        },
        Biome {
            name: String::from("Superu"),
            // odds: 1.0,
            oob_type: TileType::Wall,
            rng_range_multiplicator_rectangle_size: (0.01, 0.020),
            rng_range_number_of_direction_changes: (1, 2),
            rng_range_direction_repeat: (40, 65),
            allowed_directions: vec![(0, -1), (1, 0)],
            generation_init_center: (400, 400),
            generation_area_size: (800, 800),
        },
    ];

    // let mut handlers = Vec::new();
    // while let Some(biome) = biomes.pop() {
    //     handlers.push(thread::spawn(move || {
    //         generate_map(seed, &biome);
    //     }));
    // }
    // for handler in handlers {
    //     handler.join().unwrap();
    // }
    for biome in biomes {
        generate_map(seed, &biome);
    }
}

fn generate_map(seed: u64, biome: &Biome) {
    println!("-----For Biome {}", biome.name);
    let mut rng: ChaCha8Rng = ChaCha8Rng::seed_from_u64(seed);

    // let test: f32 = f32::sin(4.0);
    // println!("Sin is {}", test);

    // Init grid
    let mut grid: Grid = init_grid(
        biome.generation_area_size.0,
        biome.generation_area_size.1,
        biome.oob_type,
    );

    // remplir de watter ou wall
    // Pause une grosse forme, square only
    // partir d'un point qui touche le side ( ou un des neighbor est de l'eau ou du wall)
    // On joue sur le nombre, la tailler et le type de formes most likely, les range sont definis par les biomes.
    // Definir des suite de dirrection predefinies, genre left right up down, ou full droite
    generate_walkable_layout(&mut grid, biome, &mut rng);
    // add_tile(
    //     &mut grid,
    //     rng.gen_range(0..width) as usize,
    //     rng.gen_range(0..height) as usize,
    //     TileType::Start,
    // );
    // add_tile(
    //     &mut grid,
    //     rng.gen_range(0..width) as usize,
    //     rng.gen_range(0..height) as usize,
    //     TileType::Boss,
    // );
    // Remove orphan non walkable tiles, based on a threashold of x tiles
    // TODO

    // print grid
    render_grid(&grid, biome.name.clone());
}

fn generate_walkable_layout(grid: &mut Grid, biome: &Biome, rng: &mut ChaCha8Rng) {
    // input on definie si c'est de l'eau ou du mur en dehors
    // on fait des appels successifs a draw rectangle
    // definir l'impact des weight sur la generation ici
    // on posse une tile initiale, puis on choisis un edge a un non walkable, on roll la taille du rectangle un offset du centre, et on draw le rectangle
    let gridsize = grid.len() as i32;
    let init_center = biome.generation_init_center;
    // on fait le premier rectengle au centre
    draw_rectangle(
        grid,
        TileType::Floor,
        (
            (gridsize as f32
                * rng.gen_range(
                    biome.rng_range_multiplicator_rectangle_size.0
                        ..biome.rng_range_multiplicator_rectangle_size.1,
                ))
            .round() as i32,
            (gridsize as f32
                * rng.gen_range(
                    biome.rng_range_multiplicator_rectangle_size.0
                        ..biome.rng_range_multiplicator_rectangle_size.1,
                ))
            .round() as i32,
        ),
        init_center,
    );

    let mut center: (i32, i32) = init_center;
    for _ in 0..rng.gen_range(
        biome.rng_range_number_of_direction_changes.0
            ..biome.rng_range_number_of_direction_changes.1,
    ) {
        // on cherche un nouveau centre , depuis la position du precedent, avec une range maximum et un angle de recherche
        // on pause des point au hazard dans la zone possible jusq'a tomber sur du sol
        // quand on touche le sol, on cherge un edge sur une dirrection ( direction est haut bas gauche ou droite, on pouras donner des poids a chaque via le biome)
        let iterrations: i32 =
            rng.gen_range(biome.rng_range_direction_repeat.0..biome.rng_range_direction_repeat.1);

        for _ in 0..iterrations {
            let direction: (i32, i32) =
                biome.allowed_directions[rng.gen_range(0..biome.allowed_directions.len())];

            center = find_point_on_edge(grid, center, biome, direction);
            draw_rectangle(
                grid,
                TileType::Floor,
                (
                    (gridsize as f32
                        * rng.gen_range(
                            biome.rng_range_multiplicator_rectangle_size.0
                                ..biome.rng_range_multiplicator_rectangle_size.1,
                        ))
                    .round() as i32,
                    (gridsize as f32
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
}

fn find_point_on_edge(
    grid: &Grid,
    previous_center: (i32, i32),
    biome: &Biome,
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
            != biome.oob_type
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
                    TileType::Boss => image::Rgb([181u8, 181u8, 181u8]),
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
