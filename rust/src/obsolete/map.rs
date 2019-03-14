use std::ffi::CString;
use std::os::raw::{c_int,c_char};

extern {
    fn luacall_initialize() -> c_int;
    fn luacall_cleanup();
    fn luacall_loadfile(script: *const c_char) -> c_int;
    fn luacall_func_string(func: *const c_char, arg: *const c_char, ret: *mut c_char) -> c_int;
    fn luacall_getglobal(func: *const c_char) -> c_int;
}

const LUA_OK: c_int = 0;
const LUA_TFUNCTION: c_int = 6;

pub type RecordMapper = fn(&str) -> String;
pub type RecordHasher = Box<Fn(&str) -> String>;

pub struct Mapper {
    /// name of the Lua script to load
    script: String,
    /// name of the function to be used as a hasher/mapper
    func: CString,
    /// maximum length of the record ID. Used to preallocate recid
    max_id: usize,
}

impl Mapper {
    /// Just initializes the Lua engine. The Lua state is kept into the library.
    pub fn new(script: &str, func: &str, max_id: usize) -> Mapper {
        // load Lua environment
        let mut rc: c_int;
        unsafe {
            rc = luacall_initialize();
        }
        if rc != LUA_OK {
            panic!("unable to initialize Lua interpreter, rc={}", rc);
        }

        // load script and compiles it
        let c_script = CString::new(script.clone()).unwrap();
        unsafe {
            rc = luacall_loadfile(c_script.as_ptr());
        } 
        if rc != LUA_OK {
            panic!("unable to load Lua script {}, Lua rc={}", script, rc);
        }

        // check whether provided function name is existing
        let c_func = CString::new(func).unwrap();
        unsafe {
            rc = luacall_getglobal(c_func.as_ptr());
        }
        if rc != LUA_TFUNCTION {
            panic!("Lua function {} is not a function, Lua rc={}", func, rc);
        }                            

        Mapper{ 
            script: script.to_string(),
            func: CString::new(func).unwrap(),
            max_id: max_id
        }
    }

    /// Calls the mapper func, which maps a string into a string.
    pub fn call(&self, in_arg: &str) -> String {
        // convert arguments
        let arg = CString::new(in_arg).unwrap();
        //let ret = arg.clone();
        let pre_allocated = String::with_capacity(self.max_id+1);
        let ret = CString::new(pre_allocated).unwrap();

        // need to transform into a raw pointer to call C
        let ptr = ret.into_raw();

        let rc: c_int;
        unsafe {
            rc = luacall_func_string(self.func.as_ptr(), arg.as_ptr(), ptr);
        } 
        if rc != LUA_OK {
            panic!("unable to call Lua function {:?}, Lua rc={}", self.func, rc);
        }        

        // need to retake ownership
        let returned_string = unsafe { CString::from_raw(ptr).into_string() };

        returned_string.unwrap()
    }

}

/// Cleans up Lua environment.
impl Drop for Mapper {
    fn drop(&mut self) {
        unsafe {
            luacall_cleanup();
        }
    }    
}

#[cfg(test)]
mod tests {

    use map::Mapper;

    #[test]
    #[should_panic]    
    #[allow(unused_variables)]
    fn non_existing_script() {
        // load our layout
        let map = Mapper::new("foo.lua", "func", 10);
    }

    #[test]
    #[should_panic]    
    #[allow(unused_variables)]    
    fn non_existing_func() {
        // load our layout
        let map = Mapper::new("./tests/lua/test.lua", "func", 10);
        let s = map.call("AABB");
    }

    #[test]
    fn call_func() {
        // load our layout
        let map = Mapper::new("./tests/lua/test.lua", "map1", 2);
        let s = map.call("AABBAAAAAAAAAAAAAAAAAAAA");
        assert_eq!(s, "AA".to_string());
    }        

}