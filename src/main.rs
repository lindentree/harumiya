mod create_world;

fn main() {
    println!("Hello, world!");

    create_world::create_world_controller().await;
}
