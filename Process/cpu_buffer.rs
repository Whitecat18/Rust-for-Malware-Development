/*
 * Increase CPU Usage while using multiple buffers
 * Execute at your own Risk
 * @5mukx
*/

use std::{f32::consts::PI, ptr::null_mut};
use winapi::shared::minwindef::TRUE;
use winapi::{ctypes::c_void, 
    shared::{minwindef::{BOOL, FALSE, LPARAM, LPVOID, LRESULT, UINT, WPARAM}, 
    windef::{HBITMAP, HBRUSH, HDC, HGDIOBJ, HWND}}, 
    um::{libloaderapi::GetModuleHandleW, 
    synchapi::Sleep, 
    wingdi::{BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetObjectW, GetStockObject, SelectObject, BITMAP, BLACK_BRUSH, SRCCOPY}, 
    winuser::{CreateWindowExW, DefWindowProcW, DispatchMessageW, FillRect, GetDC, GetMessageW, LoadCursorW, LoadIconW, LoadImageW, PeekMessageW, PostQuitMessage, RegisterClassExW, ReleaseDC, SetRect, ShowWindow, UpdateWindow, CW_USEDEFAULT, IDC_ARROW, IDI_APPLICATION, IMAGE_BITMAP, LR_DEFAULTSIZE, LR_LOADFROMFILE, PM_NOREMOVE, SW_SHOW, VK_PAUSE, WM_CREATE, WM_DESTROY, WM_ERASEBKGND, WM_KEYDOWN, WM_KILLFOCUS, WM_NCLBUTTONDOWN, WM_PAINT, WM_SETFOCUS, WNDCLASSEXW, WS_MAXIMIZEBOX, WS_THICKFRAME}
    }
};
use winapi::um::timeapi::timeGetTime;
use winapi::um::winuser::WS_OVERLAPPEDWINDOW;
extern crate winapi;
use winapi::um::winuser::MSG;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::windef::RECT;
const WINDOW_CLASS_NAME: &str = "TESTCLASS";
const WINDOW_TITLE: &str = "GDI Demo by Smukx";
const WINDOW_WIDTH: i32 = 405;
const WINDOW_HEIGHT: i32 = 290;


struct Backbuffer {
    hwnd: HWND,
    hdc: HDC,
    hdc2: HDC,
    hbmp: HBITMAP,
    hbmp2: HBITMAP,
    hbmp_prev: HBITMAP,
    hbmp_prev2: HBITMAP,
    hbr: HBRUSH,
    cx: i32,
    cy: i32,
}

struct Object {
    hdc: HDC,
    hbmp: HBITMAP,
    hbmp_prev: HBITMAP,
    amplitude: f32,
    frequency: f32,
    offset: f32,
    x: i32,
    y: i32,
    cx: i32,
    cy: i32,
}

static mut G_PAUSE: bool = false;
static mut G_LOST_FOCUS: BOOL = FALSE;
static mut G_RUNNING: BOOL = FALSE;
static mut G_BACKBUFFER: Backbuffer = Backbuffer {
    hwnd: null_mut(),
    hdc: null_mut(),
    hdc2: null_mut(),
    hbmp: null_mut(),
    hbmp2: null_mut(),
    hbmp_prev: null_mut(),
    hbmp_prev2: null_mut(),
    hbr: null_mut(),
    cx: 0,
    cy: 0,
};

static mut G_BACKGROUND: Object = Object {
    hdc: null_mut(),
    hbmp: null_mut(),
    hbmp_prev: null_mut(),
    amplitude: 0.0,
    frequency: 0.0,
    offset: 0.0,
    x: 0,
    y: 0,
    cx: 0,
    cy: 0,
};

fn main(){
    unsafe {
        let h_instance = GetModuleHandleW(null_mut());

        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as UINT,
            style: 0,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: h_instance,
            hIcon: LoadIconW(null_mut(), IDI_APPLICATION),
            hCursor: LoadCursorW(null_mut(), IDC_ARROW),
            hbrBackground: null_mut(),
            lpszMenuName: null_mut(),
            lpszClassName: to_wstring(WINDOW_CLASS_NAME).as_ptr(),
            hIconSm: LoadIconW(null_mut(), IDI_APPLICATION),
        };

        RegisterClassExW(&wc);

        let hwnd = CreateWindowExW(
            0,
            to_wstring(WINDOW_CLASS_NAME).as_ptr(),
            to_wstring(WINDOW_TITLE).as_ptr(),
            WS_OVERLAPPEDWINDOW & !WS_THICKFRAME & !WS_MAXIMIZEBOX,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            null_mut(),
            null_mut(),
            h_instance,
            null_mut(),
        );

        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);

        run();

    }
}

