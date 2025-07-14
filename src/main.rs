#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use std::ffi::c_void;
use std::os::raw::{c_int, c_long, c_uint, c_ulong};
use std::ptr::null_mut;
use std::mem::zeroed;

const PANEL_HEIGHT: i32 = 40;
const PANEL_PADDING_TOP: i32 = 0;
const DOT_RADIUS: i32 = 8;
const DOT_MARGIN: i32 = 8;
const DOT_MARGIN_RIGHT: i32 = 24;
const ICON_SIZE: i32 = 24;
const ICON_MARGIN_LEFT: i32 = 12;
const TEXT_MARGIN_LEFT: i32 = 8;
const WIN_W: i32 = 800;
const WIN_H: i32 = 600;
const RESIZE_BORDER_WIDTH: i32 = 8;

type HWND = *mut c_void;
type HINSTANCE = *mut c_void;
type HICON = *mut c_void;
type HCURSOR = *mut c_void;
type HBRUSH = *mut c_void;
type HMENU = *mut c_void;
type HDC = *mut c_void;
type HGDIOBJ = *mut c_void;
type LPARAM = isize;
type WPARAM = usize;
type LRESULT = isize;
type UINT = c_uint;
type DWORD = c_ulong;
type BOOL = c_int;
type ATOM = u16;
type LPVOID = *mut c_void;
type LPCWSTR = *const u16;
type LPMSG = *mut MSG;

const CS_HREDRAW: UINT = 0x0002;
const CS_VREDRAW: UINT = 0x0001;
const WS_POPUP: DWORD = 0x80000000;
const WS_VISIBLE: DWORD = 0x10000000;
const WS_THICKFRAME: DWORD = 0x00040000;
const WS_BORDER: u32 = 0x00800000;
const CW_USEDEFAULT: c_int = 0x80000000u32 as c_int;
const SW_MINIMIZE: c_int = 6;
const SW_MAXIMIZE: c_int = 3;
const SW_RESTORE: c_int = 9;
const SW_SHOW: c_int = 5;
const WM_CREATE: UINT = 0x0001;
const WM_DESTROY: UINT = 0x0002;
const WM_PAINT: UINT = 0x000F;
const WM_SIZE: UINT = 0x0005;
const WM_MOUSEMOVE: UINT = 0x0200;
const WM_LBUTTONUP: UINT = 0x0202;
const WM_NCHITTEST: UINT = 0x0084;
const WM_MOUSELEAVE: UINT = 0x02A3;
const HTCLIENT: c_int = 1;
const HTCAPTION: c_int = 2;
const HTLEFT: c_int = 10;
const HTRIGHT: c_int = 11;
const HTTOP: c_int = 12;
const HTTOPLEFT: c_int = 13;
const HTTOPRIGHT: c_int = 14;
const HTBOTTOM: c_int = 15;
const HTBOTTOMLEFT: c_int = 16;
const HTBOTTOMRIGHT: c_int = 17;
const GWLP_USERDATA: c_int = -21;
const NULL_BRUSH: c_int = 5;
const NULL_PEN: c_int = 8;
const COLOR_WINDOW: c_int = 5;
const TME_LEAVE: DWORD = 0x00000002;
const WM_NCCALCSIZE: UINT = 0x0083;

