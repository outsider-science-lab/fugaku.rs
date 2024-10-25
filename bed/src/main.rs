use mpi;

const SIZE: usize = 10;

fn main() -> anyhow::Result<()> {
  let mut universe = mpi::initialize()?;
  println!("Initialized");
  let mut world = universe.world();
  let rank = world.rank()?;
  println!("size = {}, rank = {}", world.size()?, rank);
  {
    let send_buff = [rank as i32; SIZE];
    let mut recv_buff = [-1; SIZE];

    println!("[Send/Before] send_buff = {:?}", send_buff);
    println!("[Send/Before] recv_buff = {:?}", recv_buff);
    world.all_to_all(
      &send_buff,
      &mut recv_buff,
    )?;
    println!("[Send/After] recv_buff = {:?}", recv_buff);
  }
  Ok(())
}