// extern system into the rod


extern "system" fn window_proc(hwnd: HWND, u_msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    unsafe {
        match u_msg {
            WM_CREATE => {
                if !create_backbuffer(hwnd) || !create_objects(hwnd) {
                    return -1;
                }
            }
            WM_KEYDOWN => {
                if w_param == VK_PAUSE as WPARAM {
                    G_PAUSE = !G_PAUSE;
                    if G_PAUSE {
                        show();
                    }
                }
            }
            WM_PAINT | WM_NCLBUTTONDOWN => show(),
            WM_ERASEBKGND => return 0,
            WM_SETFOCUS => G_LOST_FOCUS = FALSE,
            WM_KILLFOCUS => G_LOST_FOCUS = TRUE,
            WM_DESTROY => {
                destroy_backbuffer();
                destroy_objects();
                PostQuitMessage(0);
            }
            _ => {}
        }
        DefWindowProcW(hwnd, u_msg, w_param, l_param)
    }
}

unsafe fn run() -> i32 {
    let mut msg: MSG = std::mem::zeroed();
    let mut b_move = 1;
    let dw_interval = 15;
    let mut dw_next_time = timeGetTime();

    G_RUNNING = TRUE;

    while G_RUNNING != 0 {
        if G_PAUSE != false || G_LOST_FOCUS != 0 || PeekMessageW(&mut msg, null_mut(), 0, 0, PM_NOREMOVE) != 0 {
            if GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
                DispatchMessageW(&msg);
            } else {
                break;
            }
        } else {
            if b_move != 0 {
                update();
                render();
                b_move = FALSE;
            }

            let dw_cur_time = timeGetTime();

            if dw_cur_time > dw_next_time {
                show();
                dw_next_time += dw_interval;

                if dw_next_time < dw_cur_time {
                    dw_next_time = dw_cur_time + dw_interval;
                }
                b_move = 1; // TRUE
            } else {
                Sleep(dw_next_time - dw_cur_time);
            }
        }
    }

    msg.wParam as i32
}


unsafe fn create_backbuffer(hwnd: HWND) -> bool {
    let hdc = GetDC(hwnd);
    G_BACKBUFFER.hdc = CreateCompatibleDC(hdc);
    G_BACKBUFFER.hbmp = CreateCompatibleBitmap(hdc, 640, 480);
    G_BACKBUFFER.hdc2 = CreateCompatibleDC(hdc);
    G_BACKBUFFER.hbmp2 = CreateCompatibleBitmap(hdc, 640, 480);

    if G_BACKBUFFER.hbmp.is_null() {
        ReleaseDC(hwnd, hdc);
        return false;
    }

    G_BACKBUFFER.hbmp_prev = SelectObject(G_BACKBUFFER.hdc, G_BACKBUFFER.hbmp as *mut c_void) as HBITMAP;
    G_BACKBUFFER.hbmp_prev2 = SelectObject(G_BACKBUFFER.hdc2, G_BACKBUFFER.hbmp2 as *mut c_void) as HBITMAP;
    ReleaseDC(hwnd, hdc);

    G_BACKBUFFER.hwnd = hwnd;
    G_BACKBUFFER.hbr = GetStockObject(BLACK_BRUSH as i32) as HBRUSH;
    G_BACKBUFFER.cx = 640;
    G_BACKBUFFER.cy = 480;

    true // TRUE
}

unsafe fn destroy_backbuffer() {
    if !G_BACKBUFFER.hdc.is_null() && !G_BACKBUFFER.hdc2.is_null() {
        if !G_BACKBUFFER.hbmp.is_null() && !G_BACKBUFFER.hbmp2.is_null() {
            SelectObject(G_BACKBUFFER.hdc2, G_BACKBUFFER.hbmp_prev2 as *mut c_void);
            DeleteObject(G_BACKBUFFER.hbmp2 as *mut c_void);
            SelectObject(G_BACKBUFFER.hdc, G_BACKBUFFER.hbmp_prev as *mut c_void);
            DeleteObject(G_BACKBUFFER.hbmp as *mut c_void);
        }
        DeleteDC(G_BACKBUFFER.hdc2);
        DeleteDC(G_BACKBUFFER.hdc);
    }
}

