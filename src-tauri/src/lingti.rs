use libloading::{Library, Symbol};
use std::ffi::{CString, CStr};
use std::os::raw::{c_char};
use std::path::PathBuf;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref DLL: Mutex<Option<Library>> = Mutex::new(None);
}

fn dll_path() -> PathBuf {
    std::env::current_exe().unwrap().parent().unwrap().join("lingti_sdk.dll")
}

/// 加载 DLL（只加载一次）
pub fn load_sdk() {
    let mut guard = DLL.lock().unwrap();
    if guard.is_none() {
        *guard = Some(unsafe { Library::new(dll_path()).expect("❌ 无法加载 lingti_sdk.dll") });
        println!("🔌 Lingti SDK loaded.");
    }
}

/// 调 StartTun2R(configJSON)
pub fn start_tun2r(json: &str) -> i32 {
    load_sdk();

    let dll = DLL.lock().unwrap();
    let lib = dll.as_ref().unwrap();
    let func: Symbol<unsafe extern "C" fn(*const c_char) -> i32> = unsafe { lib.get(b"StartTun2R").unwrap() };
    let c_json = CString::new(json).unwrap();

    unsafe { func(c_json.as_ptr()) }
}

/// ⭐ 调用：StartTun2RWithConfigFile(const char* configPath)
pub fn start_tun2r_with_config_file(config_path: &str) -> i32 {
    load_sdk();

    let dll = DLL.lock().unwrap();
    let lib = dll.as_ref().unwrap();

    unsafe {
        let func: Symbol<unsafe extern "C" fn(*const c_char) -> i32> =
            lib.get(b"StartTun2RWithConfigFile").unwrap();

        let cstr = CString::new(config_path).unwrap();
        func(cstr.as_ptr())
    }
}

/// 停止服务 StopTun2R()
pub fn stop_tun2r() -> i32 {
    load_sdk();
    let dll = DLL.lock().unwrap();
    let lib = dll.as_ref().unwrap();
    let func: Symbol<unsafe extern "C" fn() -> i32> = unsafe { lib.get(b"StopTun2R").unwrap() };
    unsafe { func() }
}

/// 获取版本号 GetSDKVersion()
pub fn get_version() -> String {
    load_sdk();
    let dll = DLL.lock().unwrap();
    let lib = dll.as_ref().unwrap();

    unsafe {
        let func: Symbol<unsafe extern "C" fn() -> *mut c_char> = lib.get(b"GetSDKVersion").unwrap();
        let free: Symbol<unsafe extern "C" fn(*mut c_char)> = lib.get(b"FreeString").unwrap();

        let result_ptr = func();

        if result_ptr.is_null() {
            return "Unknown".into();
        }
        let c_str = CStr::from_ptr(result_ptr);
        let version = c_str.to_string_lossy().into_owned();
        free(result_ptr);
        version
    }
}
