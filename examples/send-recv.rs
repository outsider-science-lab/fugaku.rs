fn main() -> anyhow::Result<()> {
  let mut universe = fujitsu_mpi::initialize()?;
  println!("[MPI] Initialized");
  let mut world = universe.world();
  let rank = world.rank()?;
  println!("[MPI] size = {}, rank = {}", world.size()?, rank);
  if rank == 0 {
    let mut array: [u64; 3] = [1, 2, 3];
    world.send(&mut array, 1, 0)?;
    println!("[MPI] Sent.");
  } else {
    let mut array: [u64; 3] = [0, 0, 0];
    world.recv(&mut array, 0, 0)?;
    println!("[MPI] Recv: {:?}", array);
  };
  Ok(())
}