#[repr(C)]
struct WNDCLASSW {
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

type WNDPROC = Option<unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT>;

#[repr(C)]
struct MSG {
    hwnd: HWND,
    message: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
    time: DWORD,
    pt: POINT,
}

#[repr(C)]
struct POINT {
    x: c_long,
    y: c_long,
}

#[repr(C)]
struct RECT {
    left: c_long,
    top: c_long,
    right: c_long,
    bottom: c_long,
}

#[repr(C)]
struct PAINTSTRUCT {
    hdc: HDC,
    fErase: BOOL,
    rcPaint: RECT,
    fRestore: BOOL,
    fIncUpdate: BOOL,
    rgbReserved: [u8; 32],
}

#[repr(C)]
struct TRACKMOUSEEVENT {
    cbSize: DWORD,
    dwFlags: DWORD,
    hwndTrack: HWND,
    dwHoverTime: DWORD,
}

#[repr(C)]
struct WINDOWPLACEMENT {
    length: UINT,
    flags: UINT,
    showCmd: UINT,
    ptMinPosition: POINT,
    ptMaxPosition: POINT,
    rcNormalPosition: RECT,
}

struct WindowState {
    hover_dot: Option<usize>,
    tracking_mouse: bool,
}

fn rgb(r: u8, g: u8, b: u8) -> u32 {
    r as u32 | ((g as u32) << 8) | ((b as u32) << 16)
}

fn get_dots() -> [(usize, u32); 3] {
    [
        (0, rgb(255, 95, 86)),
        (1, rgb(255, 189, 46)),
        (2, rgb(39, 201, 63)),
    ]
}

fn to_wide(s: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    std::ffi::OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}

fn center_window(hwnd: HWND, width: i32, height: i32) {
    unsafe {
        let mut rect: RECT = zeroed();
        let desktop = GetDesktopWindow();
        if GetWindowRect(desktop, &mut rect) == 0 { return; }
        let screen_w = rect.right - rect.left;
        let screen_h = rect.bottom - rect.top;
        let x = (screen_w - width) / 2;
        let y = (screen_h - height) / 2;
        MoveWindow(hwnd, x, y, width, height, 1);
    }
}

fn main() {
    unsafe {
        let h_instance = GetModuleHandleW(null_mut());
        let class_name = to_wide("SolkaEngineClass");
        let icon_path = to_wide("icon.ico");
        let font_path = to_wide("Oregano-Regular.ttf");
        AddFontResourceExW(font_path.as_ptr(), 0x10, null_mut());
        let h_icon = LoadImageW(
            null_mut(),
            icon_path.as_ptr(),
            1,
            ICON_SIZE,
            ICON_SIZE,
            0x00000010,
        ) as HICON;
        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: h_instance,
            hIcon: h_icon,
            hCursor: null_mut(),
            hbrBackground: null_mut(),
            lpszMenuName: null_mut(),
            lpszClassName: class_name.as_ptr(),
        };
        if RegisterClassW(&wc) == 0 {
            eprintln!("Failed to register window class");
            RemoveFontResourceExW(font_path.as_ptr(), 0x10, null_mut());
            return;
        }
        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            to_wide("Solka Engine").as_ptr(),
            WS_POPUP | WS_VISIBLE | WS_BORDER,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            WIN_W,
            WIN_H,
            null_mut(),
            null_mut(),
            h_instance,
            null_mut(),
        );
        if hwnd == null_mut() {
            eprintln!("Failed to create window");
            RemoveFontResourceExW(font_path.as_ptr(), 0x10, null_mut());
            return;
        }
        SetClassLongPtrW(hwnd, -34, h_icon as isize);
        center_window(hwnd, WIN_W, WIN_H);
        let state = Box::new(WindowState {
            hover_dot: None,
            tracking_mouse: false,
        });
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(state) as isize);
        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);
        let mut msg: MSG = zeroed();
        while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
            TranslateMessage(&mut msg);
            DispatchMessageW(&mut msg);
        }
        RemoveFontResourceExW(font_path.as_ptr(), 0x10, null_mut());
    }
}

