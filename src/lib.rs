pub fn add(left: usize, right: usize) -> usize {
  left + right
}

pub struct Universe {

}

impl Drop for Universe {
  fn drop(&mut self) {
    fujitsu_mpi_sys::finalize().expect("Failed to finalize MPI");
  }
}

pub fn initialize() -> anyhow::Result<Universe> {
  fujitsu_mpi_sys::initialize().map(|_| { Universe{} })
}
