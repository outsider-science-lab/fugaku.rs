use fujitsu_mpi_sys as ffi;
use ffi::{
  MPI_Comm,
  MPI_SUCCESS,
};
use crate::mpi;

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
    where T: mpi::DataType
  {
    let r = unsafe {
      ffi::MPI_Send(
        buff.as_mut_ptr() as *mut std::os::raw::c_void,
        buff.len() as i32,
        T::to_ffi(),
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
    where T: mpi::DataType
  {
    let mut status: ffi::MPI_Status = unsafe {
      std::mem::MaybeUninit::<ffi::MPI_Status>::zeroed().assume_init()
    };
    let r = unsafe {
      ffi::MPI_Recv(
        buff.as_mut_ptr() as *mut std::os::raw::c_void,
        buff.len() as i32,
        T::to_ffi(),
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
    where T: mpi::DataType
  {
    let r = unsafe {
      ffi::MPI_Bcast(
        buff.as_mut_ptr() as *mut std::os::raw::c_void, 
        buff.len() as i32,
        T::to_ffi(),
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
    where T: mpi::DataType
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
    where T: mpi::DataType
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

  pub fn reduce<T>(&mut self, send_buff: &mut [T], recv_buff: &mut [T], op: mpi::Op , root: usize) -> anyhow::Result<()>
    where T: mpi::DataType
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
}
