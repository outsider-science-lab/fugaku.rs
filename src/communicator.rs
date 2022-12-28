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

  pub fn size(&mut self) -> anyhow::Result<usize> {
    let mut size: i32 = 0;
    let r = unsafe {
      ffi::MPI_Comm_size(self.comm, &mut size) as u32
    };
    match r {
      MPI_SUCCESS => Ok(size as usize),
      _ => Err(anyhow::Error::msg(format!("[MPI_Comm_size] Unknown code: {}", r))),
    }
  }

  pub fn rank(&mut self) -> anyhow::Result<usize> {
    let mut rank: i32 = 0;
    let r = unsafe {
      ffi::MPI_Comm_rank(self.comm, &mut rank) as u32
    };
    match r {
      MPI_SUCCESS => Ok(rank as usize),
      _ => Err(anyhow::Error::msg(format!("[MPI_Comm_size] Unknown code: {}", r))),
    }
  }

  pub fn send<T>(&mut self, buff: &mut [T], to: usize, tag: i32) -> anyhow::Result<()>
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

  pub fn recv<T>(&mut self, buff: &mut [T], from: usize, tag: i32) -> anyhow::Result<()>
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

  pub fn broadcast<T>(&mut self, buff: &mut [T], root: usize) -> anyhow::Result<()>
    where T: data_types::DataType
  {
    let r = unsafe {
      ffi::MPI_Bcast(
        buff.as_mut_ptr() as *mut std::os::raw::c_void, 
        buff.len() as i32,
        T::mpi_data_type(),
        root as i32,
        self.comm
      ) as u32
    };
    match r {
      MPI_SUCCESS => Ok(()),
      _ => Err(anyhow::Error::msg(format!("[MPI_Bcast] Unknown code: {}", r))),
    }
  }

  pub fn scatter<T>(&mut self, send_buff: &mut [T], recv_buff: &mut [T], root: usize) -> anyhow::Result<()>
    where T: data_types::DataType
  {
    let r = unsafe {
      ffi::MPI_Scatter(
        send_buff.as_mut_ptr() as *mut std::os::raw::c_void,
        send_buff.len() as i32,
        T::mpi_data_type(),
        recv_buff.as_mut_ptr() as *mut std::os::raw::c_void,
        recv_buff.len() as i32,
        T::mpi_data_type(),
        root as i32,
        self.comm,
      ) as u32
    };
    match r {
      MPI_SUCCESS => Ok(()),
      _ => Err(anyhow::Error::msg(format!("[MPI_Bcast] Unknown code: {}", r))),
    }
  }
}
