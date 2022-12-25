
fn main() -> anyhow::Result<()> {
  let universe = fujitsu_mpi::initialize()?;
  println!("Initialized");
  let world = universe.world();
  println!("Size: {}, rank: {}, cores={}", world.size()?, world.rank()?, num_cpus::get());
  Ok(())
}