#[link(name = "user32")]
#[link(name = "gdi32")]
#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HINSTANCE;
    fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> ATOM;
    fn CreateWindowExW(dwExStyle: DWORD, lpClassName: LPCWSTR, lpWindowName: LPCWSTR, dwStyle: DWORD,
        X: c_int, Y: c_int, nWidth: c_int, nHeight: c_int, hWndParent: HWND, hMenu: HMENU, hInstance: HINSTANCE, lpParam: LPVOID) -> HWND;
    fn DefWindowProcW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;
    fn UpdateWindow(hWnd: HWND) -> BOOL;
    fn GetMessageW(lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> BOOL;
    fn TranslateMessage(lpMsg: LPMSG) -> BOOL;
    fn DispatchMessageW(lpMsg: LPMSG) -> LRESULT;
    fn PostQuitMessage(nExitCode: c_int);
    fn BeginPaint(hWnd: HWND, lpPaint: *mut PAINTSTRUCT) -> HDC;
    fn EndPaint(hWnd: HWND, lpPaint: *const PAINTSTRUCT) -> BOOL;
    fn GetClientRect(hWnd: HWND, lpRect: *mut RECT) -> BOOL;
    fn FillRect(hDC: HDC, lprc: *const RECT, hbr: HBRUSH) -> c_int;
    fn CreateSolidBrush(color: DWORD) -> HBRUSH;
    fn DeleteObject(hObject: HGDIOBJ) -> BOOL;
    fn SelectObject(hdc: HDC, h: HGDIOBJ) -> HGDIOBJ;
    fn GetStockObject(i: c_int) -> HGDIOBJ;
    fn Ellipse(hdc: HDC, left: c_int, top: c_int, right: c_int, bottom: c_int) -> BOOL;
    fn InvalidateRect(hWnd: HWND, lpRect: *const RECT, bErase: BOOL) -> BOOL;
    fn SetWindowLongPtrW(hWnd: HWND, nIndex: c_int, dwNewLong: isize) -> isize;
    fn GetWindowLongPtrW(hWnd: HWND, nIndex: c_int) -> isize;
    fn MoveWindow(hWnd: HWND, X: c_int, Y: c_int, nWidth: c_int, nHeight: c_int, bRepaint: BOOL) -> BOOL;
    fn GetDesktopWindow() -> HWND;
    fn GetWindowRect(hWnd: HWND, lpRect: *mut RECT) -> BOOL;
    fn TrackMouseEvent(ptme: *mut TRACKMOUSEEVENT) -> BOOL;
    fn ScreenToClient(hWnd: HWND, lpPoint: *mut POINT) -> BOOL;
    fn GetWindowPlacement(hWnd: HWND, lpwndpl: *mut WINDOWPLACEMENT) -> BOOL;
    fn LoadImageW(hInst: HINSTANCE, name: LPCWSTR, type_: UINT, cx: c_int, cy: c_int, fuLoad: UINT) -> HICON;
    fn DrawIconEx(hdc: HDC, xLeft: c_int, yTop: c_int, hIcon: HICON, cxWidth: c_int, cyWidth: c_int, istepIfAniCur: UINT, hbrFlickerFreeDraw: HBRUSH, diFlags: UINT) -> BOOL;
    fn SetClassLongPtrW(hWnd: HWND, nIndex: c_int, dwNewLong: isize) -> isize;
    fn GetDC(hWnd: HWND) -> HDC;
    fn ReleaseDC(hWnd: HWND, hDC: HDC) -> c_int;
    fn TextOutW(hdc: HDC, x: c_int, y: c_int, lpString: LPCWSTR, c: c_int) -> BOOL;
    fn SetTextColor(hdc: HDC, color: u32) -> u32;
    fn SetBkMode(hdc: HDC, mode: c_int) -> c_int;
    fn CreateFontW(height: c_int, width: c_int, escapement: c_int, orientation: c_int, weight: c_int, italic: c_int, underline: c_int, strikeout: c_int, charset: c_int, out_precision: c_int, clip_precision: c_int, quality: c_int, pitch_and_family: c_int, face: LPCWSTR) -> HGDIOBJ;
    fn AddFontResourceExW(lpszFilename: LPCWSTR, fl: DWORD, pdv: LPVOID) -> c_int;
    fn RemoveFontResourceExW(lpszFilename: LPCWSTR, fl: DWORD, pdv: LPVOID) -> c_int;
}

