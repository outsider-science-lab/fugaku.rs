fn main() -> anyhow::Result<()> {
  let mut universe = fujitsu_mpi::initialize_thread(fujitsu_mpi::ThreadLevel::Multiple)?;
  println!("Initialized");
  let mut world = universe.world();
  let size = world.size()?;
  let rank = world.rank()?;
  println!("size = {}, rank = {}", size, rank);
  Ok(())
}
