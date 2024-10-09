use mpi;

fn main() -> anyhow::Result<()> {
  let mut universe = mpi::initialize()?;
  println!("Initialized");
  let mut world = universe.world();
  let rank = world.rank()?;
  println!("size = {}, rank = {}", world.size()?, rank);
  if rank == 0 {
    let send_buff: [u64; 2] = [1, 2];
    let mut recv_buff: [u64; 2] = [0, 0];

    println!("[Send/Before] send_buff = {:?}", send_buff);
    println!("[Send/Before] recv_buff = {:?}", recv_buff);
    world.all_to_all(
      &send_buff,
      &mut recv_buff,
    )?;
    println!("[Send/After] send_buff = {:?}", send_buff);
    println!("[Send/After] recv_buff = {:?}", recv_buff);
  } else {
    let send_buff: [u64; 2] = [3, 4];
    let mut recv_buff: [u64; 2] = [0, 0];

    println!("[Send/Before] send_buff = {:?}", send_buff);
    println!("[Send/Before] recv_buff = {:?}", recv_buff);
    world.all_to_all(
      &send_buff,
      &mut recv_buff,
    )?;
    println!("[Send/After] send_buff = {:?}", send_buff);
    println!("[Send/After] recv_buff = {:?}", recv_buff);
  };
  Ok(())
}
