use std::ptr::addr_of_mut;

use mpi_sys as ffi;

pub enum Op {
  Null,
  Sum,
}

impl Op {
  pub fn to_ffi(&self) -> ffi::MPI_Op {
    let op = unsafe {
      match self {
        &Self::Null => addr_of_mut!(ffi::ompi_mpi_op_null),
        &Self::Sum => addr_of_mut!(ffi::ompi_mpi_op_sum),
      }
    };
    op as ffi::MPI_Op
  }
}
