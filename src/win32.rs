// at the top of the file
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::os::raw::{c_ushort};
use core::ptr::{null, null_mut};
use core::mem::{size_of};

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
pub const WS_EX_APPWINDOW:u32 =       0x00040000;
pub const WS_EX_WINDOWEDGE:u32 =      0x00000100;
pub const WS_EX_CLIENTEDGE:u32 =      0x00000200;
pub const WS_EX_OVERLAPPEDWINDOW:u32 = (WS_EX_WINDOWEDGE | WS_EX_CLIENTEDGE);

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
pub const WM_QUIT: u32 = 0x0012;
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


/// Allocates a unique device context for each window in the class.
pub const CS_OWNDC: u32 = 0x0020;

/// Redraws the entire window if a movement or size adjustment changes the width
/// of the client area.
pub const CS_HREDRAW: u32 = 0x0002;

/// Redraws the entire window if a movement or size adjustment changes the
/// height of the client area.
pub const CS_VREDRAW: u32 = 0x0001;

/// [`PIXELFORMATDESCRIPTOR`] pixel type
pub const PFD_TYPE_RGBA: u8 = 0;
/// [`PIXELFORMATDESCRIPTOR`] pixel type
pub const PFD_TYPE_COLORINDEX: u8 = 1;

/// [`PIXELFORMATDESCRIPTOR`] layer type
pub const PFD_MAIN_PLANE: u8 = 0;
/// [`PIXELFORMATDESCRIPTOR`] layer type
pub const PFD_OVERLAY_PLANE: u8 = 1;
/// [`PIXELFORMATDESCRIPTOR`] layer type
pub const PFD_UNDERLAY_PLANE: u8 = u8::MAX /* was (-1) */;

pub const PFD_DOUBLEBUFFER: u32 = 0x00000001;
pub const PFD_STEREO: u32 = 0x00000002;
pub const PFD_DRAW_TO_WINDOW: u32 = 0x00000004;
pub const PFD_DRAW_TO_BITMAP: u32 = 0x00000008;
pub const PFD_SUPPORT_GDI: u32 = 0x00000010;
pub const PFD_SUPPORT_OPENGL: u32 = 0x00000020;
pub const PFD_GENERIC_FORMAT: u32 = 0x00000040;
pub const PFD_NEED_PALETTE: u32 = 0x00000080;
pub const PFD_NEED_SYSTEM_PALETTE: u32 = 0x00000100;
pub const PFD_SWAP_EXCHANGE: u32 = 0x00000200;
pub const PFD_SWAP_COPY: u32 = 0x00000400;
pub const PFD_SWAP_LAYER_BUFFERS: u32 = 0x00000800;
pub const PFD_GENERIC_ACCELERATED: u32 = 0x00001000;
pub const PFD_SUPPORT_DIRECTDRAW: u32 = 0x00002000;
pub const PFD_DIRECT3D_ACCELERATED: u32 = 0x00004000;
pub const PFD_SUPPORT_COMPOSITION: u32 = 0x00008000;

/// use with [`ChoosePixelFormat`] only
pub const PFD_DEPTH_DONTCARE: u32 = 0x20000000;
/// use with [`ChoosePixelFormat`] only
pub const PFD_DOUBLEBUFFER_DONTCARE: u32 = 0x40000000;
/// use with [`ChoosePixelFormat`] only
pub const PFD_STEREO_DONTCARE: u32 = 0x80000000;

#[repr(C)]
#[derive(Debug)]
pub struct PIXELFORMATDESCRIPTOR {
  pub nSize: WORD,
  pub nVersion: WORD,
  pub dwFlags: DWORD,
  pub iPixelType: BYTE,
  pub cColorBits: BYTE,
  pub cRedBits: BYTE,
  pub cRedShift: BYTE,
  pub cGreenBits: BYTE,
  pub cGreenShift: BYTE,
  pub cBlueBits: BYTE,
  pub cBlueShift: BYTE,
  pub cAlphaBits: BYTE,
  pub cAlphaShift: BYTE,
  pub cAccumBits: BYTE,
  pub cAccumRedBits: BYTE,
  pub cAccumGreenBits: BYTE,
  pub cAccumBlueBits: BYTE,
  pub cAccumAlphaBits: BYTE,
  pub cDepthBits: BYTE,
  pub cStencilBits: BYTE,
  pub cAuxBuffers: BYTE,
  pub iLayerType: BYTE,
  pub bReserved: BYTE,
  pub dwLayerMask: DWORD,
  pub dwVisibleMask: DWORD,
  pub dwDamageMask: DWORD,
}

