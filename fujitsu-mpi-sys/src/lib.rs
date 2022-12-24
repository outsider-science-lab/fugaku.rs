use libc::{c_int, c_char};

extern "C" {
  fn MPI_Init(argc: *const c_int, argv: *const *const *const c_char) -> c_int;
  fn MPI_Finalize() -> c_int;
}

const MPI_SUCCESS: c_int = 0;

pub fn initialize() -> anyhow::Result<()> {
  let args: Vec<String> = std::env::args().collect();
  let argc = args.len() as c_int;
  let mut argv: Vec<*const c_char> = Vec::new();
  for arg in &args {
    argv.push(arg.as_ptr() as *const c_char);
  }
  let argv_ptr = argv.as_ptr();
  let r = unsafe {
    MPI_Init(&argc as *const c_int, &argv_ptr as *const *const *const c_char)
  };
  match r {
    MPI_SUCCESS => Ok(()),
    _ => Err(anyhow::Error::msg(format!("[MPI_Init] Unknown code: {}", r))),
  }
}

pub fn finalize() -> anyhow::Result<()> {
  let r = unsafe {
    MPI_Finalize()
  };
  match r {
    MPI_SUCCESS => Ok(()),
    _ => Err(anyhow::Error::msg(format!("[MPI_Finalize] Unknown code: {}", r))),
  }
}
