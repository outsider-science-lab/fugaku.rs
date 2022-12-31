fn main() -> anyhow::Result<()> {
  let mut universe = fujitsu_mpi::initialize()?;
  println!("[MPI] Initialized");
  let mut world = universe.world();
  println!("[MPI] size = {}, rank = {}", world.size()?, world.rank()?);
  Ok(())
}
