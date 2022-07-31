
// at the top of the file
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::os::raw::{c_ulong, c_ushort};
use core::ptr::{null, null_mut};

pub mod win32;
use RustTriangleFromScratch::win32::*;

pub unsafe extern "system" fn window_procedure( hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
  match Msg {
    self::WM_CLOSE =>  drop(DestroyWindow(hWnd)),
    self::WM_DESTROY => {
      match get_window_userdata::<i32>(hWnd) {
        Ok(ptr) if !ptr.is_null() => {
          Box::from_raw(ptr);
          println!("Cleaned up the box.");
        }
        Ok(_) => {
          println!("userdata ptr is null, no cleanup")
        }
        Err(e) => {
          println!("Error while getting the userdata ptr to clean it up: {}", e)
        }
      }
      post_quit_message();
    }
    self::WM_PAINT => {
      match get_window_userdata::<i32>(hWnd) {
        Ok(ptr) if !ptr.is_null() => {
          println!("Current ptr: {}", *ptr);
          *ptr += 1;
        }
        Ok(_) => {
          println!("userdata ptr is null")
        }
        Err(e) => {
          println!("Error while getting the userdata ptr: {}", e)
        }
      }
      let paint_result = match begin_paint(hWnd) 
      {
        Ok(result) => result,
        Err(error) => panic!("Failed begining paint {}", error),
      };
      let _success = FillRect(paint_result.0, &paint_result.1.rcPaint, (COLOR_WINDOW + 1) as HBRUSH);
      EndPaint(hWnd, &paint_result.1);
    }
    self::WM_NCCREATE => {
      println!("NC Create");
      let createstruct: *mut CREATESTRUCTW = lParam as *mut _;
      if createstruct.is_null() {
        return 0;
      }
      let ptr = (*createstruct).lpCreateParams as *mut i32;
      return set_window_userdata::<i32>(hWnd, ptr).is_ok() as LRESULT;
    }
    self::WM_CREATE => println!("Create"),
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
    wc.hCursor = load_predefined_cursor(IDCursor::Arrow).unwrap();
    let return_result:ATOM= register_class(&wc).unwrap_or_else(|e| {
      panic!("Error when registering class {}", e);
    }) ;
  
    let lparam: *mut i32 = Box::leak(Box::new(5_i32));
    let hwnd = 
    match create_window_ex_w( 
      "Some Window Class", 
    "Some Window Name",
  Some([WINDOW_START_POS_X as i32, WINDOW_START_POS_Y as i32]),
  [WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32],
  lparam.cast()) 
  {
    Ok(hwnd) => hwnd,
    Err(error) => panic!("Failed creating window {}", error),
  };

  let _previously_visible = unsafe { ShowWindow(hwnd, SW_SHOW) };
  loop {
    match get_any_message() {
      Ok(msg) => {
        if msg.message == WM_QUIT {
          break;
        }
        translate_message(&msg);
        unsafe
        {
          DispatchMessageW(&msg);
        }
      }
      Err(msg) => panic!("Error message {}", msg),
    }
  }
}
