mod workflow;
use workflow::Workflow;
use std::collections::HashMap;

use libc::c_char;
use std::ffi::{CStr,CString};

#[repr(C)]
pub struct CWorkflow {
    on: CTrigger,
    name: *const c_char,
}

impl Workflow {
    fn to_c(self) -> CWorkflow {
        CWorkflow {
            on: CTrigger::CTriggerAtom(CString::new("push").unwrap().into_raw()),
                //self.on.to_c(),
            name: CString::new(self.name).unwrap().into_raw(),
        }
    }
}

#[repr(C)]
enum CTrigger {
    CCTriggerAtom(*const c_char),
}

#[repr(C)]
pub enum CResult {
    Ok(CWorkflow),
    Err(*const c_char),
}

#[no_mangle]
pub extern "C" fn parse(c_str_p: *mut c_char) -> CResult {
    let c_str = unsafe {
        assert!(!c_str_p.is_null());

        CStr::from_ptr(c_str_p)
    };
    let s = c_str.to_str().unwrap();
    match workflow::parse(s) {
        Ok(w) => CResult::Ok(w.to_c()),
        Err(e) => {
            let c_str = CString::new(e.to_string()).unwrap();
            CResult::Err(c_str.into_raw())
        }
    }
}

#[no_mangle]
pub extern "C" fn is_result_ok(r: CResult) -> bool {
    match r {
        CResult::Ok(_) => true,
        CResult::Err(_) => false,
    }
}

#[no_mangle]
pub extern "C" fn err_from_result(r: CResult) ->  *const c_char {
    match r {
        CResult::Ok(_) => panic!("wups"),
        CResult::Err(e) => e,
    }
}

#[no_mangle]
pub extern "C" fn workflow_from_result(r: CResult) -> CWorkflow {
    match r {
        CResult::Ok(w) => w,
        CResult::Err(_) => panic!("wups"),
    }
}
