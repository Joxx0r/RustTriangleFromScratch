use std::os::raw::{c_ulong, c_ushort};
use core::ptr::{null, null_mut};

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
type DWORD = c_ulong;
type WORD = c_ushort;
type ATOM = WORD;
type HMENU = HANDLE;
type LPVOID = *mut core::ffi::c_void;
type BOOL = c_int;

/*
 * Window Styles
 */

/*
 * Window Styles
 */
const WS_OVERLAPPED:u32 =          0x00000000;
const WS_POPUP:u32 =              0x80000000;
const WS_CHILD:u32 =              0x40000000;
const WS_MINIMIZE:u32 =           0x20000000;
const WS_VISIBLE:u32 =            0x10000000;
const WS_DISABLED:u32 =           0x08000000;
const WS_CLIPSIBLINGS:u32 =       0x04000000;
const WS_CLIPCHILDREN:u32 =       0x02000000;
const WS_MAXIMIZE:u32 =           0x01000000;
const WS_CAPTION:u32 =            0x00C00000;    
const WS_BORDER:u32 =             0x00800000;
const WS_DLGFRAME:u32 =           0x00400000;
const WS_VSCROLL:u32 =            0x00200000;
const WS_HSCROLL:u32 =            0x00100000;
const WS_SYSMENU:u32 =            0x00080000;
const WS_THICKFRAME:u32 =         0x00040000;
const WS_GROUP:u32 =              0x00020000;
const WS_TABSTOP:u32 =            0x00010000;

const WS_MINIMIZEBOX:u32 =        0x00020000;
const WS_MAXIMIZEBOX:u32 =        0x00010000;

const WS_TILED:u32=         WS_OVERLAPPED;
const WS_ICONIC:u32=        WS_MINIMIZE;
const WS_SIZEBOX:u32=       WS_THICKFRAME;
const WS_TILEDWINDOW:u32=   WS_OVERLAPPEDWINDOW;

/*
 * Common Window Styles
 */
const WS_OVERLAPPEDWINDOW:u32 = 
                            (WS_OVERLAPPED | 
                             WS_CAPTION        | 
                             WS_SYSMENU        | 
                             WS_THICKFRAME     | 
                             WS_MINIMIZEBOX    | 
                             WS_MAXIMIZEBOX);

const CW_USEDEFAULT:c_int   = 0x8000000 as c_int;
const SW_SHOW: c_int = 5;

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
  pub fn GetLastError() -> DWORD;
}

#[link(name = "User32")]
extern "system" {
  /// [`GetModuleHandleW`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
  pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> ATOM;
  pub fn CreateWindowExW(dwExStyle: DWORD, lpClassName: LPCWSTR, lpWindowName: LPCWSTR, dwStyle: DWORD, X: c_int, Y: c_int, nWidth: c_int, nHeight: c_int, hWndParent: HWND, hMenu: HMENU, hInstance: HINSTANCE, lpParam: LPVOID,) -> HWND;
  pub fn ShowWindow(hWnd:HWND, nCmdShow:c_int) -> BOOL;
  /// [`DefWindowProcW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
  pub fn DefWindowProcW( hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM,) -> LRESULT;
}

pub fn wide_null(string:&str ) -> Vec<u16> {
  return string.encode_utf16().chain(Some(0)).collect();
}


fn main() {

    let hInstance = unsafe { GetModuleHandleW(core::ptr::null()) };
    let window_class_name = wide_null("Some Window Class");
    let window_name = wide_null("Some Window Name");

    let mut wc:WNDCLASSW = WNDCLASSW::default();
    wc.lpfnWndProc = Some(dummy_window_procedure);
    wc.hInstance = hInstance;
    wc.lpszClassName = window_class_name.as_ptr();
    let return_result:ATOM= unsafe { RegisterClassW(&wc) } ;
    if return_result == 0 {
      let last_error = unsafe { GetLastError() };
      panic!("Error code failed creating window {} ", last_error);
    }

    let hwnd = unsafe {
        CreateWindowExW(
          0, 
          window_class_name.as_ptr(), 
          window_name.as_ptr(), 
          WS_OVERLAPPEDWINDOW,
          CW_USEDEFAULT,
          CW_USEDEFAULT,
          CW_USEDEFAULT,
          CW_USEDEFAULT,
          null_mut(),
          null_mut(),
          hInstance,
          null_mut())
    };

    if hwnd.is_null() {
      panic!("failed creating window");
    }

    let _previously_visible = unsafe { ShowWindow(hwnd, SW_SHOW) };

}
