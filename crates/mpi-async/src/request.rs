// Doc:
// - https://rookiehpc.github.io/mpi/docs/mpi_request/

use mpi_common::malloc;

use mpi_sys as ffi;
use ffi::{
  MPI_Request,
  MPI_Status,
  MPI_SUCCESS,
};
use futures::Future;

pub struct Request {
  req: MPI_Request,
}

impl Future for Request {
    type Output = anyhow::Result<()>;

    fn poll(mut self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
      use std::task::Poll;
      let t = self.test();
      if t.is_ok() {
        if t.unwrap() {
          println!("OK!");
          Poll::Ready(Ok(()))
        } else {
          println!("FALSE");
          Poll::Pending
        }
      } else {
        println!("ERR");
        Poll::Ready(Err(t.unwrap_err()))
      }
    }
}

impl Request {
  pub(crate) fn new(req: MPI_Request) -> Self {
    Self {
      req,
    }
  }

  fn test(&mut self) -> anyhow::Result<bool> {
    let mut status: MPI_Status = malloc();
    let mut ready = 0;
    let r = unsafe {
        ffi::MPI_Test(
          &mut self.req,
          &mut ready,
          &mut status,
        ) as u32
    };
    match r {
      MPI_SUCCESS => Ok(ready != 0),
      _ => Err(anyhow::Error::msg(format!("[MPI_Test] Unknown code: {}", r))),
    }
  }
}
