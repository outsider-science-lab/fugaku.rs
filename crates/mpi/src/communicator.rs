// Doc:
// - https://rookiehpc.github.io/mpi/docs/mpi_comm/

use mpi_sys as ffi;
use ffi::{
  MPI_Comm,
  MPI_SUCCESS,
};
use mpi_common::DataType;
use mpi_common::Op;
use mpi_common::malloc;

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

  pub fn abort(&mut self, error_code: i32) -> anyhow::Result<()> {
    let r = unsafe {
      ffi::MPI_Abort(self.comm, error_code) as u32
    };
    match r {
      MPI_SUCCESS => Ok(()),
      _ => Err(anyhow::Error::msg(format!("[MPI_Comm_size] Unknown code: {}", r))),
    }
  }

  pub fn send<T>(&mut self, buff: &mut [T], peer: usize, tag: i32) -> anyhow::Result<()>
    where T: DataType,
  {
    let r = unsafe {
      ffi::MPI_Send(
        buff.as_mut_ptr() as *mut std::os::raw::c_void,
        buff.len() as i32,
        T::to_ffi(),
        peer as i32,
        tag,
        self.comm,
      ) as u32
    };
    match r {
      MPI_SUCCESS => Ok(()),
      _ => Err(anyhow::Error::msg(format!("[MPI_Send] Unknown code: {}", r))),
    }
  }

  pub fn recv<T>(&mut self, buff: &mut [T], peer: usize, tag: i32) -> anyhow::Result<()>
    where T: DataType,
  {
    let mut status: ffi::MPI_Status = malloc();
    let r = unsafe {
      ffi::MPI_Recv(
        buff.as_mut_ptr() as *mut std::os::raw::c_void,
        buff.len() as i32,
        T::to_ffi(),
        peer as i32,
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

  pub fn send_recv<T, U>(
    &mut self, send_buff: &mut [T], send_peer: usize, send_tag: usize,
    recv_buff: &mut [U], recv_peer: usize, recv_tag: usize,
  ) -> anyhow::Result<()>
    where
      T: DataType,
      U: DataType,
  {
    let mut status: ffi::MPI_Status = malloc();
    let r = unsafe {
      ffi::MPI_Sendrecv(
        send_buff.as_mut_ptr() as *mut std::os::raw::c_void,
        send_buff.len() as i32,
        T::to_ffi(),
        send_peer as i32,
        send_tag as i32,
        recv_buff.as_mut_ptr() as *mut std::os::raw::c_void,
        recv_buff.len() as i32,
        U::to_ffi(),
        recv_peer as i32,
        recv_tag as i32,
        self.comm,
        &mut status,
      ) as u32
    };
    match r {
      MPI_SUCCESS => Ok(()),
      _ => Err(anyhow::Error::msg(format!("[MPI_Sendrecv] Unknown code: {}", r))),
    }
  }

  pub fn send_recv_replace<T>(
    &mut self, buff: &mut [T],
    send_peer: usize, send_tag: usize,
    recv_peer: usize, recv_tag: usize,
  ) -> anyhow::Result<()>
    where
      T: DataType,
  {
    let mut status: ffi::MPI_Status = malloc();

    let r = unsafe {
      ffi::MPI_Sendrecv_replace(
        buff.as_mut_ptr() as *mut std::os::raw::c_void,
        buff.len() as i32,
        T::to_ffi(),
        send_peer as i32,
        send_tag as i32,
        recv_peer as i32,
        recv_tag as i32,
        self.comm,
        &mut status,
      ) as u32
    };
    match r {
      MPI_SUCCESS => Ok(()),
      _ => Err(anyhow::Error::msg(format!("[MPI_Sendrecv] Unknown code: {}", r))),
    }
  }

  pub fn broadcast<T>(&mut self, buff: &mut [T], root: usize) -> anyhow::Result<()>
    where T: DataType,
  {
    let r = unsafe {
      ffi::MPI_Bcast(
        buff.as_mut_ptr() as *mut std::os::raw::c_void, 
        buff.len() as i32,
        T::to_ffi(),
        root as i32,
        self.comm,
      ) as u32
    };
    match r {
      MPI_SUCCESS => Ok(()),
      _ => Err(anyhow::Error::msg(format!("[MPI_Bcast] Unknown code: {}", r))),
    }
  }

  pub fn scatter<T>(&mut self, send_buff: &mut [T], recv_buff: &mut [T], root: usize) -> anyhow::Result<()>
    where T: DataType,
  {
    let size = self.size()?;
    let rank = self.rank()?;
    if rank == root {
      let required = size * recv_buff.len();
      if required != send_buff.len() {
        return Err(anyhow::Error::msg(format!("SendBuf size not match. Required: {} != Actual: {}", required, send_buff.len())));
      }
    }

    let r = unsafe {
      ffi::MPI_Scatter(
        send_buff.as_mut_ptr() as *mut std::os::raw::c_void,
        recv_buff.len() as i32,
        T::to_ffi(),
        recv_buff.as_mut_ptr() as *mut std::os::raw::c_void,
        recv_buff.len() as i32,
        T::to_ffi(),
        root as i32,
        self.comm,
      ) as u32
    };
    match r {
      MPI_SUCCESS => Ok(()),
      _ => Err(anyhow::Error::msg(format!("[MPI_Scatter] Unknown code: {}", r))),
    }
  }

  pub fn gather<T>(&mut self, send_buff: &mut [T], recv_buff: &mut [T], root: usize) -> anyhow::Result<()>
    where T: DataType,
  {
    let size = self.size()?;
    let rank = self.rank()?;
    if rank == root {
      let required = size * send_buff.len();
      if required != recv_buff.len() {
        return Err(anyhow::Error::msg(format!("SendBuf size not match. Required: {} != Actual: {}", required, send_buff.len())));
      }
    }
    let r = unsafe {
      ffi::MPI_Gather(
        send_buff.as_mut_ptr() as *mut std::os::raw::c_void,
        send_buff.len() as i32,
        T::to_ffi(),
        recv_buff.as_mut_ptr() as *mut std::os::raw::c_void,
        send_buff.len() as i32,
        T::to_ffi(),
        root as i32,
        self.comm,
      ) as u32
    };
    match r {
      MPI_SUCCESS => Ok(()),
      _ => Err(anyhow::Error::msg(format!("[MPI_Gather] Unknown code: {}", r))),
    }
  }

  pub fn reduce<T>(&mut self, send_buff: &mut [T], recv_buff: &mut [T], op: Op, root: usize) -> anyhow::Result<()>
    where T: DataType,
  {
    let rank = self.rank()?;
    if rank == root {
      if send_buff.len() != recv_buff.len() {
        return Err(anyhow::Error::msg(format!("SendBuf and RecvBuf must have the same length. SendBuf: {} != RecvBuf: {}", send_buff.len(), recv_buff.len())));
      }
    }
    let count = send_buff.len();

    let r = unsafe {
      ffi::MPI_Reduce(
        send_buff.as_mut_ptr() as *mut std::os::raw::c_void,
        recv_buff.as_mut_ptr() as *mut std::os::raw::c_void,
        count as i32,
        T::to_ffi(),
        op.to_ffi(),
        root as i32,
        self.comm,
      ) as u32
    };
    match r {
      MPI_SUCCESS => Ok(()),
      _ => Err(anyhow::Error::msg(format!("[MPI_Reduce] Unknown code: {}", r))),
    }
  }

  pub fn all_reduce<T>(&mut self, send_buff: &mut [T], recv_buff: &mut [T], op: Op) -> anyhow::Result<()>
    where T: DataType,
  {
    if send_buff.len() != recv_buff.len() {
      return Err(anyhow::Error::msg(format!("SendBuf and RecvBuf must have the same length. SendBuf: {} != RecvBuf: {}", send_buff.len(), recv_buff.len())));
    }
    let count = send_buff.len();
    let r = unsafe {
      ffi::MPI_Allreduce(
        send_buff.as_mut_ptr() as *mut std::os::raw::c_void,
        recv_buff.as_mut_ptr() as *mut std::os::raw::c_void,
        count as i32,
        T::to_ffi(),
        op.to_ffi(),
        self.comm,
      ) as u32
    };
    match r {
      MPI_SUCCESS => Ok(()),
      _ => Err(anyhow::Error::msg(format!("[MPI_Allreduce] Unknown code: {}", r))),
    }
  }
}
