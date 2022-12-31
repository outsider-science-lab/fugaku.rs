fn main() -> anyhow::Result<()> {
  let mut universe = fujitsu_mpi::initialize()?;
  println!("Initialized");
  let mut world = universe.world();
  let size = world.size()?;
  let rank = world.rank()?;
  let root: usize = 0;
  println!("size = {}, rank = {}", size, rank);
  let mut send_buff: Vec<u64> = if rank == root {
    vec![1, 2, 3, 4, 5, 6]
  } else {
    vec![]
  };
  let mut recv_buff: Vec<u64> = vec![0; 3];
  println!("SendBuf: {:?} / RecvBuf {:?}", send_buff, recv_buff);
  world.scatter(
    &mut send_buff,
    &mut recv_buff,
    0,
  )?;
  println!("SendBuf: {:?} / RecvBuf {:?}", send_buff, recv_buff);
  Ok(())
}
