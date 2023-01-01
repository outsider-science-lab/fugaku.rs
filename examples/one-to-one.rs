fn main() -> anyhow::Result<()> {
  let mut universe = fujitsu_mpi::initialize()?;
  println!("Initialized");
  let mut world = universe.world();
  let rank = world.rank()?;
  println!("size = {}, rank = {}", world.size()?, rank);
  if rank == 0 {
    let mut send_buff: [u64; 3] = [1, 2, 3];
    world.send(&mut send_buff, 1, 0)?;
    println!("[Send] send_buff = {:?}", send_buff);
  } else {
    let mut recv_buff: [u64; 3] = [0, 0, 0];
    world.recv(&mut recv_buff, 0, 0)?;
    println!("[Recv] recv_buff = {:?}", recv_buff);
  };
  Ok(())
}
