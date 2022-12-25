use fujitsu_mpi_sys as ffi;
use ffi::{
  MPI_Comm,
  MPI_SUCCESS,
};

pub struct Communicator {
  comm: MPI_Comm,
}

impl Communicator {
  pub fn new(comm: MPI_Comm,) -> Self {
    Self {
      comm,
    }
  }

  pub fn size(&self) -> anyhow::Result<usize> {
    let mut size: i32 = 0;
    let r = unsafe {
      ffi::MPI_Comm_size(self.comm, &mut size) as u32
    };
    match r {
      MPI_SUCCESS => Ok(size as usize),
      _ => Err(anyhow::Error::msg(format!("[MPI_Comm_size] Unknown code: {}", r))),
    }
  }

  pub fn rank(&self) -> anyhow::Result<usize> {
    let mut rank: i32 = 0;
    let r = unsafe {
      ffi::MPI_Comm_rank(self.comm, &mut rank) as u32
    };
    match r {
      MPI_SUCCESS => Ok(rank as usize),
      _ => Err(anyhow::Error::msg(format!("[MPI_Comm_size] Unknown code: {}", r))),
    }
  }
}
