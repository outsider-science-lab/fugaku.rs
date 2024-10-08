use std::ptr::addr_of_mut;

use mpi_sys as ffi;

pub enum Op {
  Null,
  BitAnd,
  BitOr,
  BitXor,
  LogicalAnd,
  LogicalOr,
  LogicalXor,
  Max,
  MaxLocation,
  Min,
  MinLocation,
  Sum,
  Prod,
}

impl Op {
  pub fn to_ffi(&self) -> ffi::MPI_Op {
    let op = unsafe {
      match self {
        &Self::Null => addr_of_mut!(ffi::ompi_mpi_op_null),
        &Self::BitAnd => addr_of_mut!(ffi::ompi_mpi_op_band),
        &Self::BitOr => addr_of_mut!(ffi::ompi_mpi_op_bor),
        &Self::BitXor => addr_of_mut!(ffi::ompi_mpi_op_bxor),
        &Self::LogicalAnd => addr_of_mut!(ffi::ompi_mpi_op_land),
        &Self::LogicalOr => addr_of_mut!(ffi::ompi_mpi_op_lor),
        &Self::LogicalXor => addr_of_mut!(ffi::ompi_mpi_op_lxor),
        &Self::Max => addr_of_mut!(ffi::ompi_mpi_op_max),
        &Self::MaxLocation => addr_of_mut!(ffi::ompi_mpi_op_maxloc),
        &Self::Min => addr_of_mut!(ffi::ompi_mpi_op_min),
        &Self::MinLocation => addr_of_mut!(ffi::ompi_mpi_op_minloc),
        &Self::Sum => addr_of_mut!(ffi::ompi_mpi_op_sum),
        &Self::Prod => addr_of_mut!(ffi::ompi_mpi_op_prod),
      }
    };
    op as ffi::MPI_Op
  }
}
