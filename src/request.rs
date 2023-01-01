// Doc:
// - https://rookiehpc.github.io/mpi/docs/mpi_request/index.html

use fujitsu_mpi_sys as ffi;
pub struct Request {
  req: ffi::MPI_Request,
}

impl Request {
  pub(crate) fn new(req: ffi::MPI_Request) -> Self {
    Self {
      req,
    }
  }
  pub(crate) fn inner(&mut self) -> &mut ffi::MPI_Request {
    &mut self.req
  }
}
