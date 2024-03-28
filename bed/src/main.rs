use mpi_async as mpi;

fn main() -> anyhow::Result<()> {
  let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;
  rt.block_on(async {
    let mut universe = mpi::initialize()?;
    println!("Initialized");
    let mut world = universe.world();
    let rank = world.rank()?;
    println!("size = {}, rank = {}", world.size()?, rank);
    if rank == 0 {
      let mut send_buff: [u64; 3] = [1, 2, 3];
      println!("[Send/Before] send_buff = {:?}", send_buff);
      tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
      let mem = world.send(&mut send_buff, 1, 0)?.await?;
      println!("[Send/After] send_buff = {:?}", mem);
    } else {
      let mut recv_buff: [u64; 3] = [0, 0, 0];
      println!("[Recv/Before] recv_buff = {:?}", recv_buff);
      let mem = world.recv(&mut recv_buff, 0, 0)?.await?;
      println!("[Recv/After] recv_buff = {:?}", mem);
    };
    Ok(())
  })
}
