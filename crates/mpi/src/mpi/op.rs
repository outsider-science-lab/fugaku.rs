use mpi_sys as ffi;

pub enum Op {
  Null,
  Sum,
}

impl Op {
  pub(crate) fn to_ffi(&self) -> ffi::MPI_Op {
    let op = unsafe {
      match self {
        &Self::Null => &mut ffi::ompi_mpi_op_null,
        &Self::Sum => &mut ffi::ompi_mpi_op_sum,
      }
    };
    op as *mut ffi::ompi_predefined_op_t as ffi::MPI_Op
  }
}
