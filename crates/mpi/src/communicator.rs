// Doc:
// - https://rookiehpc.github.io/mpi/docs/mpi_comm/

use mpi_common::{as_void_ptr, as_mut_void_ptr};
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
  size: Option<usize>,
  rank: Option<usize>,
}

impl Communicator {
  pub fn new(
    comm: MPI_Comm,
  ) -> Self {
    Self {
      comm,
      size: None,
      rank: None,
    }
  }

  pub fn size(&mut self) -> anyhow::Result<usize> {
    match self.size {
      Some(size) => Ok(size),
      None => {
        let size: usize = mpi_common::communicator::size(self.comm)?;
        self.size = Some(size);
        Ok(size)
      }
    }
  }

  pub fn rank(&mut self) -> anyhow::Result<usize> {
    match self.rank {
      Some(rank) => Ok(rank),
      None => {
        let rank: usize = mpi_common::communicator::rank(self.comm)?;
        self.rank = Some(rank);
        Ok(rank)
      }
    }
  }

  pub fn abort(&self, error_code: i32) -> anyhow::Result<()> {
    mpi_common::communicator::abort(self.comm, error_code)
  }
  
  pub fn send<T>(&mut self, send_buff: &[T], peer: usize, tag: i32) -> anyhow::Result<()>
    where T: DataType,
  {
    let r = unsafe {
      ffi::MPI_Send(
        as_void_ptr(send_buff),
        send_buff.len() as i32,
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

  pub fn recv<T>(&mut self, recv_buff: &mut [T], peer: usize, tag: i32) -> anyhow::Result<()>
    where T: DataType,
  {
    let r = unsafe {
      let mut status: ffi::MPI_Status = malloc();
      ffi::MPI_Recv(
        as_mut_void_ptr(recv_buff),
        recv_buff.len() as i32,
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
    &mut self,
    send_buff: &[T], send_peer: usize, send_tag: usize,
    recv_buff: &mut [U], recv_peer: usize, recv_tag: usize,
  ) -> anyhow::Result<()>
    where
      T: DataType,
      U: DataType,
  {
    let r = unsafe {
      let mut status: ffi::MPI_Status = malloc();
      ffi::MPI_Sendrecv(
        as_void_ptr(send_buff),
        send_buff.len() as i32,
        T::to_ffi(),
        send_peer as i32,
        send_tag as i32,
        as_mut_void_ptr(recv_buff),
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
    &mut self, send_recv_buff: &mut [T],
    send_peer: usize, send_tag: usize,
    recv_peer: usize, recv_tag: usize,
  ) -> anyhow::Result<()>
    where
      T: DataType,
  {
    let r = unsafe {
      let mut status: ffi::MPI_Status = malloc();
      ffi::MPI_Sendrecv_replace(
        as_mut_void_ptr(send_recv_buff),
        send_recv_buff.len() as i32,
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
        as_mut_void_ptr(buff),
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

  pub fn scatter<T>(&mut self, send_buff: &[T], recv_buff: &mut [T], root: usize) -> anyhow::Result<()>
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
        as_void_ptr(send_buff),
        send_buff.len() as i32,
        T::to_ffi(),
        as_mut_void_ptr(recv_buff),
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

  pub fn gather<T>(&mut self, send_buff: &[T], recv_buff: &mut [T], root: usize) -> anyhow::Result<()>
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
        as_void_ptr(send_buff),
        send_buff.len() as i32,
        T::to_ffi(),
        as_mut_void_ptr(recv_buff),
        recv_buff.len() as i32,
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

  pub fn reduce<T>(&mut self, send_buff: &[T], recv_buff: &mut [T], op: Op, root: usize) -> anyhow::Result<()>
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
        as_void_ptr(send_buff),
        as_mut_void_ptr(recv_buff),
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

  pub fn all_reduce<T>(&mut self, send_buff: &[T], recv_buff: &mut [T], op: Op) -> anyhow::Result<()>
    where T: DataType,
  {
    if send_buff.len() != recv_buff.len() {
      return Err(anyhow::Error::msg(format!("SendBuf and RecvBuf must have the same length. SendBuf: {} != RecvBuf: {}", send_buff.len(), recv_buff.len())));
    }
    let count = send_buff.len();
    let r = unsafe {
      ffi::MPI_Allreduce(
        as_void_ptr(send_buff),
        as_mut_void_ptr(recv_buff),
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

  pub fn all_to_all<T>(
    &mut self,
    send_buff: &[T], send_count: usize,
    recv_buff: &mut [T], recv_count: usize,
  ) -> anyhow::Result<()>
    where T: DataType,
  {
    let r = unsafe {
      // https://learn.microsoft.com/en-us/message-passing-interface/mpi-alltoall-function
      ffi::MPI_Alltoall(
        as_void_ptr(send_buff),
        send_count as i32,
        T::to_ffi(),
        as_mut_void_ptr(recv_buff),
        recv_count as i32,
        T::to_ffi(),
        self.comm,
      ) as u32
    };
    match r {
      MPI_SUCCESS => Ok(()),
      _ => Err(anyhow::Error::msg(format!("[MPI_Allreduce] Unknown code: {}", r))),
    }
  }
}
