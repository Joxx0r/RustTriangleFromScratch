type c_int = i32;
type c_uint = u32;
type HANDLE = PVOID;
type HBRUSH = HANDLE;
type HCURSOR = HICON;
type HICON = HANDLE;
type HINSTANCE = HANDLE;
type HWND = HANDLE;
type LONG_PTR = isize;
type LPARAM = LONG_PTR;
type LPCWSTR = *const WCHAR;
type LRESULT = LONG_PTR;
type PVOID = *mut core::ffi::c_void;
type UINT = c_uint;
type UINT_PTR = usize;
type WCHAR = wchar_t;
type wchar_t = u16;
type WPARAM = UINT_PTR;
type HMODULE = HANDLE;


macro_rules! unsafe_impl_default_zeroed {
    ($t:ty) => {
      impl Default for $t {
        #[inline]
        #[must_use]
        fn default() -> Self {
          unsafe { core::mem::zeroed() }
        }
      }
    };
  }


type WNDPROC = Option<
  unsafe extern "system" fn(
    hwnd: HWND,
    uMsg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
  ) -> LRESULT
>;

unsafe extern "system" fn dummy_window_procedure(
    hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM,
  ) -> LRESULT {
    unimplemented!()
  }


#[repr(C)]
pub struct WNDCLASSW {
  style: UINT,
  lpfnWndProc: WNDPROC,
  cbClsExtra: c_int,
  cbWndExtra: c_int,
  hInstance: HINSTANCE,
  hIcon: HICON,
  hCursor: HCURSOR,
  hbrBackground: HBRUSH,
  lpszMenuName: LPCWSTR,
  lpszClassName: LPCWSTR,
}
unsafe_impl_default_zeroed!(WNDCLASSW);

#[link(name = "Kernel32")]
extern "system" {
  /// [`GetModuleHandleW`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
  pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
}

fn main() {

    let hInstance = unsafe { GetModuleHandleW(core::ptr::null()) };
    let mut wc:WNDCLASSW = WNDCLASSW::default();
    wc.lpfnWndProc = Some(dummy_window_procedure);
    wc.hInstance = todo!();
    wc.lpszClassName = todo!();
}
