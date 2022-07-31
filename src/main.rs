
// at the top of the file
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::os::raw::{c_ulong, c_ushort};
use core::ptr::{null, null_mut};

pub mod win32;
use RustTriangleFromScratch::win32::*;

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
    let window_class_name = wide_null("Some Window Class");
    let hInstance = get_process_handle();
    let mut wc:WNDCLASSW = WNDCLASSW::default();
    wc.lpfnWndProc = Some(window_procedure);
    wc.hInstance = hInstance;
    wc.lpszClassName = window_class_name.as_ptr();
    //wc.hCursor = load_predefined_cursor(IDCursor::Arrow).unwrap();
    let return_result:ATOM= register_class(&wc).unwrap_or_else(|e| {
      panic!("Error when registering class {}", e);
    }) ;
  
    let lparam: *mut i32 = Box::leak(Box::new(5_i32));
    let hwnd_result = create_window_ex_w( 
      "Some Window Class", 
    "Some Window Name",
  Some([WINDOW_START_POS_X as i32, WINDOW_START_POS_Y as i32]),
  [WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32],
  lparam.cast());

    let hwnd = match hwnd_result {
      Ok(hwnd) => hwnd,
      Err(error) => panic!("Failed creating file {}", error),
    };
    if hwnd.is_null() {
      let error = Win32Error(get_last_error());
      panic!("failed creating window {}", error);
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
