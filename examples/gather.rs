fn main() -> anyhow::Result<()> {
  let mut universe = fujitsu_mpi::initialize()?;
  println!("Initialized");
  let mut world = universe.world();
  let size = world.size()?;
  let rank = world.rank()?;
  let root: usize = 0;
  println!("size = {}, rank = {}", size, rank);
  let mut send_buff: Vec<u64> = if rank == 0 {
    vec![1, 2, 3]
  } else {
    vec![4, 5, 6]
  };
  let mut recv_buff: Vec<u64> = if rank == root {
    vec![0; 6]
  } else {
    vec![]
  };
  println!("send_buff = {:?}, recv_buff = {:?}", send_buff, recv_buff);
  world.gather(
    &mut send_buff,
    &mut recv_buff,
    0,
  )?;
  println!("send_buff = {:?}, recv_buff = {:?}", send_buff, recv_buff);
  Ok(())
}
