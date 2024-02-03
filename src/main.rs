use area_gen::generate_area;

mod area_gen;

fn main() {
    for i in 0..5 {
        generate_area(i);
    }
}
