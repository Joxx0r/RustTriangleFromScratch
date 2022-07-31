// at the top of the file
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::os::raw::{c_ushort};
use core::ptr::{null, null_mut};

pub type c_int = i32;
pub type c_uint = u32;
pub type HANDLE = PVOID;
pub type HBRUSH = HANDLE;
pub type HCURSOR = HICON;
pub type HICON = HANDLE;
pub type HINSTANCE = HANDLE;
pub type HWND = HANDLE;
pub type LONG_PTR = isize;
pub type LPARAM = LONG_PTR;
pub type LPCWSTR = *const WCHAR;
pub type LRESULT = LONG_PTR;
pub type PVOID = *mut core::ffi::c_void;
pub type UINT = c_uint;
pub type UINT_PTR = usize;
pub type WCHAR = wchar_t;
pub type wchar_t = u16;
pub type LPWSTR = *mut WCHAR;
pub type ULONG_PTR = usize;
pub type WPARAM = UINT_PTR;
pub type HMODULE = HANDLE;
pub type DWORD = c_ulong;
pub type WORD = c_ushort;
pub type ATOM = WORD;
pub type HMENU = HANDLE;
pub type LPVOID = *mut core::ffi::c_void;
pub type BOOL = c_int;
pub type LONG = c_long;
pub type c_long = i32;
pub type LPMSG = *mut MSG;
pub type HDC = HANDLE;
pub type BYTE = u8;
pub type LPPAINTSTRUCT = *mut PAINTSTRUCT;
pub type LPCVOID = *const core::ffi::c_void;
pub type va_list = *mut c_char;
pub type c_char = i8;
pub type HLOCAL = HANDLE;
pub type c_ulong = u32;

pub const FORMAT_MESSAGE_ALLOCATE_BUFFER:u32=   0x00000100; 
pub const FORMAT_MESSAGE_IGNORE_INSERTS:u32 =   0x00000200;
pub const FORMAT_MESSAGE_FROM_STRING:u32 =      0x00000400;
pub const FORMAT_MESSAGE_FROM_HMODULE:u32 =     0x00000800;
pub const FORMAT_MESSAGE_FROM_SYSTEM:u32 =      0x00001000;
pub const FORMAT_MESSAGE_ARGUMENT_ARRAY:u32 =   0x00002000;
pub const FORMAT_MESSAGE_MAX_WIDTH_MASK:u32 =   0x000000FF;

pub const WINDOW_HEIGHT:f32  = 1024.0;
pub const WINDOW_WIDTH:f32  = 1024.0;
pub const WINDOW_START_POS_X:f32  = 0.0;
pub const WINDOW_START_POS_Y:f32  = 0.0;
pub const GWLP_USERDATA: c_int = -21;

/*
 * Window Styles
 */

/*
 * Window Styles
 */
pub const WS_OVERLAPPED:u32 =          0x00000000;
pub const WS_POPUP:u32 =              0x80000000;
pub const WS_CHILD:u32 =              0x40000000;
pub const WS_MINIMIZE:u32 =           0x20000000;
pub const WS_VISIBLE:u32 =            0x10000000;
pub const WS_DISABLED:u32 =           0x08000000;
pub const WS_CLIPSIBLINGS:u32 =       0x04000000;
pub const WS_CLIPCHILDREN:u32 =       0x02000000;
pub const WS_MAXIMIZE:u32 =           0x01000000;
pub const WS_CAPTION:u32 =            0x00C00000;    
pub const WS_BORDER:u32 =             0x00800000;
pub const WS_DLGFRAME:u32 =           0x00400000;
pub const WS_VSCROLL:u32 =            0x00200000;
pub const WS_HSCROLL:u32 =            0x00100000;
pub const WS_SYSMENU:u32 =            0x00080000;
pub const WS_THICKFRAME:u32 =         0x00040000;
pub const WS_GROUP:u32 =              0x00020000;
pub const WS_TABSTOP:u32 =            0x00010000;

pub const WS_MINIMIZEBOX:u32 =        0x00020000;
pub const WS_MAXIMIZEBOX:u32 =        0x00010000;

pub const WS_TILED:u32=         WS_OVERLAPPED;
pub const WS_ICONIC:u32=        WS_MINIMIZE;
pub const WS_SIZEBOX:u32=       WS_THICKFRAME;
pub const WS_TILEDWINDOW:u32=   WS_OVERLAPPEDWINDOW;

