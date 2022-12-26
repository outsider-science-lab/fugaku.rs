fn main() -> anyhow::Result<()> {
  let mut universe = fujitsu_mpi::initialize()?;
  println!("Initialized");
  let mut world = universe.world();
  println!("Size: {}, rank: {}", world.size()?, world.rank()?);
  Ok(())
}
