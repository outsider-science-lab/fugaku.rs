pub fn add(left: usize, right: usize) -> usize {
  left + right
}

pub struct Universe {

}

impl Universe {
  pub fn world(&self) -> Communicator {
    Communicator {
      comm: fujitsu_mpi_sys::comm_world(),
    }
  }
}

pub struct Communicator {
  comm: fujitsu_mpi_sys::MPI_Comm,
}

impl Communicator {
  pub fn size(&self) -> anyhow::Result<usize> {
    fujitsu_mpi_sys::comm_size(self.comm)
  }
  pub fn rank(&self) -> anyhow::Result<usize> {
    fujitsu_mpi_sys::comm_rank(self.comm)
  }
}

impl Drop for Universe {
  fn drop(&mut self) {
    fujitsu_mpi_sys::finalize().expect("Failed to finalize MPI");
  }
}

pub fn initialize() -> anyhow::Result<Universe> {
  fujitsu_mpi_sys::initialize().map(|_| { Universe{} })
}