unsafe extern "system" fn window_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let state_ptr = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut WindowState };
    let state = unsafe { state_ptr.as_mut() };
    match msg {
        WM_CREATE => { 0 },
        0x0014 => {
            let hdc = wparam as HDC;
            let mut client_rect: RECT = unsafe { zeroed() };
            unsafe { GetClientRect(hwnd, &mut client_rect); }
            let bg_color = rgb(32, 34, 37);
            let bg_brush = unsafe { CreateSolidBrush(bg_color) };
            unsafe {
                FillRect(hdc, &client_rect, bg_brush);
                DeleteObject(bg_brush as _);
            }
            1
        },
        msg if msg == WM_NCCALCSIZE => { return 0; },
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = unsafe { zeroed() };
            let hdc = unsafe { BeginPaint(hwnd, &mut ps) };
            if hdc == null_mut() { return 0; }
            let mut client_rect: RECT = unsafe { zeroed() };
            unsafe { GetClientRect(hwnd, &mut client_rect); }
            let win_w = client_rect.right - client_rect.left;
            let _win_h = client_rect.bottom - client_rect.top;
            let bg_rect = RECT { left: 0, top: 0, right: win_w, bottom: _win_h };
            let bg_color = rgb(32, 34, 37);
            let bg_brush = unsafe { CreateSolidBrush(bg_color) };
            unsafe {
                FillRect(hdc, &bg_rect, bg_brush);
                DeleteObject(bg_brush as _);
            }
            let panel_color = rgb(54, 57, 63);
            let panel_brush = unsafe { CreateSolidBrush(panel_color) };
            let panel_rect = RECT { left: 0, top: 0, right: win_w, bottom: PANEL_HEIGHT };
            unsafe {
                FillRect(hdc, &panel_rect, panel_brush);
                DeleteObject(panel_brush as _);
            }
            let icon_path = to_wide("icon.ico");
            let h_icon = unsafe {
                LoadImageW(null_mut(), icon_path.as_ptr(), 1, ICON_SIZE, ICON_SIZE, 0x00000010)
            } as HICON;
            if h_icon != null_mut() {
                let icon_y = (PANEL_HEIGHT - ICON_SIZE) / 2;
                unsafe {
                    DrawIconEx(hdc, ICON_MARGIN_LEFT, icon_y, h_icon, ICON_SIZE, ICON_SIZE, 0, null_mut(), 0x0003);
                }
            }
            let text = to_wide("Solka Engine");
            let text_x = ICON_MARGIN_LEFT + ICON_SIZE + TEXT_MARGIN_LEFT;
            let text_y = (PANEL_HEIGHT - 22) / 2;
            let font_name = to_wide("Oregano");
            let h_font = unsafe {
                CreateFontW(
                    22, 0, 0, 0, 400, 0, 0, 0, 0, 0, 0, 5, 0, font_name.as_ptr()
                )
            };
            let old_font = unsafe { SelectObject(hdc, h_font) };
            unsafe {
                SetBkMode(hdc, 1);
                SetTextColor(hdc, rgb(255, 255, 255));
                TextOutW(hdc, text_x, text_y, text.as_ptr(), text.len() as i32 - 1);
                SelectObject(hdc, old_font);
                DeleteObject(h_font);
            }
            if let Some(state) = state {
                let dot_y = (PANEL_HEIGHT - DOT_RADIUS * 2) / 2;
                for (i, color) in get_dots().iter() {
                    let x = win_w - DOT_MARGIN_RIGHT - (DOT_RADIUS * 2 + DOT_MARGIN) * (3 - *i as i32);
                    let is_hover = state.hover_dot == Some(*i);
                    let dot_color = if is_hover {
                        rgb(
                            ((*color & 0xFF) as u8).saturating_sub(30),
                            (((*color >> 8) & 0xFF) as u8).saturating_sub(30),
                            (((*color >> 16) & 0xFF) as u8).saturating_sub(30),
                        )
                    } else {
                        *color
                    };
                    let brush = unsafe { CreateSolidBrush(dot_color) };
                    let old_brush = unsafe { SelectObject(hdc, brush as _) };
                    unsafe {
                        Ellipse(hdc, x, dot_y, x + DOT_RADIUS * 2, dot_y + DOT_RADIUS * 2);
                        SelectObject(hdc, old_brush);
                        DeleteObject(brush as _);
                    }
                }
            }
            unsafe { EndPaint(hwnd, &ps); }
            0
        }
        WM_MOUSEMOVE => {
            let x = (lparam & 0xFFFF) as i32;
            let y = ((lparam >> 16) & 0xFFFF) as i32;
            let mut client_rect: RECT = unsafe { zeroed() };
            unsafe { GetClientRect(hwnd, &mut client_rect); }
            let win_w = client_rect.right - client_rect.left;
            let dot_y = (PANEL_HEIGHT - DOT_RADIUS * 2) / 2;
            let mut new_hover_dot = None;
            if y < PANEL_HEIGHT {
                for (i, _) in get_dots().iter() {
                    let dot_x = win_w - DOT_MARGIN_RIGHT - (DOT_RADIUS * 2 + DOT_MARGIN) * (3 - *i as i32);
                    if x >= dot_x && x <= dot_x + DOT_RADIUS * 2 &&
                        y >= dot_y && y <= dot_y + DOT_RADIUS * 2 {
                        new_hover_dot = Some(*i);
                        break;
                    }
                }
            }
            if let Some(state) = state {
                if state.hover_dot != new_hover_dot {
                    state.hover_dot = new_hover_dot;
                    unsafe { InvalidateRect(hwnd, null_mut(), 0); }
                }
                if !state.tracking_mouse {
                    let mut tme = TRACKMOUSEEVENT {
                        cbSize: std::mem::size_of::<TRACKMOUSEEVENT>() as u32,
                        dwFlags: TME_LEAVE,
                        hwndTrack: hwnd,
                        dwHoverTime: 0,
                    };
                    unsafe { TrackMouseEvent(&mut tme); }
                    state.tracking_mouse = true;
                }
            }
            0
        }
        WM_LBUTTONUP => {
            let x = (lparam & 0xFFFF) as i32;
            let y = ((lparam >> 16) & 0xFFFF) as i32;
            let mut client_rect: RECT = unsafe { zeroed() };
            unsafe { GetClientRect(hwnd, &mut client_rect); }
            let win_w = client_rect.right - client_rect.left;
            let dot_y = (PANEL_HEIGHT - DOT_RADIUS * 2) / 2;
            if y < PANEL_HEIGHT {
                for (i, _) in get_dots().iter() {
                    let dot_x = win_w - DOT_MARGIN_RIGHT - (DOT_RADIUS * 2 + DOT_MARGIN) * (3 - *i as i32);
                    if x >= dot_x && x <= dot_x + DOT_RADIUS * 2 &&
                        y >= dot_y && y <= dot_y + DOT_RADIUS * 2 {
                        match i {
                            0 => { unsafe { PostQuitMessage(0); } },
                            1 => { unsafe { ShowWindow(hwnd, SW_MINIMIZE); } },
                            2 => {
                                let mut placement: WINDOWPLACEMENT = unsafe { zeroed() };
                                placement.length = std::mem::size_of::<WINDOWPLACEMENT>() as u32;
                                if unsafe { GetWindowPlacement(hwnd, &mut placement) } != 0 {
                                    if placement.showCmd == SW_MAXIMIZE as u32 {
                                        unsafe { ShowWindow(hwnd, SW_RESTORE); }
                                    } else {
                                        unsafe { ShowWindow(hwnd, SW_MAXIMIZE); }
                                    }
                                }
                            },
                            _ => {}
                        }
                        return 0;
                    }
                }
            }
            0
        }
        WM_MOUSELEAVE => {
            if let Some(state) = state {
                if state.hover_dot.is_some() {
                    state.hover_dot = None;
                    unsafe { InvalidateRect(hwnd, null_mut(), 0); }
                }
                state.tracking_mouse = false;
            }
            0
        }
        WM_SIZE => {
            unsafe { InvalidateRect(hwnd, null_mut(), 1); }
            0
        }
        WM_NCHITTEST => {
            let mut client_rect: RECT = unsafe { zeroed() };
            unsafe { GetClientRect(hwnd, &mut client_rect); }
            let win_w = client_rect.right - client_rect.left;
            let _win_h = client_rect.bottom - client_rect.top;
            let mut pt = POINT {
                x: (lparam & 0xFFFF) as i16 as i32,
                y: ((lparam >> 16) & 0xFFFF) as i16 as i32,
            };
            unsafe { ScreenToClient(hwnd, &mut pt); }
            let x = pt.x;
            let y = pt.y;
            let dot_y = (PANEL_HEIGHT - DOT_RADIUS * 2) / 2;
            if y < PANEL_HEIGHT {
                for (i, _) in get_dots().iter() {
                    let dot_x = win_w - DOT_MARGIN_RIGHT - (DOT_RADIUS * 2 + DOT_MARGIN) * (3 - *i as i32);
                    if x >= dot_x && x <= dot_x + DOT_RADIUS * 2 &&
                        y >= dot_y && y <= dot_y + DOT_RADIUS * 2 {
                        return HTCLIENT as isize;
                    }
                }
                return HTCAPTION as isize;
            }
            HTCLIENT as isize
        }
        WM_DESTROY => {
            if !state_ptr.is_null() {
                unsafe { let _ = Box::from_raw(state_ptr); }
                unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0); }
            }
            unsafe { PostQuitMessage(0); }
            0
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}
