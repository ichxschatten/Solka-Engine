#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use std::ffi::c_void;
use std::os::raw::{c_int, c_long, c_uint, c_ulong};
use std::ptr::null_mut;
use std::mem::zeroed;
use Solka_Engine_UI::{ButtonRow, Button, WizardBlock, make_wizard, make_main_buttons, layout_wizard_and_buttons, WizardLayout};

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

type WNDPROC = Option<unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT>;

#[repr(C)]
pub struct MSG {
    hwnd: HWND,
    message: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
    time: DWORD,
    pt: POINT,
}

#[repr(C)]
pub struct POINT {
    x: c_long,
    y: c_long,
}

#[repr(C)]
pub struct RECT {
    left: c_long,
    top: c_long,
    right: c_long,
    bottom: c_long,
}

#[repr(C)]
pub struct PAINTSTRUCT {
    hdc: HDC,
    fErase: BOOL,
    rcPaint: RECT,
    fRestore: BOOL,
    fIncUpdate: BOOL,
    rgbReserved: [u8; 32],
}

#[repr(C)]
pub struct TRACKMOUSEEVENT {
    cbSize: DWORD,
    dwFlags: DWORD,
    hwndTrack: HWND,
    dwHoverTime: DWORD,
}

#[repr(C)]
pub struct WINDOWPLACEMENT {
    length: UINT,
    flags: UINT,
    showCmd: UINT,
    ptMinPosition: POINT,
    ptMaxPosition: POINT,
    rcNormalPosition: RECT,
}

