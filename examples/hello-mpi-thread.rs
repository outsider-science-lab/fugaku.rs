fn main() -> anyhow::Result<()> {
  let request = fujitsu_mpi::ThreadLevel::Multiple;
  let mut universe = fujitsu_mpi::initialize_thread(request)?;
  println!("Initialized");
  let mut world = universe.world();
  let size = world.size()?;
  let rank = world.rank()?;
  println!("size = {}, rank = {}", size, rank);
  println!(
    "required = {:?}, provided = {:?}, request fulfilled = {}",
    request,
    universe.level(),
    request <= universe.level(),
  );
  Ok(())
}
