
fn main() -> anyhow::Result<()> {
  let universe = fujitsu_mpi::initialize()?;
  println!("Hello, MPI!");
  Ok(())
}