pub struct WindowState {
    pub hover_dot: Option<usize>,
    pub tracking_mouse: bool,
    pub greet_idx: usize,
    pub hover_button: Option<usize>,
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

fn draw_wizard_block(hdc: HDC, win_w: i32, win_h: i32, font_name: &[u16], wizard: &WizardBlock, layout: &WizardLayout) {
    let lower_block_x = layout.wizard_x;
    let lower_block_y = layout.wizard_y;
    let lower_block_w = layout.wizard_w;
    let lower_block_h = layout.wizard_h - 48;
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
    let step_text = to_wide(wizard.step);
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
    let wizard_text = to_wide(wizard.title);
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

    let instr_text = to_wide(wizard.description);
    let instr_font = unsafe { CreateFontW(13, 0, 0, 0, 600, 0, 0, 0, 0, 0, 0, 5, 0, font_name.as_ptr()) };
    let old_instr_font = unsafe { SelectObject(hdc, instr_font) };
    let mut instr_rect = RECT {
        left: lower_block_x + 16,
        top: lower_block_y + 24,
        right: lower_block_x + lower_block_w - 16,
        bottom: lower_block_y + lower_block_h + 48,
    };
    unsafe {
        SetBkMode(hdc, 1);
        SetTextColor(hdc, rgb(210, 210, 210));
        DrawTextW(hdc, instr_text.as_ptr(), (instr_text.len() as i32 - 1).max(0), &mut instr_rect, 0x0001 | 0x0010 | 0x0004 | 0x0020); // DT_CENTER | DT_WORDBREAK
        SelectObject(hdc, old_instr_font);
        DeleteObject(instr_font);
    }
    let info_text = to_wide(wizard.info);
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
    let left_margin = 32;
    let msg = to_wide("To get started, please choose one of the actions below");
    let msg_font = unsafe { CreateFontW(22, 0, 0, 0, 700, 0, 0, 0, 0, 0, 0, 5, 0, font_name.as_ptr()) };
    let old_font = unsafe { SelectObject(hdc, msg_font) };
    let mut msg_rect = RECT { left: 0, top: 0, right: 0, bottom: 0 };
    let msg_len = msg.len() as i32 - 1;
    winapi_text_extent(hdc, msg.as_ptr(), msg_len, &mut msg_rect);
    let msg_x = left_margin;
    let msg_y = line_y - 32;
    unsafe {
        SetBkMode(hdc, 1);
        SetTextColor(hdc, rgb(255, 255, 255));
        TextOutW(hdc, msg_x, msg_y, msg.as_ptr(), msg_len);
        SelectObject(hdc, old_font);
        DeleteObject(msg_font);
    }

    let button_count = 4;
    let button_w = 140;
    let button_gap = 14;
    let line_x = 32;
    let line_y = layout.buttons_y - 18; // чуть выше кнопок
    let line_right = line_x + (button_count * button_w + (button_count - 1) * button_gap);
    let line_pen = unsafe { CreatePen(0, 2, rgb(200, 200, 200)) };
    let old_pen = unsafe { SelectObject(hdc, line_pen as _) };
    unsafe {
        MoveToEx(hdc, line_x, line_y, std::ptr::null_mut());
        LineTo(hdc, line_right, line_y);
        SelectObject(hdc, old_pen);
        DeleteObject(line_pen);
    }
    for btn in &wizard.button_row.buttons {
        let btn_rect = RECT {
            left: btn.x,
            top: btn.y,
            right: btn.x + btn.w,
            bottom: btn.y + btn.h,
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
        let label_wide = to_wide(btn.label);
        let label_font = unsafe { CreateFontW(17, 0, 0, 0, 700, 0, 0, 0, 0, 0, 0, 5, 0, font_name.as_ptr()) };
        let old_label_font = unsafe { SelectObject(hdc, label_font) };
        let mut label_rect = RECT {
            left: btn.x,
            top: btn.y,
            right: btn.x + btn.w,
            bottom: btn.y + btn.h,
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

fn draw_header_panel(hdc: HDC, win_w: i32, win_h: i32, font_name: &[u16], greet_idx: usize, state: Option<&WindowState>) {
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
    if h_app_icon == null_mut() {
        eprintln!("[UI] Не удалось загрузить icon.ico");
    } else {
        unsafe {
            DrawIconEx(hdc, app_icon_x, app_icon_y, h_app_icon, app_icon_size, app_icon_size, 0, null_mut(), 0x0003);
        }
    }

    let app_name = to_wide("Solka Engine");
    let app_name_font = unsafe { CreateFontW(
        18, 0, 0, 0, 700, 0, 0, 0, 0, 0, 0, 5, 0, font_name.as_ptr()
    ) };
    let old_app_name_font = unsafe { SelectObject(hdc, app_name_font) };
    let mut app_name_size: RECT = unsafe { zeroed() };
    let _ = winapi_text_extent(hdc, app_name.as_ptr(), app_name.len() as i32 - 1, &mut app_name_size);
    let app_name_w = app_name_size.right - app_name_size.left;
    let app_name_h = app_name_size.bottom - app_name_size.top;
    let app_name_x = app_icon_x + app_icon_size + TEXT_MARGIN_LEFT;
    let app_name_y = (PANEL_HEIGHT - app_name_h) / 2;
    unsafe {
        SetBkMode(hdc, 1);
        SetTextColor(hdc, rgb(255, 255, 255));
        TextOutW(hdc, app_name_x, app_name_y, app_name.as_ptr(), app_name.len() as i32 - 1);
        SelectObject(hdc, old_app_name_font);
        DeleteObject(app_name_font);
    }

    let greet_icon_size = 64;
    let greet_bg_size = 72;
    let greet_left_margin = 32;
    let greet_icon_x = greet_left_margin + (greet_bg_size - greet_icon_size) / 2;
    let greet_icon_y = PANEL_HEIGHT + 24;
    let greet_bg_x = greet_left_margin;
    let greet_bg_y = greet_icon_y - (greet_bg_size - greet_icon_size) / 2;
    let greet_bg_rect = RECT {
        left: greet_bg_x,
        top: greet_bg_y,
        right: greet_bg_x + greet_bg_size,
        bottom: greet_bg_y + greet_bg_size,
    };
    let greet_bg_brush = unsafe { CreateSolidBrush(rgb(35, 39, 42)) };
    unsafe {
        let old_brush = SelectObject(hdc, greet_bg_brush as _);
        RoundRect(
            hdc,
            greet_bg_rect.left,
            greet_bg_rect.top,
            greet_bg_rect.right,
            greet_bg_rect.bottom,
            8,
            8,
        );
        SelectObject(hdc, old_brush);
        DeleteObject(greet_bg_brush as _);
    }
    let greet_icon_path = to_wide("icon.ico");
    let h_greet_icon = unsafe {
        LoadImageW(null_mut(), greet_icon_path.as_ptr(), 1, greet_icon_size, greet_icon_size, 0x00000010)
    } as HICON;
    if h_greet_icon != null_mut() {
        unsafe {
            DrawIconEx(hdc, greet_icon_x, greet_icon_y, h_greet_icon, greet_icon_size, greet_icon_size, 0, null_mut(), 0x0003);
        }
    }
    let greetings = Solka_Engine_UI::greetings();
    let greet = to_wide(greetings[greet_idx % greetings.len()]);
    let greet_font = unsafe { CreateFontW(22, 0, 0, 0, 600, 0, 0, 0, 0, 0, 0, 5, 0, font_name.as_ptr()) };
    let old_greet_font = unsafe { SelectObject(hdc, greet_font) };
    let mut greet_size: RECT = unsafe { zeroed() };
    let greet_len = if greet.is_empty() { 0 } else { greet.len() as i32 - 1 };
    winapi_text_extent(hdc, greet.as_ptr(), greet_len, &mut greet_size);
    let greet_text_x = greet_bg_x + greet_bg_size + 24;
    let greet_text_y = greet_icon_y + (greet_icon_size - (greet_size.bottom - greet_size.top)) / 2;
    unsafe {
        SetBkMode(hdc, 1);
        SetTextColor(hdc, rgb(255, 255, 255));
        TextOutW(hdc, greet_text_x, greet_text_y, greet.as_ptr(), greet_len);
        SelectObject(hdc, old_greet_font);
        DeleteObject(greet_font);
    }
    if let Some(state) = state {
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
    }
}

fn draw_main_buttons(hdc: HDC, font_name: &[u16], button_row: &ButtonRow) {
    for btn in &button_row.buttons {
        let btn_rect = RECT {
            left: btn.x,
            top: btn.y,
            right: btn.x + btn.w,
            bottom: btn.y + btn.h,
        };
        let btn_bg = unsafe { CreateSolidBrush(rgb(44, 47, 51)) };
        unsafe {
            FillRect(hdc, &btn_rect, btn_bg);
            DeleteObject(btn_bg as _);
        }
        let btn_border = unsafe { CreatePen(0, 2, rgb(255, 255, 255)) };
        let old_pen = unsafe { SelectObject(hdc, btn_border as _) };
        let old_brush = unsafe { SelectObject(hdc, GetStockObject(NULL_BRUSH)) };
        unsafe {
            RoundRect(
                hdc,
                btn_rect.left,
                btn_rect.top,
                btn_rect.right,
                btn_rect.bottom,
                8,
                8,
            );
            SelectObject(hdc, old_pen);
            SelectObject(hdc, old_brush);
            DeleteObject(btn_border);
        }
        let mut text_offset_x = 0;
        if btn.has_icon {
            let icon_type = btn.icon_type.unwrap_or("");
            if icon_type == "document" {
                let icon_w = 18;
                let icon_h = 22;
                let icon_x = btn.x + 10;
                let icon_y = btn.y + (btn.h - icon_h) / 2;
                let cut = 7;
                let doc_points = [
                    POINT { x: icon_x, y: icon_y },
                    POINT { x: icon_x + icon_w - cut, y: icon_y },
                    POINT { x: icon_x + icon_w, y: icon_y + cut },
                    POINT { x: icon_x + icon_w, y: icon_y + icon_h },
                    POINT { x: icon_x, y: icon_y + icon_h },
                ];
                let icon_pen = unsafe { CreatePen(0, 1, rgb(255,255,255)) };
                let old_pen = unsafe { SelectObject(hdc, icon_pen as _) };
                let old_brush = unsafe { SelectObject(hdc, GetStockObject(NULL_BRUSH)) };
                unsafe {
                    Polygon(hdc, doc_points.as_ptr(), 5);
                    MoveToEx(hdc, icon_x + icon_w - cut, icon_y, std::ptr::null_mut());
                    LineTo(hdc, icon_x + icon_w, icon_y + cut);
                    SelectObject(hdc, old_pen);
                    SelectObject(hdc, old_brush);
                    DeleteObject(icon_pen);
                }
                text_offset_x = icon_w + 16;
            } else if icon_type == "folder" {
                let icon_w = 22;
                let icon_h = 18;
                let icon_x = btn.x + 8;
                let icon_y = btn.y + (btn.h - icon_h) / 2;
                let pen_color = rgb(255,255,255);
                let pen = unsafe { CreatePen(0, 1, pen_color) };
                let old_pen = unsafe { SelectObject(hdc, pen as _) };
                let old_brush = unsafe { SelectObject(hdc, GetStockObject(NULL_BRUSH)) };
                let tab_w = 10;
                let tab_h = 4;
                let tab_x = icon_x;
                let tab_y = icon_y - tab_h;
                let tab_points = [
                    POINT { x: tab_x, y: tab_y + tab_h },
                    POINT { x: tab_x + 2, y: tab_y },
                    POINT { x: tab_x + tab_w, y: tab_y },
                    POINT { x: tab_x + tab_w + 2, y: tab_y + tab_h },
                ];
                unsafe {
                    Polyline(hdc, tab_points.as_ptr(), 4);
                }
                unsafe {
                    Rectangle(hdc, icon_x, icon_y, icon_x + icon_w, icon_y + icon_h);
                }
                unsafe {
                    SelectObject(hdc, old_pen);
                    SelectObject(hdc, old_brush);
                    DeleteObject(pen);
                }
                text_offset_x = icon_w + 16;
            } else if icon_type == "import" {
                let icon_w = 22;
                let icon_h = 18;
                let icon_x = btn.x + 8;
                let icon_y = btn.y + (btn.h - icon_h) / 2;
                let pen_color = rgb(255,255,255);
                let pen = unsafe { CreatePen(0, 1, pen_color) };
                let old_pen = unsafe { SelectObject(hdc, pen as _) };
                let old_brush = unsafe { SelectObject(hdc, GetStockObject(NULL_BRUSH)) };
                unsafe {
                    Rectangle(hdc, icon_x, icon_y, icon_x + icon_w, icon_y + icon_h);
                    SelectObject(hdc, old_pen);
                    SelectObject(hdc, old_brush);
                    DeleteObject(pen);
                }
                let arrow_color = rgb(255,255,255);
                let arrow_pen = unsafe { CreatePen(0, 1, arrow_color) };
                let old_pen = unsafe { SelectObject(hdc, arrow_pen as _) };
                let arrow_x = icon_x + icon_w / 2;
                let arrow_top = icon_y - 3;
                let arrow_bot = icon_y + icon_h / 2 + 2;
                unsafe {
                    MoveToEx(hdc, arrow_x, arrow_top, std::ptr::null_mut());
                    LineTo(hdc, arrow_x, arrow_bot);
                    MoveToEx(hdc, arrow_x, arrow_bot, std::ptr::null_mut());
                    LineTo(hdc, arrow_x - 4, arrow_bot - 4);
                    MoveToEx(hdc, arrow_x, arrow_bot, std::ptr::null_mut());
                    LineTo(hdc, arrow_x + 4, arrow_bot - 4);
                    SelectObject(hdc, old_pen);
                    DeleteObject(arrow_pen);
                }
                text_offset_x = icon_w + 16;
            } else if icon_type == "guide" {
                let icon_w = 18;
                let icon_h = 22;
                let icon_x = btn.x + 10;
                let icon_y = btn.y + (btn.h - icon_h) / 2;
                let pen_color = rgb(255,255,255);
                let pen = unsafe { CreatePen(0, 1, pen_color) };
                let old_pen = unsafe { SelectObject(hdc, pen as _) };
                let old_brush = unsafe { SelectObject(hdc, GetStockObject(NULL_BRUSH)) };
                let cut = 7;
                let doc_points = [
                    POINT { x: icon_x, y: icon_y },
                    POINT { x: icon_x + icon_w - cut, y: icon_y },
                    POINT { x: icon_x + icon_w, y: icon_y + cut },
                    POINT { x: icon_x + icon_w, y: icon_y + icon_h },
                    POINT { x: icon_x, y: icon_y + icon_h },
                ];
                unsafe {
                    Polygon(hdc, doc_points.as_ptr(), 5);
                    MoveToEx(hdc, icon_x + icon_w - cut, icon_y, std::ptr::null_mut());
                    LineTo(hdc, icon_x + icon_w, icon_y + cut);
                }
                let cx = icon_x + icon_w / 2 + 1;
                let cy = icon_y + icon_h / 2 + 1;
                let r = 4;
                unsafe {
                    Ellipse(hdc, cx - r, cy - r, cx + r, cy + r);
                    MoveToEx(hdc, cx + r - 1, cy + r - 1, std::ptr::null_mut());
                    LineTo(hdc, cx + r + 4, cy + r + 4);
                    SelectObject(hdc, old_pen);
                    SelectObject(hdc, old_brush);
                    DeleteObject(pen);
                }
                text_offset_x = icon_w + 16;
            }
        }
        let label_wide = to_wide(btn.label);
        let label_font = unsafe { CreateFontW(17, 0, 0, 0, 700, 0, 0, 0, 0, 0, 0, 5, 0, font_name.as_ptr()) };
        let old_label_font = unsafe { SelectObject(hdc, label_font) };
        let mut label_rect = RECT {
            left: btn.x + 8 + text_offset_x,
            top: btn.y,
            right: btn.x + btn.w,
            bottom: btn.y + btn.h,
        };

        let mut text_size: RECT = unsafe { zeroed() };
        winapi_text_extent(hdc, label_wide.as_ptr(), (label_wide.len() as i32 - 1).max(0), &mut text_size);
        let text_h = text_size.bottom - text_size.top;
        let text_y = btn.y + (btn.h - text_h) / 2;
        label_rect.top = text_y;
        label_rect.bottom = text_y + text_h;
        unsafe {
            SetBkMode(hdc, 1);
            SetTextColor(hdc, rgb(255, 255, 255));
            DrawTextW(hdc, label_wide.as_ptr(), (label_wide.len() as i32 - 1).max(0), &mut label_rect, 0x0000);
            SelectObject(hdc, old_label_font);
            DeleteObject(label_font);
        }
    }
}

fn draw_wizard_message_and_line(hdc: HDC, win_w: i32, win_h: i32, font_name: &[u16], layout: &WizardLayout) {
    let left_margin = 32;
    let msg_y = layout.msg_y;
    let msg = to_wide("To get started, please choose one of the actions below");
    let msg_font = unsafe { CreateFontW(18, 0, 0, 0, 600, 0, 0, 0, 0, 0, 0, 5, 0, font_name.as_ptr()) };
    let old_font = unsafe { SelectObject(hdc, msg_font) };
    let mut msg_rect = RECT { left: 0, top: 0, right: 0, bottom: 0 };
    let msg_len = msg.len() as i32 - 1;
    winapi_text_extent(hdc, msg.as_ptr(), msg_len, &mut msg_rect);
    let text_height = msg_rect.bottom - msg_rect.top;
    unsafe {
        SetBkMode(hdc, 1);
        SetTextColor(hdc, rgb(200, 200, 200));
        TextOutW(hdc, left_margin, msg_y, msg.as_ptr(), msg_len);
        SelectObject(hdc, old_font);
        DeleteObject(msg_font);
    }
    let line_y = msg_y + text_height + 6;
    let line_pen = unsafe { CreatePen(0, 2, rgb(200, 200, 200)) };
    let old_pen = unsafe { SelectObject(hdc, line_pen as _) };
    let line_right = win_w - left_margin;
    unsafe {
        MoveToEx(hdc, left_margin, line_y, std::ptr::null_mut());
        LineTo(hdc, line_right, line_y);
        SelectObject(hdc, old_pen);
        DeleteObject(line_pen);
    }
    let main_buttons = make_main_buttons(win_w, win_h);
    draw_main_buttons(hdc, &font_name, &main_buttons);
}

fn draw_left_line(hdc: HDC, layout: &WizardLayout) {
    let line_pen = unsafe { CreatePen(0, 2, rgb(200, 200, 200)) };
    let old_pen = unsafe { SelectObject(hdc, line_pen as _) };
    unsafe {
        MoveToEx(hdc, layout.line_x, layout.line_y, std::ptr::null_mut());
        LineTo(hdc, layout.line_right, layout.line_y);
        SelectObject(hdc, old_pen);
        DeleteObject(line_pen);
    }
}

pub fn run_window() {
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
            hover_button: None,
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
    fn Polyline(hdc: HDC, lpPoints: *const POINT, cCount: c_int) -> BOOL;
    fn Arc(hdc: HDC, left: c_int, top: c_int, right: c_int, bottom: c_int, startX: c_int, startY: c_int, endX: c_int, endY: c_int) -> BOOL;
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
            let greet_idx = if let Some(ref state) = state { state.greet_idx } else { 0 };
            let font_name = to_wide("Oregano");
            draw_header_panel(hdc, win_w, win_h, &font_name, greet_idx, state.as_deref());
            let layout = layout_wizard_and_buttons(win_w, win_h);
            let wizard = make_wizard(win_w, win_h, greet_idx);
            draw_wizard_block(hdc, win_w, win_h, &font_name, &wizard, &layout);
            let main_buttons = make_main_buttons(win_w, win_h);
            draw_main_buttons(hdc, &font_name, &main_buttons);
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
    res
}