impl Default for PIXELFORMATDESCRIPTOR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    let mut out: Self = unsafe { core::mem::zeroed() };
    out.nSize = core::mem::size_of::<Self>() as WORD;
    out.nVersion = 1;
    out
  }
}

/// See [`GetSysColor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor)
pub enum SysColor 
{
  _3dDarkShadow = 21,
  _3dLight = 22,
  ActiveBorder = 10,
  ActiveCaption = 2,
  AppWorkspace = 12,
  /// Button face, also "3D face" color.
  ButtonFace = 15,
  /// Button highlight, also "3D highlight" color.
  ButtonHighlight = 20,
  /// Button shadow, also "3D shadow" color.
  ButtonShadow = 16,
  ButtonText = 18,
  CaptionText = 9,
  /// Desktop background color
  Desktop = 1,
  GradientActiveCaption = 27,
  GradientInactiveCaption = 28,
  GrayText = 17,
  Highlight = 13,
  HighlightText = 14,
  HotLight = 26,
  InactiveBorder = 11,
  InactiveCaption = 3,
  InactiveCaptionText = 19,
  InfoBackground = 24,
  InfoText = 23,
  Menu = 4,
  MenuHighlight = 29,
  MenuBar = 30,
  MenuText = 7,
  ScrollBar = 0,
  Window = 5,
  WindowFrame = 6,
  WindowText = 8,
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

#[repr(transparent)]
pub struct Win32Error(pub DWORD);

impl std::error::Error for Win32Error {}

impl core::fmt::Debug for Win32Error {
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

impl core::fmt::Display for Win32Error {
  /// Same as `Debug` impl
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{:?}", self)
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
   pub fn SetLastError(dwErrCode: DWORD);
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

