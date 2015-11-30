use std::ptr::Unique;
use ffi;
use std::ffi::{CString, CStr};

extern crate libc;

fn to_c_str(s: &str) -> *const libc::c_char {
    CString::new(s).unwrap().as_ptr()
}

pub struct Status {
    p: Unique<ffi::TF_Status>,
}
pub struct Session {
    p: Unique<ffi::TF_Session>,
}
pub struct SessionOptions {
    p: Unique<ffi::TF_SessionOptions>,
}
pub struct Tensor {
    pub p: Unique<ffi::TF_Tensor>,
}

impl Status {
    fn new() -> Status {
        let p = unsafe { Unique::new(ffi::TF_NewStatus()) };
        Status { p: p }
    }
    pub fn code(&self) -> ffi::TF_Code {
        unsafe { ffi::TF_GetCode(self.p.get()) }
    }

    pub fn message(&self) -> &CStr {
        unsafe { CStr::from_ptr(ffi::TF_Message(self.p.get())) }
    }
}

impl Drop for Status {
    fn drop(&mut self) {
        unsafe { ffi::TF_DeleteStatus(self.p.get_mut()) }
    }
}

impl SessionOptions {
    pub fn new() -> SessionOptions {
        let p = unsafe { Unique::new(ffi::TF_NewSessionOptions()) };
        SessionOptions { p: p }
    }

    pub fn set_target(&mut self, target: &str) -> &Self {
        unsafe { ffi::TF_SetTarget(self.p.get_mut(), to_c_str(target)) }
        self
    }

    pub fn set_config(&mut self, config_proto: &str) -> Result<&Self, Status> {
        let mut status = Status::new();
        unsafe {
            ffi::TF_SetConfig(self.p.get_mut(),
                              to_c_str(config_proto) as *mut libc::c_void,
                              config_proto.len() as u64,
                              status.p.get_mut())
        }
        match status.code() {
            ffi::TF_OK => Ok(self),
            _ => Err(status),
        }
    }

}

impl Drop for SessionOptions {
    fn drop(&mut self) {
        unsafe { ffi::TF_DeleteSessionOptions(self.p.get_mut()) }
    }
}

impl Session {
    pub fn new(o: &SessionOptions) -> Result<Session, Status> {
        let mut status = Status::new();
        let session = unsafe { Unique::new(ffi::TF_NewSession(o.p.get(), status.p.get_mut())) };
        match status.code() {
            ffi::TF_OK => Ok(Session { p: session }),
            _ => Err(status),
        }
    }

    pub fn extend_graph(&mut self, graph_proto: &str) -> Result<&Self, Status> {
        let mut status = Status::new();
        unsafe {
            ffi::TF_ExtendGraph(self.p.get_mut(),
                                to_c_str(graph_proto) as *mut libc::c_void,
                                graph_proto.len() as u64,
                                status.p.get_mut())
        }
        match status.code() {
            ffi::TF_OK => Ok(self),
            _ => Err(status),
        }
    }

    pub fn run(&mut self,
               input_names: &[&str],
               target_names: &[&str],
               output_names: &[&str],
               input_tensors: &mut [*mut ffi::TF_Tensor],
               output_tensors: &mut [*mut ffi::TF_Tensor])
               -> Result<(), Status> {
        fn to_c_char(s: &[&str]) -> Vec<*const ::libc::c_char> {
            return s.iter().map(|&s| to_c_str(s)).collect::<Vec<_>>();
        }
        let mut input_names = to_c_char(input_names);
        let mut target_names = to_c_char(target_names);
        let mut output_names = to_c_char(output_names);
        let mut status = Status::new();
        unsafe {
            ffi::TF_Run(self.p.get_mut(),
                        input_names.as_mut_ptr(),
                        input_tensors.as_mut_ptr(),
                        input_tensors.len() as i32,
                        output_names.as_mut_ptr(),
                        output_tensors.as_mut_ptr(),
                        output_tensors.len() as i32,
                        target_names.as_mut_ptr(),
                        target_names.len() as i32,
                        status.p.get_mut())
        }
        match status.code() {
            ffi::TF_OK => Ok(()),
            _ => Err(status),
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        let mut status = Status::new();
        unsafe { ffi::TF_DeleteSession(self.p.get_mut(), status.p.get_mut()) }
    }
}
