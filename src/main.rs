mod create_world;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    create_world::create_world_controller().await?;
    Ok(())
}
