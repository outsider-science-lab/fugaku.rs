fn main() -> anyhow::Result<()> {
  let mut universe = fujitsu_mpi::initialize()?;
  println!("Initialized");
  let mut world = universe.world();
  let rank = world.rank()?;
  println!("Size: {}, rank: {}, cores={}", world.size()?, rank, num_cpus::get());
  let mut array: [u64; 3] = [1, 2, 3];
  world.broadcast(&mut array, 0)?;
  println!("Recv: {:?}", array);
Ok(())
}
