// Doc:
// - https://rookiehpc.github.io/mpi/docs/mpi_request/

use std::borrow::Borrow;

use mpi_common::malloc;

use mpi_sys as ffi;
use ffi::{
  MPI_Request,
  MPI_Status,
  MPI_SUCCESS,
};
use futures::Future;

pub struct Request<'v, T: ?Sized> {
  mem: &'v T,
  req: MPI_Request,
}

impl <'v, T: ?Sized> Future for Request<'v, T> {
  type Output = anyhow::Result<&'v T>;

  fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
    use std::task::Poll;
    match self.test() {
      Ok(ready) => {
        if ready {
          Poll::Ready(Ok(self.mem))
        } else {
          // FIXME(ledyba): Better way to wake.
          cx.waker().wake_by_ref();
          Poll::Pending
        }
      }
      Err(err) => {
        Poll::Ready(Err(err))
      },
    }
  }
}

impl <'v, T: ?Sized> Request<'v, T> {
  pub(crate) fn new<V: ?Sized>(mem: &V, req: MPI_Request) -> Request<V> {
    Request {
      mem,
      req,
    }
  }

  fn test(&mut self) -> anyhow::Result<bool> {
    let mut status: MPI_Status = malloc();
    let mut ready = 0;
    let r = unsafe {
      // https://www.open-mpi.org/doc/v4.1/man3/MPI_Test.3.php
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
