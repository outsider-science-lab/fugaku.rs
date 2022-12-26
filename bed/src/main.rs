fn main() -> anyhow::Result<()> {
  let universe = fujitsu_mpi::initialize()?;
  println!("Initialized");
  let world = universe.world();
  let rank = world.rank()?;
  println!("Size: {}, rank: {}, cores={}", world.size()?, rank, num_cpus::get());
  if rank == 0 {
    let mut array: [u64; 3] = [1, 2, 3];
    world.send(&mut array, 1, 0)?;
    println!("Sent.");
  } else {
    let mut array: [u64; 3] = [0, 0, 0];
    world.recv(&mut array, 0, 0)?;
    println!("Recv: {:?}", array);
  };
  Ok(())
}