pub const WM_NCCREATE: u32 = 0x0081;
pub const WM_CREATE: u32 = 0x0001;

/*
 * Common Window Styles
 */
pub const WS_OVERLAPPEDWINDOW:u32 = 
                            (WS_OVERLAPPED | 
                             WS_CAPTION        | 
                             WS_SYSMENU        | 
                             WS_THICKFRAME     | 
                             WS_MINIMIZEBOX    | 
                             WS_MAXIMIZEBOX);

pub const CW_USEDEFAULT:c_int   = 0x8000000 as c_int;
pub const SW_SHOW: c_int = 5;
pub const WM_DESTROY: u32 = 0x0002;
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_SETCURSOR: u32 = 0x0020;
pub const WM_PAINT: u32 = 0x000F;
pub const COLOR_WINDOW: u32 = 5;

/// The predefined cursor styles.
pub enum IDCursor {
  /// Standard arrow and small hourglass
  AppStarting = 32650,
  /// Standard arrow
  Arrow = 32512,
  /// Crosshair
  Cross = 32515,
  /// Hand
  Hand = 32649,
  /// Arrow and question mark
  Help = 32651,
  /// I-beam
  IBeam = 32513,
  /// Slashed circle
  No = 32648,
  /// Four-pointed arrow pointing north, south, east, and west
  SizeAll = 32646,
  /// Double-pointed arrow pointing northeast and southwest
  SizeNeSw = 32643,
  /// Double-pointed arrow pointing north and south
  SizeNS = 32645,
  /// Double-pointed arrow pointing northwest and southeast
  SizeNwSe = 32642,
  /// Double-pointed arrow pointing west and east
  SizeWE = 32644,
  /// Vertical arrow
  UpArrow = 32516,
  /// Hourglass
  Wait = 32514,
}

pub fn load_predefined_cursor(cursor:IDCursor) -> Result<HCURSOR, Win32Error> {
  let hcursor =
    unsafe { LoadCursorW(null_mut(), MAKEINTRESOURCEW(cursor as WORD)) };
  if hcursor.is_null() {
    Err(Win32Error(get_last_error()))
  } else {
    Ok(hcursor)
  }
}

pub fn register_class(window_class:&WNDCLASSW) -> Result<ATOM, Win32Error> {
  let atom = unsafe { RegisterClassW(window_class) };
  if atom == 0 {
    Err((Win32Error(get_last_error())))
  } else {
    Ok(atom)
  }
}

/// Gets the thread-local last-error code value.
///
/// See [`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
pub fn get_last_error() -> DWORD {
  unsafe { GetLastError() }
}

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
  pub x: LONG,
  pub y: LONG,
}
unsafe_impl_default_zeroed!(POINT);

pub type WNDPROC = Option<
  unsafe extern "system" fn(
    hwnd: HWND,
    uMsg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
  ) -> LRESULT
>;


#[repr(C)]
pub struct WNDCLASSW {
  pub style: UINT,
  pub lpfnWndProc: WNDPROC,
  pub cbClsExtra: c_int,
  pub cbWndExtra: c_int,
  pub hInstance: HINSTANCE,
  pub hIcon: HICON,
  pub hCursor: HCURSOR,
  pub hbrBackground: HBRUSH,
  pub lpszMenuName: LPCWSTR,
  pub lpszClassName: LPCWSTR,
}
unsafe_impl_default_zeroed!(WNDCLASSW);

#[repr(C)]
pub struct MSG {
  pub hwnd: HWND,
  pub message: UINT,
  pub wParam: WPARAM,
  pub lParam: LPARAM,
  pub time: DWORD,
  pub pt: POINT,
  pub lPrivate: DWORD,
}
unsafe_impl_default_zeroed!(MSG);


#[repr(C)]
pub struct PAINTSTRUCT {
  pub hdc: HDC,
  pub fErase: BOOL,
  pub rcPaint: RECT,
  pub fRestore: BOOL,
  pub fIncUpdate: BOOL,
  pub rgbReserved: [BYTE; 32],
}
unsafe_impl_default_zeroed!(PAINTSTRUCT);
#[repr(C)]
pub struct RECT {
  pub left: LONG,
  pub top: LONG,
  pub right: LONG,
  pub bottom: LONG,
}
unsafe_impl_default_zeroed!(RECT);

