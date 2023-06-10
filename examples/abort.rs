fn main() -> anyhow::Result<()> {
  let request = mpi::ThreadLevel::Multiple;
  let mut universe = mpi::initialize_thread(request)?;
  println!("Initialized");
  let mut world = universe.world();
  let size = world.size()?;
  let rank = world.rank()?;
  println!("size = {}, rank = {}", size, rank);
  world.abort(-1)?;
  Ok(())
}
