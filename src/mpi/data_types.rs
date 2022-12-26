use fujitsu_mpi_sys as ffi;
use ffi::{
  MPI_Datatype,
  ompi_predefined_datatype_t,
};

pub trait DataType {
  fn mpi_data_type() -> MPI_Datatype;
}

unsafe fn cast(typ: &mut ompi_predefined_datatype_t) -> MPI_Datatype {
  typ as *mut ffi::ompi_predefined_datatype_t as MPI_Datatype
}

impl DataType for f32 {
  fn mpi_data_type() -> MPI_Datatype {
    unsafe {
      cast(&mut ffi::ompi_mpi_float)
    }
  }
}

impl DataType for f64 {
  fn mpi_data_type() -> MPI_Datatype {
    unsafe {
      cast(&mut ffi::ompi_mpi_double)
    }
  }
}

impl DataType for u8 {
  fn mpi_data_type() -> MPI_Datatype {
    unsafe {
      cast(&mut ffi::ompi_mpi_unsigned_char)
    }
  }
}

impl DataType for i8 {
  fn mpi_data_type() -> MPI_Datatype {
    unsafe {
      cast(&mut ffi::ompi_mpi_signed_char)
    }
  }
}

impl DataType for u16 {
  fn mpi_data_type() -> MPI_Datatype {
    unsafe {
      cast(&mut ffi::ompi_mpi_unsigned_short)
    }
  }
}

impl DataType for i16 {
  fn mpi_data_type() -> MPI_Datatype {
    unsafe {
      cast(&mut ffi::ompi_mpi_unsigned_short)
    }
  }
}

impl DataType for u32 {
  fn mpi_data_type() -> MPI_Datatype {
    unsafe {
      cast(&mut ffi::ompi_mpi_unsigned)
    }
  }
}

impl DataType for i32 {
  fn mpi_data_type() -> MPI_Datatype {
    unsafe {
      cast(&mut ffi::ompi_mpi_int)
    }
  }
}

impl DataType for u64 {
  fn mpi_data_type() -> MPI_Datatype {
    unsafe {
      cast(&mut ffi::ompi_mpi_unsigned_long)
    }
  }
}

impl DataType for i64 {
  fn mpi_data_type() -> MPI_Datatype {
    unsafe {
      cast(&mut ffi::ompi_mpi_long)
    }
  }
}
