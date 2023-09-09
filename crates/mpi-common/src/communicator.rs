use mpi_sys as ffi;
use ffi::{
  MPI_Comm,
  MPI_SUCCESS,
};

pub fn size(comm: MPI_Comm) -> anyhow::Result<usize> {
  let mut size: i32 = 0;
  let r = unsafe {
    ffi::MPI_Comm_size(comm, &mut size) as u32
  };
  match r {
    MPI_SUCCESS => Ok(size as usize),
    _ => Err(anyhow::Error::msg(format!("[MPI_Comm_size] Unknown code: {}", r))),
  }
}

pub fn rank(comm: MPI_Comm) -> anyhow::Result<usize> {
  let mut rank: i32 = 0;
  let r = unsafe {
    ffi::MPI_Comm_rank(comm, &mut rank) as u32
  };
  match r {
    MPI_SUCCESS => Ok(rank as usize),
    _ => Err(anyhow::Error::msg(format!("[MPI_Comm_size] Unknown code: {}", r))),
  }
}

pub fn abort(comm: MPI_Comm, error_code: i32) -> anyhow::Result<()> {
  let r = unsafe {
    ffi::MPI_Abort(comm, error_code) as u32
  };
  match r {
    MPI_SUCCESS => Ok(()),
    _ => Err(anyhow::Error::msg(format!("[MPI_Comm_size] Unknown code: {}", r))),
  }
}
