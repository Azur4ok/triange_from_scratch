#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::ptr::{null, null_mut};

type c_uint = u32;
type c_int = i32;
type UINT = c_uint;
type wchar_t = u16;
type LONG_PTR = isize;
type UINT_PTR = usize;
type WCHAR = wchar_t;
type WPARAM = UINT_PTR;
type PVOID = *mut core::ffi::c_void;
type HANDLE = PVOID;
type HINSTANCE = HANDLE;
type HMODULE = HINSTANCE;
type HICON = HANDLE;
type HCURSOR = HICON;
type HMENU = HANDLE;
type HBRUSH = HANDLE;
type HWND = HANDLE;
type LPVOID = *mut core::ffi::c_void;
type LPCWSTR = *const WCHAR;
type LPARAM = LONG_PTR;
type LRESULT = LONG_PTR;
type BOOL = c_int;

type WNDPROC = Option<
    unsafe extern "system" fn(hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT,
>;

type ATOM = WORD;
type WORD = c_ushort;
type c_ushort = u16;

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

#[link(name = "User32")]
extern "system" {
    pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> ATOM;
}

#[link(name = "User32")]
extern "system" {
    pub fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;
}

type c_ulong = u32;
type DWORD = c_ulong;
// type LPMSG = *const MSG;
type LONG = c_ulong;
#[repr(C)]
pub struct POINT {
    x: LONG,
    y: LONG,
}
unsafe_impl_default_zeroed!(POINT);

#[link(name = "User32")]
extern "system" {
    pub fn TranslateMessage(lpMsg: *const MSG) -> BOOL;

    pub fn DispatchMessageW(lpMsg: *const MSG) -> LRESULT;
}

#[link(name = "User32")]
extern "system" {
    pub fn GetMessageW(
        lpMsg: *const MSG,
        hWnd: HWND,
        wMsgFilterMin: UINT,
        wMsgFilterMax: UINT,
    ) -> BOOL;
}

#[link(name = "User32")]
extern "system" {
    pub fn MessageBoxW(hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: UINT) -> c_int;
}

const MB_OKCANCEL: u32 = 1;
const IDOK: c_int = 1;

pub const WM_CLOSE: u32 = 0x0010;
pub const WM_DESTROY: u32 = 0x0002;

#[link(name = "User32")]
extern "system" {
    pub fn DestroyWindow(hWnd: HWND) -> BOOL;

    pub fn PostQuitMessage(nExitCode: c_int);
}

#[link(name = "Kernel32")]
extern "system" {
    pub fn GetLastError() -> DWORD;
}

#[link(name = "User32")]
extern "system" {
    pub fn CreateWindowExW(
        dwExStyle: DWORD,
        lpClassName: LPCWSTR,
        lpWindowName: LPCWSTR,
        dwStyle: DWORD,
        X: c_int,
        Y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: LPVOID,
    ) -> HWND;
}

#[link(name = "User32")]
extern "system" {
    pub fn DefWindowProcW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
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

impl Default for WNDCLASSW {
    #[inline]
    #[must_use]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

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

type HDC = HANDLE;
type BYTE = u8;

#[repr(C)]
pub struct RECT {
    left: LONG,
    top: LONG,
    right: LONG,
    bottom: LONG,
}

unsafe_impl_default_zeroed!(RECT);

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

#[link(name = "User32")]
extern "system" {
    pub fn BeginPaint(hWnd: HWND, lpPaint: *mut PAINTSTRUCT) -> HDC;

    pub fn FillRect(hDC: HDC, lprc: *const RECT, hbr: HBRUSH) -> c_int;

    pub fn EndPaint(hWnd: HWND, lpPaint: *const PAINTSTRUCT) -> BOOL;
}
const COLOR_WINDOW: u32 = 5;

type LPWSTR = *mut WCHAR;
type ULONG_PTR = usize;

pub const fn MAKEINTRECOURCEW(i: WORD) -> LPWSTR {
    i as ULONG_PTR as LPWSTR
}
const IDC_ARROW: LPWSTR = MAKEINTRECOURCEW(32512);

unsafe_impl_default_zeroed!(MSG);

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
// unsafe extern "system" fn dummy_window_procedure(
//     hwnd: HWND,
//     uMsg: UINT,
//     wParam: WPARAM,
//     lParam: LPARAM,
// ) -> LRESULT {
//     unimplemented!()
// }

pub fn wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

#[link(name = "Kernel32")]
extern "system" {
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
}

#[link(name = "User32")]
extern "system" {
    pub fn LoadCursorW(hInstance: HINSTANCE, lpCursorName: LPCWSTR) -> HCURSOR;
}

#[link(name = "User32")]
extern "system" {
    pub fn SetCursor(hCursor: HCURSOR) -> HCURSOR;
}

const WM_NCCREATE: u32 = 0x0081;
const WM_CREATE: u32 = 0x0001;

const WS_OVERLAPPED: u32 = 0x00000000;
const WS_CAPTION: u32 = 0x00C00000;
const WS_SYSMENU: u32 = 0x00080000;
const WS_THICKFRAME: u32 = 0x00040000;
const WS_MINIMIZEBOX: u32 = 0x00020000;
const WS_MAXIMIZEBOX: u32 = 0x00010000;
const WS_OVERLAPPEDWINDOW: u32 =
    WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;
const CW_USEDEFAULT: c_int = 0x80000000_u32 as c_int;
const SW_SHOW: c_int = 5;
const WM_PAINT: u32 = 0x000F;

#[link(name = "User32")]
extern "system" {
    pub fn SetWindowLongPtrW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR) -> LONG_PTR;
}

const GWLP_USERDATA: c_int = -21;

#[link(name = "User32")]
extern "system" {
    pub fn GetWindowLongPtrW(hWnd: HWND, nIndex: c_int) -> LONG_PTR;
}

pub unsafe extern "system" fn window_procedure(
    hWnd: HWND,
    Msg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
) -> LRESULT {
    match Msg {
        WM_NCCREATE => {
            println!("NC CREATED");
            let create_struct: *mut CREATESTRUCTW = lParam as *mut _;
            if create_struct.is_null() {
                return 0;
            }
            let boxed_i32_ptr: *mut i32 = (*create_struct).lpCreateParams.cast();
            SetWindowLongPtrW(hWnd, GWLP_USERDATA, boxed_i32_ptr as LONG_PTR);
            return 1;
        }
        WM_CREATE => println!("Create"),
        WM_CLOSE => drop(DestroyWindow(hWnd)),
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
        _ => return DefWindowProcW(hWnd, Msg, wParam, lParam),
    }
    0
}

fn main() {
    let hInstance = unsafe { GetModuleHandleW(null()) };
    let sample_window_class_wn = wide_null("Sample Window Class");
    let sample_window_name_wn = wide_null("Sample Window Name");
    let mut wc = WNDCLASSW::default();

    wc.lpfnWndProc = Some(window_procedure);
    wc.hInstance = hInstance;
    wc.lpszClassName = sample_window_class_wn.as_ptr();
    wc.hCursor = unsafe { LoadCursorW(null_mut(), IDC_ARROW) };

    let atom = unsafe { RegisterClassW(&wc) };

    if atom == 0 {
        let last_error = unsafe { GetLastError() };
        panic!(
            "Could not register the window class, error code: {}",
            last_error
        );
    }
    let lparam: *mut i32 = Box::leak(Box::new(5_32));
    let hwnd = unsafe {
        CreateWindowExW(
            0,
            sample_window_class_wn.as_ptr(),
            sample_window_name_wn.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            null_mut(),
            null_mut(),
            hInstance,
            lparam.cast(),
        )
    };

    if hwnd.is_null() {
        panic!("Failed to create a window.");
    }

    let _previously_visible = unsafe { ShowWindow(hwnd, SW_SHOW) };

    let mut msg = MSG::default();
    loop {
        let message_return = unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) };
        if message_return == 0 {
            break;
        }
        if message_return == -1 {
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
