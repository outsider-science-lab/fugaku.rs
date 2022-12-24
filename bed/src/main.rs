
fn main() -> anyhow::Result<()> {
  let universe = fujitsu_mpi::initialize()?;
  println!("Initialized");
  let world = universe.world();
  println!("Size: {}, rank: {}", world.size()?, world.rank()?);
  Ok(())
}