#[repr(C)]
pub struct CREATESTRUCTW {
  pub lpCreateParams: LPVOID,
  pub hInstance: HINSTANCE,
  pub hMenu: HMENU,
  pub hwndParent: HWND,
  pub cy: c_int,
  pub cx: c_int,
  pub y: c_int,
  pub x: c_int,
  pub style: LONG,
  pub lpszName: LPCWSTR,
  pub lpszClass: LPCWSTR,
  pub dwExStyle: DWORD,
}
unsafe_impl_default_zeroed!(CREATESTRUCTW);

#[derive(Debug)]
#[repr(transparent)]
pub struct Win32Error(pub DWORD);

impl std::error::Error for Win32Error {}

impl core::fmt::Display for Win32Error {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if self.0 & (1 << 29) > 0 {
      return write!(f, "Win32ApplicationError({})", self.0);
    }
    let dwFlags = FORMAT_MESSAGE_ALLOCATE_BUFFER
    | FORMAT_MESSAGE_FROM_SYSTEM
    | FORMAT_MESSAGE_IGNORE_INSERTS;
    let lpSource = null_mut();
    let dwMessageId = self.0;
    let dwLanguageId = 0;
    let mut buffer: *mut u16 = null_mut();
    let lpBuffer = &mut buffer as *mut *mut u16 as *mut u16;
    let nSize = 0;
    let Arguments = null_mut();
    let tchar_count_excluding_null = unsafe {
      FormatMessageW(
        dwFlags,
        lpSource,
        dwMessageId,
        dwLanguageId,
        lpBuffer,
        nSize,
        Arguments,
      )
    };
    if tchar_count_excluding_null == 0 || buffer.is_null() {
      return Err(core::fmt::Error);
    }
    else {
      let _on_drop = OnDropLocalFree(buffer as HLOCAL);
      let buffer_slice: &[u16] = unsafe {
        core::slice::from_raw_parts(buffer, tchar_count_excluding_null as usize)
      };
      for decode_result in core::char::decode_utf16(buffer_slice.iter().copied()) 
      {
        match decode_result {
          Ok('\r') | Ok('\n') => write!(f, " ")?,
          Ok(ch) => write!(f, "{}", ch)?,
          Err(_) => write!(f, "�")?,
        }
      }
      Ok(())
    }
  }
}


#[repr(transparent)]
struct OnDropLocalFree(HLOCAL);

impl Drop for OnDropLocalFree {
  fn drop(&mut self) {
    unsafe { LocalFree(self.0) };
  }
}

#[link(name = "Kernel32")]
extern "system" {
  pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
  pub fn GetLastError() -> DWORD;
  pub fn FormatMessageW(dwFlags: DWORD, lpSource: LPCVOID, dwMessageId: DWORD, dwLanguageId: DWORD,lpBuffer: LPWSTR, nSize: DWORD, Arguments: va_list, ) -> DWORD;
  pub fn LocalFree(hMem: HLOCAL) -> HLOCAL;
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
pub const IDC_ARROW: LPCWSTR = MAKEINTRESOURCEW(32512);

/// Returns a handle to the file used to create the calling process (.exe file)
///
/// See [`GetModuleHandleW`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
pub fn get_process_handle() -> HMODULE {
  // Safety: as per the MSDN docs.
  unsafe { GetModuleHandleW(core::ptr::null()) }
}

pub fn create_window_ex_w( class_name: &str, window_name: &str, 
  position: Option<[i32;2]>, window_size:[i32;2],  create_param: LPVOID,
) -> Result<HWND, Win32Error> {
  
  let h_instance = get_process_handle();
  let window_class_name = wide_null(class_name);
  let window_name = wide_null(window_name);
  let hwnd = unsafe
  {
    CreateWindowExW(
      0,
      window_class_name.as_ptr(),
      window_name.as_ptr(),
      WS_OVERLAPPEDWINDOW,
      position.unwrap_or([CW_USEDEFAULT, CW_USEDEFAULT])[0],
      position.unwrap_or([CW_USEDEFAULT, CW_USEDEFAULT])[1],
      window_size[0],
      window_size[1],
      null_mut(),
      null_mut(),
      h_instance,
      create_param,
    )
  };
 
  if hwnd.is_null() {
    Err(Win32Error(get_last_error()))
  } else {
    Ok(hwnd)
  }
}