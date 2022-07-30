// at the top of the file
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
type LPWSTR = *mut WCHAR;
type ULONG_PTR = usize;
type WPARAM = UINT_PTR;
type HMODULE = HANDLE;
type DWORD = c_ulong;
type WORD = c_ushort;
type ATOM = WORD;
type HMENU = HANDLE;
type LPVOID = *mut core::ffi::c_void;
type BOOL = c_int;
type LONG = c_long;
type c_long = i32;
type LPMSG = *mut MSG;
type HDC = HANDLE;
type BYTE = u8;
type LPPAINTSTRUCT = *mut PAINTSTRUCT;

const WINDOW_HEIGHT:f32  = 1024.0;
const WINDOW_WIDTH:f32  = 1024.0;
const WINDOW_START_POS_X:f32  = 0.0;
const WINDOW_START_POS_Y:f32  = 0.0;
const GWLP_USERDATA: c_int = -21;

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

const WM_NCCREATE: u32 = 0x0081;
const WM_CREATE: u32 = 0x0001;

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

pub const WM_CLOSE: u32 = 0x0010;
pub const WM_DESTROY: u32 = 0x0002;
const WM_SETCURSOR: u32 = 0x0020;
const WM_PAINT: u32 = 0x000F;
const COLOR_WINDOW: u32 = 5;

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


#[repr(C)]
pub struct POINT {
  x: LONG,
  y: LONG,
}
unsafe_impl_default_zeroed!(POINT);

type WNDPROC = Option<
  unsafe extern "system" fn(
    hwnd: HWND,
    uMsg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
  ) -> LRESULT
>;


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

#[repr(C)]
pub struct MSG {
  hwnd: HWND,
  message: UINT,
  wParam: WPARAM,
  lParam: LPARAM,
  time: DWORD,
  pt: POINT,
  lPrivate: DWORD,
}
unsafe_impl_default_zeroed!(MSG);


#[repr(C)]
pub struct PAINTSTRUCT {
  hdc: HDC,
  fErase: BOOL,
  rcPaint: RECT,
  fRestore: BOOL,
  fIncUpdate: BOOL,
  rgbReserved: [BYTE; 32],
}
unsafe_impl_default_zeroed!(PAINTSTRUCT);
#[repr(C)]
pub struct RECT {
  left: LONG,
  top: LONG,
  right: LONG,
  bottom: LONG,
}
unsafe_impl_default_zeroed!(RECT);

#[repr(C)]
pub struct CREATESTRUCTW {
  lpCreateParams: LPVOID,
  hInstance: HINSTANCE,
  hMenu: HMENU,
  hwndParent: HWND,
  cy: c_int,
  cx: c_int,
  y: c_int,
  x: c_int,
  style: LONG,
  lpszName: LPCWSTR,
  lpszClass: LPCWSTR,
  dwExStyle: DWORD,
}
unsafe_impl_default_zeroed!(CREATESTRUCTW);

#[link(name = "Kernel32")]
extern "system" {
  /// [`GetModuleHandleW`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
  pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
  pub fn GetLastError() -> DWORD;
}

#[link(name = "User32")]
extern "system" {
    pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> ATOM;
    pub fn CreateWindowExW(dwExStyle: DWORD, lpClassName: LPCWSTR, lpWindowName: LPCWSTR, dwStyle: DWORD, X: c_int, Y: c_int, nWidth: c_int, nHeight: c_int, hWndParent: HWND, hMenu: HMENU, hInstance: HINSTANCE, lpParam: LPVOID,) -> HWND;
    pub fn ShowWindow(hWnd:HWND, nCmdShow:c_int) -> BOOL;
    pub fn DefWindowProcW( hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM,) -> LRESULT;
    pub fn GetMessageW(lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT,) -> BOOL;
    pub fn TranslateMessage(lpMsg: *const MSG) -> BOOL;
    pub fn DispatchMessageW(lpMsg: *const MSG) -> LRESULT;

    pub fn DestroyWindow(hWnd: HWND) -> BOOL;
    pub fn PostQuitMessage(nExitCode: c_int);
    pub fn LoadCursorW(hInstance: HINSTANCE, lpCursorName: LPCWSTR) -> HCURSOR;

    pub fn BeginPaint(hWnd: HWND, lpPaint: LPPAINTSTRUCT) -> HDC;
    pub fn FillRect(hDC: HDC, lprc: *const RECT, hbr: HBRUSH) -> c_int;
    pub fn EndPaint(hWnd: HWND, lpPaint: *const PAINTSTRUCT) -> BOOL;

    pub fn MessageBoxW(hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: UINT,) -> c_int;

    pub fn GetWindowLongPtrW(hWnd: HWND, nIndex: c_int) -> LONG_PTR;
    pub fn SetWindowLongPtrW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR,) -> LONG_PTR;

    pub fn SetCursor(hCursor: HCURSOR) -> HCURSOR;
}

