fn main() -> anyhow::Result<()> {
  let mut universe = mpi::initialize()?;
  println!("Initialized");
  let mut world = universe.world();
  let size = world.size()?;
  let rank = world.rank()?;
  println!("size = {}, rank = {}", size, rank);
  if rank == 0 {
    let mut send_buff: Vec<u32> = vec![1, 2, 3, 4, 5,];
    let mut recv_buff: Vec<f64> = vec![0.0; 3];

    println!("send_buff = {:?}, recv_buff = {:?}", send_buff, recv_buff);
    world.send_recv(
      &mut send_buff,
      1,
      0,
      &mut recv_buff,
      1,
      0,
    )?;
    println!("send_buff = {:?}, recv_buff = {:?}", send_buff, recv_buff);
  } else {
    let mut send_buff: Vec<f64> = vec![1.0, 2.0, 3.0];
    let mut recv_buff: Vec<u32> = vec![0; 5];

    println!("send_buff = {:?}, recv_buff = {:?}", send_buff, recv_buff);
    world.send_recv(
      &mut send_buff,
      0,
      0,
      &mut recv_buff,
      0,
      0,
    )?;
    println!("send_buff = {:?}, recv_buff = {:?}", send_buff, recv_buff);
  }
  Ok(())
}