unsafe fn create_objects(hwnd: HWND) -> bool {
    let hdc = GetDC(hwnd);
    G_BACKGROUND.hdc = CreateCompatibleDC(hdc);
    G_BACKGROUND.hbmp = LoadImageW(null_mut(), to_wstring("pic.bmp").as_ptr(), IMAGE_BITMAP, 0, 0, LR_LOADFROMFILE | LR_DEFAULTSIZE) as HBITMAP;

    if G_BACKGROUND.hbmp.is_null() {
        ReleaseDC(hwnd, hdc);
        return false;
    }

    G_BACKGROUND.hbmp_prev = SelectObject(G_BACKGROUND.hdc, G_BACKGROUND.hbmp as *mut c_void) as HBITMAP;
    G_BACKGROUND.amplitude = 10.0;
    G_BACKGROUND.frequency = 1.0 * ((2.0 * PI) / 320.0);

    let mut bm: BITMAP = std::mem::zeroed();
    GetObjectW(G_BACKGROUND.hbmp as HGDIOBJ, std::mem::size_of::<BITMAP>() as i32, &mut bm as *mut _ as LPVOID);
    G_BACKGROUND.x = 0;
    G_BACKGROUND.y = 0;
    G_BACKGROUND.cx = bm.bmWidth + 320;
    G_BACKGROUND.cy = bm.bmHeight;

    ReleaseDC(hwnd, hdc);

    true
}

unsafe fn destroy_objects() {
    if !G_BACKGROUND.hdc.is_null() {
        if !G_BACKGROUND.hbmp.is_null() {
            SelectObject(G_BACKGROUND.hdc, G_BACKGROUND.hbmp_prev as *mut c_void);
            DeleteObject(G_BACKGROUND.hbmp as *mut c_void);
        }
        DeleteDC(G_BACKGROUND.hdc);
    }
}

unsafe fn update() {
    distort_bitmap();
}

unsafe fn render() {
    let mut rc: RECT = std::mem::zeroed();
    SetRect(&mut rc, 0, 0, G_BACKBUFFER.cx, G_BACKBUFFER.cy);
    FillRect(G_BACKBUFFER.hdc, &rc, G_BACKBUFFER.hbr);
    FillRect(G_BACKBUFFER.hdc2, &rc, G_BACKBUFFER.hbr);
}

unsafe fn show() {
    let hdc = GetDC(G_BACKBUFFER.hwnd);
    BitBlt(hdc, 0, 0, G_BACKBUFFER.cx, G_BACKBUFFER.cy, G_BACKBUFFER.hdc2, 0, 0, SRCCOPY);
    ReleaseDC(G_BACKBUFFER.hwnd, hdc);
}

unsafe fn distort_bitmap() {
    for i in 0..G_BACKGROUND.cy {
        let stretch_factor = (G_BACKGROUND.amplitude * (G_BACKGROUND.frequency * (i as f32 + G_BACKGROUND.offset)).sin()) as i32;
        BitBlt(
            G_BACKBUFFER.hdc,
            (G_BACKGROUND.frequency.sin() * PI + 80.0) as i32,
            i + stretch_factor + 80,
            320,
            2,
            G_BACKGROUND.hdc,
            0,
            i,
            SRCCOPY,
        );
        G_BACKGROUND.offset += 0.2;
        if G_BACKGROUND.offset >= G_BACKGROUND.cx as f32 {
            G_BACKGROUND.offset = 0.0;
        }
    }

    for i in 0..G_BACKGROUND.cx {
        let stretch_factor = (G_BACKGROUND.amplitude * (G_BACKGROUND.frequency * (i as f32 + G_BACKGROUND.offset)).sin()) as i32;
        BitBlt(
            G_BACKBUFFER.hdc2,
            G_BACKGROUND.x + i + stretch_factor + 80,
            G_BACKGROUND.y + 80,
            2,
            255,
            G_BACKBUFFER.hdc,
            i,
            0,
            SRCCOPY,
        );
        G_BACKGROUND.offset += 0.2;
        if G_BACKGROUND.offset >= G_BACKGROUND.cx as f32 {
            G_BACKGROUND.offset = 0.0;
        }
    }
}

fn to_wstring(str: &str) -> Vec<u16> {
    std::ffi::OsString::from(str)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

