mod workflow;
use workflow::Workflow;
use std::collections::HashMap;

use libc::c_char;
use c_vec::CVec;
use std::ffi::{CStr,CString};

#[repr(C)]
pub struct CWorkflow {
    on: CTrigger,
    name: *const c_char,
    jobs: HashMap<*const c_char, CJob>,
}

impl Workflow {
    fn to_c(self) -> CWorkflow {
        CWorkflow {
            on: CTrigger::CTriggerAtom(CString::new("").unwrap().into_raw()),
                //self.on.to_c(),
            name: CString::new(self.name).unwrap().into_raw(),
            jobs: HashMap::new(),
        }
    }
}

#[repr(C)]
enum CTrigger {
    CTriggerAtom(*const c_char),
    CTriggerList(CVec<*const c_char>),
    CTriggerPush {
        push: CTriggerPushInner,
    },
    CTriggerSchedule {
        schedule: CVec<CTriggerScheduleInner>,
    },
}

#[repr(C)]
struct CTriggerPushInner {
    branches: Option<CStringList>,
    tags: Option<CStringList>,
}

#[repr(C)]
struct CTriggerScheduleInner {
    cron: *const c_char,
    branches: Option<CStringList>,
    tags: Option<CStringList>,
}

#[repr(C)]
struct CJob {
    needs: Option<CStringList>,
    conditional: Option<*const c_char>,
    strategy: Option<CStrategy>,
    name: Option<*const c_char>,
    runs_on: Option<*const c_char>,
    timeout_minutes: Option<u8>,
    cancel_timeout_minutes: Option<u8>,
    continue_on_error: Option<bool>,
    container: Option<CContainer>,
    services: Option<HashMap<*const c_char,CContainer>>,
    steps: CVec<CStep>,
}

#[repr(C)]
struct CStrategy {
    fail_fast: bool,
    max_parallel: bool,
    matrix: HashMap<*const c_char, CVec<*const c_char>>,
}

#[repr(C)]
enum CContainer {
    Name(*const c_char),
    Properties {
        image: *const c_char,
        options: *const c_char,
        env: HashMap<*const c_char,*const c_char>,
        ports: CVec<*const c_char>,
        volumes: CVec<*const c_char>,
    }
}

#[repr(C)]
struct CStep {
    name: *const c_char,
    run: *const c_char,
}

#[repr(C)]
enum CStringList {
    Atom(*const c_char),
    List(CVec<*const c_char>),
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
