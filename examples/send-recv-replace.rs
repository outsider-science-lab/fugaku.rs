fn main() -> anyhow::Result<()> {
  let mut universe = fujitsu_mpi::initialize()?;
  println!("Initialized");
  let mut world = universe.world();
  let size = world.size()?;
  let rank = world.rank()?;
  println!("size = {}, rank = {}", size, rank);
  if rank == 0 {
    let mut buff: Vec<u32> = vec![1, 2, 3, 4, 5,];

    println!("Buf: {:?}", buff);
    world.send_recv_replace(
      &mut buff,
      1,
      0,
      1,
      0,
    )?;
    println!("Buf: {:?}", buff);
  } else {
    let mut buff: Vec<u32> = vec![6, 7, 8, 9, 10,];

    println!("Buf: {:?}", buff);
    world.send_recv_replace(
      &mut buff,
      0,
      0,
      0,
      0,
    )?;
    println!("Buf: {:?}", buff);
  }
  Ok(())
}
