use fujitsu_mpi_sys as ffi;
use ffi::{
  MPI_Comm,
  MPI_SUCCESS,
};
use crate::mpi::data_types;

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

  pub fn send<T>(&self, buff: &mut [T], to: usize, tag: i32) -> anyhow::Result<()>
    where T: data_types::DataType
  {
    let r = unsafe {
      ffi::MPI_Send(
        buff.as_mut_ptr() as *mut std::os::raw::c_void,
        buff.len() as i32,
        T::mpi_data_type(),
        to as i32,
        tag,
        self.comm,
      ) as u32
    };
    match r {
      MPI_SUCCESS => Ok(()),
      _ => Err(anyhow::Error::msg(format!("[MPI_Send] Unknown code: {}", r))),
    }
  }

  pub fn recv<T>(&self, buff: &mut [T], from: usize, tag: i32) -> anyhow::Result<()>
    where T: data_types::DataType
  {
    let mut status: ffi::MPI_Status = unsafe {
      std::mem::MaybeUninit::<ffi::MPI_Status>::zeroed().assume_init()
    };
    let r = unsafe {
      ffi::MPI_Recv(
        buff.as_mut_ptr() as *mut std::os::raw::c_void,
        buff.len() as i32,
        T::mpi_data_type(),
        from as i32,
        tag,
        self.comm,
        &mut status,
      ) as u32
    };
    match r {
      MPI_SUCCESS => Ok(()),
      _ => Err(anyhow::Error::msg(format!("[MPI_Recv] Unknown code: {}", r))),
    }
  }
}