pub fn wide_null(string:&str ) -> Vec<u16> {
  return string.encode_utf16().chain(Some(0)).collect();
}

pub const fn MAKEINTRESOURCEW(i: WORD) -> LPWSTR {
  i as ULONG_PTR as LPWSTR
}
const IDC_ARROW: LPCWSTR = MAKEINTRESOURCEW(32512);

pub unsafe extern "system" fn window_procedure( hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
  match Msg {
    WM_CLOSE =>  drop(DestroyWindow(hWnd)),
    WM_DESTROY => {
      let ptr = GetWindowLongPtrW(hWnd, GWLP_USERDATA) as *mut i32;
      Box::from_raw(ptr);
      println!("Cleaned up the box.");
      PostQuitMessage(0);
    }
    WM_PAINT => {
      let ptr = GetWindowLongPtrW(hWnd, GWLP_USERDATA) as *mut i32;
      println!("Current ptr: {}", *ptr);
      *ptr += 1;
      let mut ps = PAINTSTRUCT::default();
      let hdc = BeginPaint(hWnd, &mut ps);
      let _success = FillRect(hdc, &ps.rcPaint, (COLOR_WINDOW + 1) as HBRUSH);
      EndPaint(hWnd, &ps);
    }
    WM_NCCREATE => {
      println!("NC Create");
      let createstruct: *mut CREATESTRUCTW = lParam as *mut _;
      if createstruct.is_null() {
        return 0;
      }
      let boxed_i32_ptr: *mut i32 = (*createstruct).lpCreateParams.cast();
      SetWindowLongPtrW(hWnd, GWLP_USERDATA, boxed_i32_ptr as LONG_PTR);
      return 1;
    }
    WM_CREATE => println!("Create"),
    _ => return DefWindowProcW(hWnd, Msg, wParam, lParam),
  }
  0
}



fn main() {

    let hInstance = unsafe { GetModuleHandleW(core::ptr::null()) };
    let window_class_name = wide_null("Some Window Class");
    let window_name = wide_null("Some Window Name");

    let mut wc:WNDCLASSW = WNDCLASSW::default();
    wc.lpfnWndProc = Some(window_procedure);
    wc.hInstance = hInstance;
    wc.lpszClassName = window_class_name.as_ptr();
    wc.hCursor = unsafe { LoadCursorW(null_mut(), IDC_ARROW) };
    let return_result:ATOM= unsafe { RegisterClassW(&wc) } ;
    if return_result == 0 {
      let last_error = unsafe { GetLastError() };
      panic!("Error code failed creating window {} ", last_error);
    }

    let lparam: *mut i32 = Box::leak(Box::new(5_i32));

    let hwnd = unsafe {
        CreateWindowExW(
          0, 
          window_class_name.as_ptr(), 
          window_name.as_ptr(), 
          WS_OVERLAPPEDWINDOW,
          WINDOW_START_POS_X as i32,
          WINDOW_START_POS_Y as i32,
          WINDOW_WIDTH as i32,
          WINDOW_HEIGHT as i32,
          null_mut(),
          null_mut(),
          hInstance,
          lparam.cast())
    };

    if hwnd.is_null() {
      panic!("failed creating window");
    }

    let _previously_visible = unsafe { ShowWindow(hwnd, SW_SHOW) };

    let mut msg = MSG::default();
    loop {
      let message_return = unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) };
      if message_return == 0 {
        break;
      } else if message_return == -1 {
        let last_error = unsafe { GetLastError() };
        panic!("Error with `GetMessageW`, error code: {}", last_error);
      } else {
        unsafe {
          TranslateMessage(&msg);
          DispatchMessageW(&msg);
        }
      }
    }
}
