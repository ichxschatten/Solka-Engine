#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use std::ffi::c_void;
use std::os::raw::{c_int, c_long, c_uint, c_ulong};
use std::ptr::null_mut;
use std::mem::zeroed;

const PANEL_HEIGHT: i32 = 48;
const PANEL_PADDING_TOP: i32 = 0;
const DOT_RADIUS: i32 = 8;
const DOT_MARGIN: i32 = 8;
const DOT_MARGIN_RIGHT: i32 = 0;
const ICON_SIZE: i32 = 32;
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
    greet_idx: usize,
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
            WS_POPUP | WS_VISIBLE,
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
        use std::time::{SystemTime, UNIX_EPOCH};
        let greetings_len = 13;
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let greet_idx = (now % greetings_len as u64) as usize;
        let state = Box::new(WindowState {
            hover_dot: None,
            tracking_mouse: false,
            greet_idx,
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
    fn FrameRect(hdc: HDC, lprc: *const RECT, hbr: HBRUSH) -> BOOL;
    fn Rectangle(hdc: HDC, left: c_int, top: c_int, right: c_int, bottom: c_int) -> BOOL;
    fn DrawTextW(hdc: HDC, lpString: LPCWSTR, c: i32, lpRect: *mut RECT, uFormat: UINT) -> i32;
}

#[link(name = "gdi32")]
unsafe extern "system" {
    fn Polygon(hdc: HDC, lpPoints: *const POINT, nCount: c_int) -> BOOL;
    fn CreatePen(fnPenStyle: c_int, nWidth: c_int, crColor: u32) -> HGDIOBJ;
    fn MoveToEx(hdc: HDC, x: c_int, y: c_int, lpPoint: *mut POINT) -> BOOL;
    fn LineTo(hdc: HDC, x: c_int, y: c_int) -> BOOL;
    fn RoundRect(
        hdc: HDC,
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
        ellipse_width: c_int,
        ellipse_height: c_int,
    ) -> BOOL;
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
            let win_h = client_rect.bottom - client_rect.top;
            let bg_rect = RECT { left: 0, top: 0, right: win_w, bottom: win_h };
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
            let app_icon_size = 32;
            let app_icon_margin = 8;
            let app_icon_y = (PANEL_HEIGHT - app_icon_size) / 2;
            let app_icon_x = app_icon_margin;
            let icon_path = to_wide("icon.ico");
            let h_app_icon = unsafe {
                LoadImageW(null_mut(), icon_path.as_ptr(), 1, app_icon_size, app_icon_size, 0x00000010)
            } as HICON;
            if h_app_icon != null_mut() {
                unsafe {
                    DrawIconEx(hdc, app_icon_x, app_icon_y, h_app_icon, app_icon_size, app_icon_size, 0, null_mut(), 0x0003);
                }
            }
            let app_name = to_wide("Solka Engine");
            let font_name = to_wide("Oregano");
            let app_name_font = unsafe { CreateFontW(
                18, 0, 0, 0, 700, 0, 0, 0, 0, 0, 0, 5, 0, to_wide("Oregano").as_ptr()
            ) };
            let old_app_name_font = unsafe { SelectObject(hdc, app_name_font) };
            let mut app_name_size: RECT = unsafe { zeroed() };
            let _ = winapi_text_extent(hdc, app_name.as_ptr(), app_name.len() as i32 - 1, &mut app_name_size);
            let app_name_x = app_icon_x + app_icon_size + 8;
            let app_name_y = (PANEL_HEIGHT - (app_name_size.bottom - app_name_size.top)) / 2;
            unsafe {
                SetBkMode(hdc, 1);
                SetTextColor(hdc, rgb(255, 255, 255));
                TextOutW(hdc, app_name_x, app_name_y, app_name.as_ptr(), app_name.len() as i32 - 1);
                SelectObject(hdc, old_app_name_font);
                DeleteObject(app_name_font);
            }
            if let Some(state) = state {
                let font_name = to_wide("Oregano");
                let hover_dot = state.hover_dot;
                let dot_y = (PANEL_HEIGHT - DOT_RADIUS * 2) / 2;
                for (i, &(_, color)) in get_dots().iter().enumerate() {
                    let dot_x = win_w - DOT_MARGIN_RIGHT - (DOT_RADIUS * 2 + DOT_MARGIN) * (3 - i as i32);
                    let mut draw_color = color;
                    if Some(i) == hover_dot {
                        let r = ((color & 0xFF) + 60).min(255) as u8;
                        let g = (((color >> 8) & 0xFF) + 60).min(255) as u8;
                        let b = (((color >> 16) & 0xFF) + 60).min(255) as u8;
                        draw_color = rgb(r, g, b);
                    }
                    let dot_brush = unsafe { CreateSolidBrush(draw_color) };
                    unsafe {
                        let old_brush = SelectObject(hdc, dot_brush as _);
                        Ellipse(
                            hdc,
                            dot_x,
                            dot_y,
                            dot_x + DOT_RADIUS * 2,
                            dot_y + DOT_RADIUS * 2,
                        );
                        SelectObject(hdc, old_brush);
                        DeleteObject(dot_brush as _);
                        if Some(i) == hover_dot {
                            let pen = CreatePen(0, 3, rgb(255,255,255));
                            let old_pen = SelectObject(hdc, pen as _);
                            Ellipse(
                                hdc,
                                dot_x,
                                dot_y,
                                dot_x + DOT_RADIUS * 2,
                                dot_y + DOT_RADIUS * 2,
                            );
                            SelectObject(hdc, old_pen);
                            DeleteObject(pen as _);
                        }
                    }
                }
                let rect_margin = (win_w as f32 * 0.03) as i32;
                let rect_w = win_w - rect_margin * 2;
                let rect_h = 80;
                let rect_x = rect_margin;
                let rect_y = PANEL_HEIGHT + 8;
                let rect_brush = unsafe { CreateSolidBrush(rgb(38, 40, 45)) };
                let rect = RECT {
                    left: rect_x,
                    top: rect_y,
                    right: rect_x + rect_w,
                    bottom: rect_y + rect_h,
                };
                unsafe {
                    FillRect(hdc, &rect, rect_brush);
                    DeleteObject(rect_brush as _);
                }
                let icon_size = (win_h as f32 * 0.10).clamp(40.0, 96.0) as i32;
                let icon_margin_left = (win_w as f32 * 0.01).max(8.0) as i32;
                let text_margin_left = (win_w as f32 * 0.02).max(12.0) as i32;
                let greet_rect_h = icon_size + 12;
                let greet_rect_x = 0;
                let greet_rect_y = PANEL_HEIGHT;
                let greet_rect_w = win_w;
                let greet_rect = RECT {
                    left: greet_rect_x,
                    top: greet_rect_y,
                    right: greet_rect_x + greet_rect_w,
                    bottom: greet_rect_y + greet_rect_h,
                };
                let greet_brush = unsafe { CreateSolidBrush(rgb(38, 40, 45)) };
                unsafe {
                    FillRect(hdc, &greet_rect, greet_brush);
                    DeleteObject(greet_brush as _);
                }
                let icon_x = greet_rect_x + icon_margin_left;
                let icon_y = greet_rect_y + (greet_rect_h - icon_size) / 2;
                let icon_path = to_wide("icon.ico");
                let h_icon = unsafe {
                    LoadImageW(std::ptr::null_mut(), icon_path.as_ptr(), 1, icon_size, icon_size, 0x00000010)
                } as HICON;
                if h_icon != std::ptr::null_mut() {
                    unsafe {
                        DrawIconEx(hdc, icon_x, icon_y, h_icon, icon_size, icon_size, 0, std::ptr::null_mut(), 0x0003);
                    }
                }
                let greetings = [
                    "Welcome to Solka Engine — your path to new heights!",
                    "Solka Engine inspires great projects and new ideas!",
                    "May Solka Engine bring you luck and success in every code!",
                    "Solka Engine — your cosmic companion in the world of creativity!",
                    "With Solka Engine, your dreams of cool apps become reality!",
                    "Solka Engine: where the best projects and boldest ideas are born!",
                    "Inspiration is near — Solka Engine always supports your flight!",
                    "Solka Engine — your portal to the world of innovation and creativity!",
                    "With Solka Engine, you are always one step ahead!",
                    "Solka Engine: your reliable friend in the world of code and ideas!",
                    "Let Solka Engine open new horizons for you!",
                    "Solka Engine — your source of inspiration and strength!",
                    "With Solka Engine, the impossible becomes possible!",
                ];
                let greet_idx = state.greet_idx % greetings.len();
                let greet = to_wide(greetings[greet_idx]);
                let font_name = to_wide("Oregano");
                let greet_font = unsafe { CreateFontW(22, 0, 0, 0, 800, 0, 0, 0, 0, 0, 0, 5, 0, font_name.as_ptr()) };
                let old_font = unsafe { SelectObject(hdc, greet_font) };
                let mut text_size: RECT = zeroed();
                let greet_len = if greet.is_empty() { 0 } else { greet.len() as i32 - 1 };
                winapi_text_extent(hdc, greet.as_ptr(), greet_len, &mut text_size);
                let text_h = text_size.bottom - text_size.top;
                let text_x = icon_x + icon_size + text_margin_left;
                let text_y = greet_rect_y + (greet_rect_h - text_h) / 2;
                unsafe {
                    SetBkMode(hdc, 1);
                    SetTextColor(hdc, rgb(255, 255, 255));
                    TextOutW(hdc, text_x, text_y, greet.as_ptr(), greet_len);
                    SelectObject(hdc, old_font);
                    DeleteObject(greet_font);
                }
                let lower_block_w = 600;
                let lower_block_h = 160;
                let lower_block_x = (win_w - lower_block_w) / 2;
                let lower_block_y = greet_rect_y + greet_rect_h + 12;
                let glow_offset = 4;
                let glow_rect = RECT {
                    left: lower_block_x - glow_offset,
                    top: lower_block_y - glow_offset,
                    right: lower_block_x + lower_block_w + glow_offset,
                    bottom: lower_block_y + lower_block_h + glow_offset,
                };
                let glow_brush = unsafe { CreateSolidBrush(rgb(255, 255, 255)) };
                unsafe {
                    FrameRect(hdc, &glow_rect, glow_brush);
                    DeleteObject(glow_brush as _);
                }
                let border_rect = RECT {
                    left: lower_block_x,
                    top: lower_block_y,
                    right: lower_block_x + lower_block_w,
                    bottom: lower_block_y + lower_block_h,
                };
                let white_pen = unsafe { CreatePen(0, 2, rgb(255, 255, 255)) };
                let old_pen = unsafe { SelectObject(hdc, white_pen as _) };
                let old_brush = unsafe { SelectObject(hdc, GetStockObject(NULL_BRUSH)) };
                unsafe {
                    Rectangle(
                        hdc,
                        border_rect.left,
                        border_rect.top,
                        border_rect.right,
                        border_rect.bottom,
                    );
                    SelectObject(hdc, old_pen);
                    SelectObject(hdc, old_brush);
                    DeleteObject(white_pen);
                }
                let step_text = to_wide("Step 1 of 1");
                let step_font = unsafe { CreateFontW(17, 0, 0, 0, 600, 0, 0, 0, 0, 0, 0, 5, 0, font_name.as_ptr()) };
                let old_step_font = unsafe { SelectObject(hdc, step_font) };
                let mut step_rect = RECT {
                    left: lower_block_x,
                    top: lower_block_y + 12,
                    right: lower_block_x + lower_block_w,
                    bottom: lower_block_y + 40,
                };
                unsafe {
                    SetBkMode(hdc, 1);
                    SetTextColor(hdc, rgb(180, 180, 180));
                    DrawTextW(hdc, step_text.as_ptr(), (step_text.len() as i32 - 1).max(0), &mut step_rect, 0x0001 | 0x0004);
                    SelectObject(hdc, old_step_font);
                    DeleteObject(step_font);
                }
                let wizard_text = to_wide("Setup Wizard");
                let wizard_font = unsafe { CreateFontW(28, 0, 0, 0, 900, 0, 0, 0, 0, 0, 0, 5, 0, font_name.as_ptr()) };
                let old_wizard_font = unsafe { SelectObject(hdc, wizard_font) };
                let mut wizard_rect = RECT {
                    left: lower_block_x,
                    top: lower_block_y + 40,
                    right: lower_block_x + lower_block_w,
                    bottom: lower_block_y + 90,
                };
                let mut shadow_rect = RECT {
                    left: wizard_rect.left + 2,
                    top: wizard_rect.top + 2,
                    right: wizard_rect.right + 2,
                    bottom: wizard_rect.bottom + 2,
                };
                unsafe {
                    SetBkMode(hdc, 1);
                    SetTextColor(hdc, rgb(60, 60, 60));
                    DrawTextW(hdc, wizard_text.as_ptr(), (wizard_text.len() as i32 - 1).max(0), &mut shadow_rect, 0x0001 | 0x0004);
                    SetBkMode(hdc, 1);
                    SetTextColor(hdc, rgb(255, 255, 255));
                    DrawTextW(hdc, wizard_text.as_ptr(), (wizard_text.len() as i32 - 1).max(0), &mut wizard_rect, 0x0001 | 0x0004);
                    SelectObject(hdc, old_wizard_font);
                    DeleteObject(wizard_font);
                }
                let instr_text = "To create your first project, click the Create Project button below. Follow the steps in the wizard to complete setup.";
                let instr_wide = to_wide(instr_text);
                let instr_font = unsafe { CreateFontW(16, 0, 0, 0, 600, 0, 0, 0, 0, 0, 0, 5, 0, font_name.as_ptr()) };
                let old_instr_font = unsafe { SelectObject(hdc, instr_font) };
                let mut instr_rect = RECT {
                    left: lower_block_x + 16,
                    top: lower_block_y + 95,
                    right: lower_block_x + lower_block_w - 16,
                    bottom: lower_block_y + lower_block_h - 8,
                };
                unsafe {
                    SetBkMode(hdc, 1);
                    SetTextColor(hdc, rgb(210, 210, 210));
                    DrawTextW(hdc, instr_wide.as_ptr(), (instr_wide.len() as i32 - 1).max(0), &mut instr_rect, 0x0001 | 0x0010 | 0x0004);
                    SelectObject(hdc, old_instr_font);
                    DeleteObject(instr_font);
                }
                let info_text = to_wide("To get started, please choose one of the actions below.");
                let info_font = unsafe { CreateFontW(18, 0, 0, 0, 600, 0, 0, 0, 0, 0, 0, 5, 0, font_name.as_ptr()) };
                let old_info_font = unsafe { SelectObject(hdc, info_font) };
                let mut info_rect = RECT {
                    left: lower_block_x + 8,
                    top: lower_block_y + lower_block_h + 26,
                    right: lower_block_x + lower_block_w,
                    bottom: lower_block_y + lower_block_h + 56,
                };
                unsafe {
                    SetBkMode(hdc, 1);
                    SetTextColor(hdc, rgb(200, 200, 200));
                    DrawTextW(hdc, info_text.as_ptr(), (info_text.len() as i32 - 1).max(0), &mut info_rect, 0x0000);
                    SelectObject(hdc, old_info_font);
                    DeleteObject(info_font);
                }
                let line_y = lower_block_y + lower_block_h + 64;
                let line_pen = unsafe { CreatePen(0, 2, rgb(200, 200, 200)) };
                let old_pen = unsafe { SelectObject(hdc, line_pen as _) };
                unsafe {
                    MoveToEx(hdc, lower_block_x, line_y, std::ptr::null_mut());
                    LineTo(hdc, lower_block_x + lower_block_w, line_y);
                    SelectObject(hdc, old_pen);
                    DeleteObject(line_pen);
                }
                let button_labels = [
                    "Create Project",
                    "Open Project",
                    "Import Project",
                    "Guide",
                ];
                let button_count = button_labels.len();
                let button_row_w = lower_block_w;
                let button_w = 130;
                let button_h = 40;
                let button_gap = 20;
                let total_buttons_w = button_count as i32 * button_w + (button_count as i32 - 1) * button_gap;
                let buttons_x = lower_block_x + (button_row_w - total_buttons_w) / 2;
                let buttons_y = line_y + 24;
                for (i, label) in button_labels.iter().enumerate() {
                    let btn_left = buttons_x + i as i32 * (button_w + button_gap);
                    let btn_rect = RECT {
                        left: btn_left,
                        top: buttons_y,
                        right: btn_left + button_w,
                        bottom: buttons_y + button_h,
                    };
                    let btn_bg = unsafe { CreateSolidBrush(rgb(44, 47, 51)) };
                    unsafe {
                        FillRect(hdc, &btn_rect, btn_bg);
                        DeleteObject(btn_bg as _);
                    }
                    let btn_border = unsafe { CreatePen(0, 2, rgb(255, 255, 255)) };
                    let old_pen = unsafe { SelectObject(hdc, btn_border as _) };
                    unsafe {
                        MoveToEx(hdc, btn_rect.left, btn_rect.top, std::ptr::null_mut());
                        LineTo(hdc, btn_rect.right, btn_rect.top);
                        LineTo(hdc, btn_rect.right, btn_rect.bottom);
                        LineTo(hdc, btn_rect.left, btn_rect.bottom);
                        LineTo(hdc, btn_rect.left, btn_rect.top);
                        SelectObject(hdc, old_pen);
                        DeleteObject(btn_border);
                    }
                    let label_wide = to_wide(label);
                    let label_font = unsafe { CreateFontW(17, 0, 0, 0, 700, 0, 0, 0, 0, 0, 0, 5, 0, font_name.as_ptr()) };
                    let old_label_font = unsafe { SelectObject(hdc, label_font) };
                    let mut label_rect = RECT {
                        left: btn_left,
                        top: buttons_y,
                        right: btn_left + button_w,
                        bottom: buttons_y + button_h,
                    };
                    unsafe {
                        SetBkMode(hdc, 1);
                        SetTextColor(hdc, rgb(255, 255, 255));
                        DrawTextW(hdc, label_wide.as_ptr(), (label_wide.len() as i32 - 1).max(0), &mut label_rect, 0x0001 | 0x0004);
                        SelectObject(hdc, old_label_font);
                        DeleteObject(label_font);
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
            let win_h = client_rect.bottom - client_rect.top;
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
                let prev_hover = state.hover_dot;
                if prev_hover != new_hover_dot {
                    state.hover_dot = new_hover_dot;
                    let btns_left = win_w - (DOT_RADIUS * 2 + DOT_MARGIN) * 3 - 8;
                    let btns_top = 0;
                    let btns_right = win_w;
                    let btns_bottom = PANEL_HEIGHT;
                    let btns_rect = RECT {
                        left: btns_left,
                        top: btns_top,
                        right: btns_right,
                        bottom: btns_bottom,
                    };
                    unsafe { InvalidateRect(hwnd, &btns_rect, 0); }
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
            let win_h = client_rect.bottom - client_rect.top;
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
                    let mut client_rect: RECT = zeroed();
                    GetClientRect(hwnd, &mut client_rect);
                    let win_w = client_rect.right - client_rect.left;
                    let win_h = client_rect.bottom - client_rect.top;
                    let btns_left = win_w - (DOT_RADIUS * 2 + DOT_MARGIN) * 3 - 8;
                    let btns_top = 0;
                    let btns_right = win_w;
                    let btns_bottom = PANEL_HEIGHT;
                    let btns_rect = RECT {
                        left: btns_left,
                        top: btns_top,
                        right: btns_right,
                        bottom: btns_bottom,
                    };
                    InvalidateRect(hwnd, &btns_rect, 0);
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
            let win_h = client_rect.bottom - client_rect.top;
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

fn winapi_text_extent(hdc: HDC, text: LPCWSTR, len: i32, rect: &mut RECT) -> i32 {
    use std::ptr::null_mut;
    #[repr(C)]
    struct SIZE { cx: i32, cy: i32 }
    unsafe extern "system" {
        fn GetTextExtentPoint32W(hdc: HDC, lpString: LPCWSTR, c: i32, lpSize: *mut SIZE) -> i32;
    }
    let mut size = SIZE { cx: 0, cy: 0 };
    let res = unsafe { GetTextExtentPoint32W(hdc, text, len, &mut size) };
    rect.left = 0;
    rect.top = 0;
    rect.right = size.cx;
    rect.bottom = size.cy;
    res}