    pub fn GetDC(hWnd: HWND) -> HDC;
    pub fn ReleaseDC(hWnd: HWND, hDC: HDC) -> c_int;
    pub fn UnregisterClassW(lpClassName: LPCWSTR, hInstance: HINSTANCE) -> BOOL;
}

#[link(name = "Gdi32")]
extern "system" {
  pub fn ChoosePixelFormat(hdc: HDC, ppfd: *const PIXELFORMATDESCRIPTOR,) -> c_int;
  pub fn SetPixelFormat(hdc: HDC, format: c_int, ppfd: *const PIXELFORMATDESCRIPTOR,) -> BOOL;
  pub fn DescribePixelFormat(hdc:HDC, iPixelFormat:c_int, nBytes:UINT,ppfd:*const PIXELFORMATDESCRIPTOR,) -> c_int;
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


/// Translates virtual-key messages into character messages.
///
/// The character messages go into your thread's message queue,
/// and you'll see them if you continue to consume messages.
///
/// **Returns:**
/// * `true` if the message was `WM_KEYDOWN`, `WM_KEYUP`, `WM_SYSKEYDOWN`, or
///   `WM_SYSKEYUP`.
/// * `true` for any other message type that generated a character message.
/// * otherwise `false`
///
/// See [`TranslateMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage)
pub fn translate_message(msg: &MSG) -> bool {
  0 != unsafe { TranslateMessage(msg) }
}

#[inline(always)]
pub fn get_any_message() -> Result<MSG, Win32Error> {
  let mut msg = MSG::default();
  let output = unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) };
  if output == -1 {
    Err(Win32Error(get_last_error()))
  } else {
    Ok(msg)
  }
}

pub fn post_quit_message() {
  unsafe { PostQuitMessage(0) }
}

/// See [`EndPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint)
pub unsafe fn end_paint(hwnd: HWND, ps: &PAINTSTRUCT) {
  EndPaint(hwnd, ps);
}


pub fn begin_paint(hWnd: HWND) -> Result<(HDC, PAINTSTRUCT), Win32Error> {
  let mut ps = PAINTSTRUCT::default();
  let result =  unsafe { BeginPaint(hWnd, &mut ps) };
  if result.is_null() {
    Err(Win32Error(get_last_error()))
  } else {
    Ok((result, ps))
  }
}



/// Sets the "userdata" pointer of the window (`GWLP_USERDATA`).
///
/// **Returns:** The previous userdata pointer.
///
/// [`SetWindowLongPtrW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw)
pub unsafe fn set_window_userdata<T>(
  hwnd: HWND, ptr: *mut T,
) -> Result<*mut T, Win32Error> {
  set_last_error(Win32Error(0));
  let out = SetWindowLongPtrW(hwnd, GWLP_USERDATA, ptr as LONG_PTR);
  if out == 0 {
    // if output is 0, it's only a "real" error if the last_error is non-zero
    let last_error = get_last_error();
    if last_error != 0 {
      Err(Win32Error(last_error))
    } else {
      Ok(out as *mut T)
    }
  } else {
    Ok(out as *mut T)
  }
}


/// Gets the "userdata" pointer of the window (`GWLP_USERDATA`).
///
/// **Returns:** The userdata pointer.
///
/// [`GetWindowLongPtrW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw)
pub unsafe fn get_window_userdata<T>(hwnd: HWND) -> Result<*mut T, Win32Error> {
  set_last_error(Win32Error(0));
  let out = GetWindowLongPtrW(hwnd, GWLP_USERDATA);
  if out == 0 {
    // if output is 0, it's only a "real" error if the last_error is non-zero
    let last_error = get_last_error();
    if last_error != 0 {
      Err(Win32Error(last_error))
    } else {
      Ok(out as *mut T)
    }
  } else {
    Ok(out as *mut T)
  }
}

/// Fills a rectangle with the given system color.
///
/// When filling the specified rectangle, this does **not** include the
/// rectangle's right and bottom sides. GDI fills a rectangle up to, but not
/// including, the right column and bottom row, regardless of the current
/// mapping mode.
///
/// [`FillRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-fillrect)
pub unsafe fn fill_rect_with_sys_color(
  hdc: HDC, rect: &RECT, color: SysColor,
) -> Result<(), ()> {
  if FillRect(hdc, rect, (color as u32 + 1) as HBRUSH) != 0 {
    Ok(())
  } else {
    Err(())
  }
}

/// Sets the thread-local last-error code value.
///
/// See [`SetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-setlasterror)
pub fn set_last_error(e: Win32Error) {
  unsafe { SetLastError(e.0) }
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
      WS_EX_APPWINDOW | WS_EX_OVERLAPPEDWINDOW,
      window_class_name.as_ptr(),
      window_name.as_ptr(),
      WS_OVERLAPPEDWINDOW | WS_CLIPCHILDREN | WS_CLIPSIBLINGS,
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

/// Performs [`begin_paint`] / [`end_paint`] around your closure.
pub unsafe fn do_some_painting<F, T>(hwnd: HWND, f: F) -> Result<T, Win32Error>
where
  F: FnOnce(HDC, bool, &RECT) -> Result<T, Win32Error>,
{
  let (hdc, ps) = begin_paint(hwnd)?;
  let output = f(hdc, ps.fErase != 0, &ps.rcPaint);
  end_paint(hwnd, &ps);
  output
}

/// See [`ChoosePixelFormat`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-choosepixelformat)
pub unsafe fn choose_pixel_format(
  hdc: HDC, ppfd: &PIXELFORMATDESCRIPTOR,
) -> Result<c_int, Win32Error> {
  let index = ChoosePixelFormat(hdc, ppfd);
  if index != 0 {
    Ok(index)
  } else {
    Err(Win32Error(get_last_error()))
  }
}

pub unsafe fn get_dc(hwnd: HWND) -> Option<HDC> {
  let hdc = GetDC(hwnd);
  if hdc.is_null() {
    None
  } else {
    Some(hdc)
  }
}

#[must_use]
pub unsafe fn release_dc(hwnd: HWND, hdc: HDC) -> bool {
  let was_released = ReleaseDC(hwnd, hdc);
  was_released != 0
}

pub unsafe fn destroy_window(hwnd: HWND) -> Result<(), Win32Error> {
  let destroyed = DestroyWindow(hwnd);
  if destroyed != 0 {
    Ok(())
  } else {
    Err(Win32Error(get_last_error()))
  }
}

/// Sets the pixel format of an HDC.
///
/// * If it's a window's HDC then it sets the pixel format of the window.
/// * You can't set a window's pixel format more than once.
/// * Call this *before* creating an OpenGL context.
/// * OpenGL windows should use [`WS_CLIPCHILDREN`] and [`WS_CLIPSIBLINGS`]
/// * OpenGL windows should *not* use `CS_PARENTDC`
///
/// See [`SetPixelFormat`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setpixelformat)
pub unsafe fn set_pixel_format(
  hdc: HDC, format: c_int, ppfd: &PIXELFORMATDESCRIPTOR,
) -> Result<(), Win32Error> {
  let success = SetPixelFormat(hdc, format, ppfd);
  if success != 0 {
    Ok(())
  } else {
    Err(Win32Error(get_last_error()))
  }
}


/// Gets the maximum pixel format index for the HDC.
///
/// Pixel format indexes are 1-based.
///
/// To print out info on all the pixel formats you'd do something like this:
/// ```no_run
/// # use triangle_from_scratch::win32::*;
/// let hdc = todo!("create a window to get an HDC");
/// let max = unsafe { get_max_pixel_format_index(hdc).unwrap() };
/// for index in 1..=max {
///   let pfd = unsafe { describe_pixel_format(hdc, index).unwrap() };
///   todo!("print the pfd info you want to know");
/// }
/// ```
///
/// See [`DescribePixelFormat`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-describepixelformat)
pub unsafe fn get_max_pixel_format_index(
  hdc: HDC,
) -> Result<c_int, Win32Error> {
  let max_index = DescribePixelFormat(
    hdc,
    1,
    size_of::<PIXELFORMATDESCRIPTOR>() as _,
    null_mut(),
  );
  if max_index == 0 {
    Err(Win32Error(get_last_error()))
  } else {
    Ok(max_index)
  }
}

/// Gets the pixel format info for a given pixel format index.
///
/// See [`DescribePixelFormat`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-describepixelformat)
pub unsafe fn describe_pixel_format(
  hdc: HDC, format: c_int,
) -> Result<PIXELFORMATDESCRIPTOR, Win32Error> {
  let mut pfd = PIXELFORMATDESCRIPTOR::default();
  let max_index = DescribePixelFormat(
    hdc,
    format,
    size_of::<PIXELFORMATDESCRIPTOR>() as _,
    &mut pfd,
  );
  if max_index == 0 {
    Err(Win32Error(get_last_error()))
  } else {
    Ok(pfd)
  }
}

/// Un-registers the window class from the `HINSTANCE` given.
///
/// * The name must be the name of a registered window class.
/// * This requires re-encoding the name to null-terminated utf-16, which
///   allocates. Using [`unregister_class_by_atom`] instead does not allocate,
///   if you have the atom available.
/// * Before calling this function, an application must destroy all windows
///   created with the specified class.
///
/// See
/// [`UnregisterClassW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterclassw)
pub unsafe fn unregister_class_by_name(
  name: &str, instance: HINSTANCE,
) -> Result<(), Win32Error> {
  let name_null = wide_null(name);
  let out = UnregisterClassW(name_null.as_ptr(), instance);
  if out != 0 {
    Ok(())
  } else {
    Err(Win32Error(get_last_error()))
  }
}

/// Un-registers the window class from the `HINSTANCE` given.
///
/// * The atom must be the atom of a registered window class.
/// * Before calling this function, an application must destroy all windows
///   created with the specified class.
///
/// See [`UnregisterClassW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterclassw)
pub unsafe fn unregister_class_by_atom(
  a: ATOM, instance: HINSTANCE,
) -> Result<(), Win32Error> {
  let out = UnregisterClassW(a as LPCWSTR, instance);
  if out != 0 {
    Ok(())
  } else {
    Err(Win32Error(get_last_error()))
  }
}