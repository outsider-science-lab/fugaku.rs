use mpi_sys as ffi;
use ffi::{
  MPI_Comm,
  MPI_Request,
  MPI_SUCCESS,
};

use crate::request::Request;
use mpi_common::{DataType, malloc};

pub struct Communicator {
  comm: MPI_Comm,
}

impl Communicator {
  pub fn new(
    comm: MPI_Comm,
  ) -> Self {
    Self {
      comm,
    }
  }

  pub fn size(&self) -> anyhow::Result<usize> {
    mpi_common::communicator::size(self.comm)
  }

  pub fn rank(&self) -> anyhow::Result<usize> {
    mpi_common::communicator::rank(self.comm)
  }

  pub fn abort(&self, error_code: i32) -> anyhow::Result<()> {
    mpi_common::communicator::abort(self.comm, error_code)
  }

  pub fn send<'v, T>(&mut self, buff: &'v [T], peer: usize, tag: i32) -> anyhow::Result<Request<'v, [T]>>
    where T: DataType,
  {
    let mut req: MPI_Request = malloc();
    let r = unsafe {
      ffi::MPI_Isend(
        buff.as_ptr() as *mut std::os::raw::c_void,
        buff.len() as i32,
        T::to_ffi(),
        peer as i32,
        tag,
        self.comm,
        &mut req,
      ) as u32
    };
    match r {
      MPI_SUCCESS => Ok(Request::<'v, [T]>::new(buff, req)),
      _ => Err(anyhow::Error::msg(format!("[MPI_Send] Unknown code: {}", r))),
    }
  }

  pub fn recv<'v, T>(&mut self, buff: &'v mut [T], peer: usize, tag: i32) -> anyhow::Result<Request<'v, [T]>>
    where T: DataType,
  {
    let mut req: MPI_Request = malloc();
    let r = unsafe {
      ffi::MPI_Irecv(
        buff.as_mut_ptr() as *mut std::os::raw::c_void,
        buff.len() as i32,
        T::to_ffi(),
        peer as i32,
        tag,
        self.comm,
        &mut req,
      ) as u32
    };
    match r {
      MPI_SUCCESS => Ok(Request::<'v, [T]>::new(buff, req)),
      _ => Err(anyhow::Error::msg(format!("[MPI_Send] Unknown code: {}", r))),
    }
  }
}
