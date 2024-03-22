use std::ptr::addr_of_mut;

use mpi_sys as ffi;
use ffi::MPI_Datatype;

pub trait DataType {
  fn to_ffi() -> MPI_Datatype;
}

impl DataType for f32 {
  fn to_ffi() -> MPI_Datatype {
    unsafe {
      addr_of_mut!(ffi::ompi_mpi_float) as MPI_Datatype
    }
  }
}

impl DataType for f64 {
  fn to_ffi() -> MPI_Datatype {
    unsafe {
      addr_of_mut!(ffi::ompi_mpi_double) as MPI_Datatype
    }
  }
}

impl DataType for u8 {
  fn to_ffi() -> MPI_Datatype {
    unsafe {
      addr_of_mut!(ffi::ompi_mpi_uint8_t) as MPI_Datatype
    }
  }
}

impl DataType for i8 {
  fn to_ffi() -> MPI_Datatype {
    unsafe {
      addr_of_mut!(ffi::ompi_mpi_int8_t) as MPI_Datatype
    }
  }
}

impl DataType for u16 {
  fn to_ffi() -> MPI_Datatype {
    unsafe {
      addr_of_mut!(ffi::ompi_mpi_uint16_t) as MPI_Datatype
    }
  }
}

impl DataType for i16 {
  fn to_ffi() -> MPI_Datatype {
    unsafe {
      addr_of_mut!(ffi::ompi_mpi_int16_t) as MPI_Datatype
    }
  }
}

impl DataType for u32 {
  fn to_ffi() -> MPI_Datatype {
    unsafe {
      addr_of_mut!(ffi::ompi_mpi_uint32_t) as MPI_Datatype
    }
  }
}

impl DataType for i32 {
  fn to_ffi() -> MPI_Datatype {
    unsafe {
      addr_of_mut!(ffi::ompi_mpi_int32_t) as MPI_Datatype
    }
  }
}

impl DataType for u64 {
  fn to_ffi() -> MPI_Datatype {
    unsafe {
      addr_of_mut!(ffi::ompi_mpi_uint64_t) as MPI_Datatype
    }
  }
}

impl DataType for i64 {
  fn to_ffi() -> MPI_Datatype {
    unsafe {
      addr_of_mut!(ffi::ompi_mpi_int64_t) as MPI_Datatype
    }
  }
}
