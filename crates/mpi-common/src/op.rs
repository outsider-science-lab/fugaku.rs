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
  /**
   * https://docs.open-mpi.org/en/v5.0.x/man-openmpi/man3/MPI_Rget_accumulate.3.html#mpi-rget-accumulate
   * A new predefined operation, MPI_REPLACE, is defined.
   * It corresponds to the associative function f(a, b) =b; that is,
   * the current value in the target memory is replaced by the value supplied by the origin.
   */
  Replace,
  /**
   * https://docs.open-mpi.org/en/v5.0.x/man-openmpi/man3/MPI_Rget_accumulate.3.html#mpi-rget-accumulate
   * A new predefined operation, MPI_NO_OP, is defined.
   * It corresponds to the assiciative function f(a, b) = a;
   * that is the current value in the target memoryis returned in the result buffer at the origin and
   * no operation is performed on the target buffer.
   */
  NoOp,
}

impl Op {
  pub fn to_ffi(&self) -> ffi::MPI_Op {
    let op = match self {
      &Self::Null => &raw mut ffi::ompi_mpi_op_null,
      &Self::BitAnd => &raw mut ffi::ompi_mpi_op_band,
      &Self::BitOr => &raw mut ffi::ompi_mpi_op_bor,
      &Self::BitXor => &raw mut ffi::ompi_mpi_op_bxor,
      &Self::LogicalAnd => &raw mut ffi::ompi_mpi_op_land,
      &Self::LogicalOr => &raw mut ffi::ompi_mpi_op_lor,
      &Self::LogicalXor => &raw mut ffi::ompi_mpi_op_lxor,
      &Self::Max => &raw mut ffi::ompi_mpi_op_max,
      &Self::MaxLocation => &raw mut ffi::ompi_mpi_op_maxloc,
      &Self::Min => &raw mut ffi::ompi_mpi_op_min,
      &Self::MinLocation => &raw mut ffi::ompi_mpi_op_minloc,
      &Self::Sum => &raw mut ffi::ompi_mpi_op_sum,
      &Self::Prod => &raw mut ffi::ompi_mpi_op_prod,
      &Self::Replace => &raw mut ffi::ompi_mpi_op_replace,
      &Self::NoOp => &raw mut ffi::ompi_mpi_op_no_op,
    };
    op as ffi::MPI_Op
  }
}
