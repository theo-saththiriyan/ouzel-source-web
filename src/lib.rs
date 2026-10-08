#![no_std]

use core::ptr::addr_of_mut;
#[link(wasm_import_module = "0")]
extern "C" {
    #[link_name = "66"]
    fn reg(p: *const u8, l: u32);
    #[link_name = "186"]
    fn get(dst: u32, obj: u32, key: u32);
    #[link_name = "111"]
    fn getn(obj: u32, key: u32) -> f64;
    #[link_name = "25"]
    fn setn(obj: u32, key: u32, v: f64);
    #[link_name = "9"]
    fn sets(obj: u32, key: u32, s: u32);
    #[link_name = "50"]
    fn seth(obj: u32, key: u32, h: u32);
    #[link_name = "35"]
    fn call(dst: u32, obj: u32, key: u32, n: u32, s: u32, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64);
    #[link_name = "31"]
    fn hcall(dst: u32, obj: u32, key: u32, a: u32, b: u32) -> u32;
    #[link_name = "47"]
    fn make(dst: u32, obj: u32, a: u32);
    #[link_name = "123"]
    fn lit(dst: u32, s: u32);
    #[link_name = "68"]
    fn func(dst: u32, export: u32, slot: u32);
    #[link_name = "44"]
    fn num(h: u32) -> f64;
    #[link_name = "143"]
    fn truthy(h: u32) -> u32;
    #[link_name = "70"]
    fn setf(obj: u32, key: u32, pre: u32, v: f64, suf: u32);
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

macro_rules! strings {
    ($($id:ident = $s:expr),* $(,)?) => {
        #[derive(Clone, Copy)]
        #[repr(u32)]
        #[allow(dead_code)]
        enum Id { $($id),* }
        const STRS: &[&[u8]] = &[$($s.as_bytes()),*];
    };
}
strings! {
    Document = "document",
    Body = "body",
    Create = "createElement",
    Canvas = "canvas",
    Append = "appendChild",
    GetCtx = "getContext",
    Ctx2d = "2d",
    Alpha = "alpha",
    Style = "style",
    CssText = "cssText",
    Css = "position:fixed;inset:0;width:100%;height:100%;user-select:none",
    Width = "width",
    Height = "height",
    RoCtor = "ResizeObserver",
    Observe = "observe",
    Box = "box",
    BoxVal = "device-pixel-content-box",
    ObjCtor = "Object",
    DpSize = "devicePixelContentBoxSize",
    Zero = "0",
    Inline = "inlineSize",
    Block = "blockSize",
    Dpr = "devicePixelRatio",
    InnerW = "innerWidth",
    InnerH = "innerHeight",
    Raf = "requestAnimationFrame",
    Perf = "performance",
    Now = "now",
    Menu = "oncontextmenu",
    Prevent = "preventDefault",
    HeadEl = "head",
    Link = "link",
    Rel = "rel",
    Icon = "icon",
    Href = "href",
    ToUrl = "toDataURL",
    Baseline = "textBaseline",
    Middle = "middle",
    Align = "textAlign",
    Center = "center",
    FillStyle = "fillStyle",
    GAlpha = "globalAlpha",
    Font = "font",
    Semi = "600 ",
    Plain = "",
    FontTail = "px system-ui,-apple-system,'Segoe UI',Roboto,'Helvetica Neue',Arial,sans-serif",
    FillRect = "fillRect",
    FillText = "fillText",
    BeginPath = "beginPath",
    MoveTo = "moveTo",
    LineTo = "lineTo",
    Bezier = "bezierCurveTo",
    FillPath = "fill",
    Arc = "arc",
    Bg = "#0b0d12",
    Blue = "#3d86ff",
    White = "#eef1f7",
    Grey = "#8b93a5",
    Headline = "Ouzel Web is coming soon",
    Sub = "We're almost ready. Check back soon.",
}
const PATH: &[u8] = &[
    4, 1, 6,
    0, 97, 40,
    1, 86, 35,
    2, 85, 22, 73, 15, 62, 20,
    2, 50, 25, 44, 34, 38, 42,
    2, 30, 52, 18, 58, 3, 62,
    1, 5, 70,
    1, 28, 73,
    2, 32, 90, 60, 96, 74, 78,
    2, 82, 66, 78, 54, 81, 46,
    2, 83, 45, 85, 44, 86, 44,
    1, 97, 40,
    3,
    4, 0,
    5, 75, 33, 34,
];

const K: usize = STRS.len();
const PATH_END: usize = PATH.len();

const fn offsets() -> [usize; K + 1] {
    let mut o = [0usize; K + 1];
    let mut i = 0;
    while i < K {
        o[i + 1] = o[i] + STRS[i].len();
        i += 1;
    }
    o
}

const OFFS: [usize; K + 1] = offsets();
const N: usize = PATH_END + OFFS[K];
const fn lens() -> [u32; K] {
    let mut o = [0u32; K];
    let mut i = 0;
    while i < K {
        o[i] = STRS[i].len() as u32;
        i += 1;
    }
    o
}
const LENS: [u32; K] = lens();
const fn pack() -> [u8; N] {
    let mut o = [0u8; N];
    let mut i = 0;
    while i < PATH_END {
        o[i] = PATH[i];
        i += 1;
    }
    let mut k = 0;
    while k < K {
        let mut j = 0;
        while j < STRS[k].len() {
            o[PATH_END + OFFS[k] + j] = STRS[k][j];
            j += 1;
        }
        k += 1;
    }
    let mut s: u32 = 0x1d87_2b41;
    i = 0;
    while i < N {
        s = s.wrapping_mul(1664525).wrapping_add(1013904223);
        o[i] ^= (s >> 24) as u8;
        i += 1;
    }
    o
}

static SEALED: [u8; N] = pack();

struct World {
    ready: bool,
    pending: bool,
    t0: f64,
    w: f64,
    h: f64,
    d: f64,
    buf: [u8; N],
}

static mut W: World = World {
    ready: false,
    pending: false,
    t0: -1.0,
    w: 0.0,
    h: 0.0,
    d: 1.0,
    buf: [0; N],
};

fn world() -> &'static mut World {
    let s = unsafe { &mut *addr_of_mut!(W) };
    if !s.ready {
        let mut r: u32 = 0x1d87_2b41;
        for (i, b) in SEALED.iter().enumerate() {
            r = r.wrapping_mul(1664525).wrapping_add(1013904223);
            s.buf[i] = b ^ (r >> 24) as u8;
        }
        s.ready = true;
    }
    s
}
const WIN: u32 = 0;
const DOC: u32 = 1;
const BODY: u32 = 2;
const CANVAS: u32 = 3;
const CTX: u32 = 4;
const STY: u32 = 5;
const OBJC: u32 = 6;
const OPTS: u32 = 7;
const T1: u32 = 8;
const ICN: u32 = 9;
const ICTX: u32 = 10;
const HEAD: u32 = 11;
const LNK: u32 = 12;
const URL: u32 = 13;
const CB_MENU: u32 = 14;
const EV: u32 = 15;
const CB_RAF: u32 = 16;
const TS: u32 = 17;
const CB_RO: u32 = 18;
const ENTS: u32 = 19;
const ROC: u32 = 20;
const RO: u32 = 21;
const OBJ2: u32 = 22;
const PERF: u32 = 23;
const ENT: u32 = 24;
const ARR: u32 = 25;
const SZ: u32 = 26;
const NOW: u32 = 27;
const FADE: f64 = 0.7;
const MARK: f64 = 0.7;
const HEAD_SIZE: f64 = 34.0;
const SUB_SIZE: f64 = 16.0;
const BIRD_DY: f64 = -70.0;
const SUB_DY: f64 = 38.0;
const BLOCK_DY: f64 = 26.0;

fn ease(x: f64) -> f64 {
    let x = x.clamp(0.0, 1.0);
    x * x * (3.0 - 2.0 * x)
}

unsafe fn op(o: u32, k: Id, n: u32, a: [f64; 6]) {
    call(0, o, k as u32, n, 0, a[0], a[1], a[2], a[3], a[4], a[5]);
}

unsafe fn draw_text(s: Id, x: f64, y: f64, size: f64, semi: bool) {
    let pre = if semi { Id::Semi } else { Id::Plain };
    setf(CTX, Id::Font as u32, pre as u32, size, Id::FontTail as u32);
    call(0, CTX, Id::FillText as u32, 2, s as u32 + 1, x, y, 0.0, 0.0, 0.0, 0.0);
}

impl World {
    unsafe fn bird(&self, ctx: u32, x: f64, y: f64, k: f64) {
        let b = &self.buf;
        let p = |i: usize| b[i] as f64 * k;
        let mut i = 0;
        while i < PATH_END {
            match b[i] {
                0 => {
                    op(ctx, Id::MoveTo, 2, [x + p(i + 1), y + p(i + 2), 0.0, 0.0, 0.0, 0.0]);
                    i += 3;
                }
                1 => {
                    op(ctx, Id::LineTo, 2, [x + p(i + 1), y + p(i + 2), 0.0, 0.0, 0.0, 0.0]);
                    i += 3;
                }
                2 => {
                    op(
                        ctx,
                        Id::Bezier,
                        6,
                        [
                            x + p(i + 1),
                            y + p(i + 2),
                            x + p(i + 3),
                            y + p(i + 4),
                            x + p(i + 5),
                            y + p(i + 6),
                        ],
                    );
                    i += 7;
                }
                3 => {
                    op(ctx, Id::FillPath, 0, [0.0; 6]);
                    i += 1;
                }
                4 => {
                    sets(ctx, Id::FillStyle as u32, Id::Bg as u32 + b[i + 1] as u32);
                    i += 2;
                }
                5 => {
                    op(ctx, Id::BeginPath, 0, [0.0; 6]);
                    op(
                        ctx,
                        Id::Arc,
                        5,
                        [x + p(i + 1), y + p(i + 2), p(i + 3) * 0.1, 0.0, 6.2831853, 0.0],
                    );
                    op(ctx, Id::FillPath, 0, [0.0; 6]);
                    i += 4;
                }
                _ => {
                    op(ctx, Id::BeginPath, 0, [0.0; 6]);
                    i += 1;
                }
            }
        }
    }
}
fn frame(now: f64) -> bool {
    let s = world();
    if s.t0 < 0.0 {
        s.t0 = now;
    }
    let t = (now - s.t0) * 0.001;
    let k = ease(t / FADE);
    let d = s.d;

    let cx = (s.w * 0.5) as i64 as f64;
    let cy = (s.h * 0.5 + BLOCK_DY * d) as i64 as f64;

    unsafe {
        sets(CTX, Id::FillStyle as u32, Id::Bg as u32);
        setn(CTX, Id::GAlpha as u32, 1.0);
        op(CTX, Id::FillRect, 4, [0.0, 0.0, s.w, s.h, 0.0, 0.0]);
        setn(CTX, Id::GAlpha as u32, k);

        let m = MARK * d;
        s.bird(CTX, cx - 50.0 * m, cy + BIRD_DY * d - 55.0 * m, m);

        sets(CTX, Id::FillStyle as u32, Id::White as u32);
        draw_text(Id::Headline, cx, cy, HEAD_SIZE * d, true);

        sets(CTX, Id::FillStyle as u32, Id::Grey as u32);
        draw_text(Id::Sub, cx, cy + SUB_DY * d, SUB_SIZE * d, false);
    }
    t < FADE
}

fn schedule() {
    let s = world();
    if !s.pending {
        s.pending = true;
        unsafe {
            hcall(0, WIN, Id::Raf as u32, CB_RAF, 0);
        }
    }
}
#[export_name = "11"]
pub extern "C" fn start() {
    let s = world();
    unsafe {
        for i in 0..K {
            reg(s.buf.as_ptr().add(PATH_END + OFFS[i]), LENS[i]);
        }

        get(DOC, WIN, Id::Document as u32);
        get(BODY, DOC, Id::Body as u32);
        get(OBJC, WIN, Id::ObjCtor as u32);

        lit(T1, Id::Canvas as u32);
        hcall(CANVAS, DOC, Id::Create as u32, T1, 0);
        hcall(0, BODY, Id::Append as u32, CANVAS, 0);
        get(STY, CANVAS, Id::Style as u32);
        sets(STY, Id::CssText as u32, Id::Css as u32);

        make(OPTS, OBJC, 0);
        setn(OPTS, Id::Alpha as u32, 0.0);
        lit(T1, Id::Ctx2d as u32);
        hcall(CTX, CANVAS, Id::GetCtx as u32, T1, OPTS);

        lit(T1, Id::Canvas as u32);
        hcall(ICN, DOC, Id::Create as u32, T1, 0);
        setn(ICN, Id::Width as u32, 64.0);
        setn(ICN, Id::Height as u32, 64.0);
        lit(T1, Id::Ctx2d as u32);
        hcall(ICTX, ICN, Id::GetCtx as u32, T1, 0);
        let k = 64.0 * 0.0094;
        s.bird(ICTX, 32.0 - 50.0 * k, 32.0 - 55.0 * k, k);
        get(HEAD, DOC, Id::HeadEl as u32);
        lit(T1, Id::Link as u32);
        hcall(LNK, DOC, Id::Create as u32, T1, 0);
        hcall(0, HEAD, Id::Append as u32, LNK, 0);
        sets(LNK, Id::Rel as u32, Id::Icon as u32);
        hcall(URL, ICN, Id::ToUrl as u32, 0, 0);
        seth(LNK, Id::Href as u32, URL);

        get(PERF, WIN, Id::Perf as u32);
        func(CB_MENU, 5, EV);
        seth(WIN, Id::Menu as u32, CB_MENU);
        func(CB_RAF, 35, TS);
        func(CB_RO, 47, ENTS);

        get(ROC, WIN, Id::RoCtor as u32);
        make(RO, ROC, CB_RO);
        make(OBJ2, OBJC, 0);
        sets(OBJ2, Id::Box as u32, Id::BoxVal as u32);
        if hcall(0, RO, Id::Observe as u32, CANVAS, OBJ2) == 0 {
            hcall(0, RO, Id::Observe as u32, CANVAS, 0);
        }
    }
}

#[export_name = "5"]
pub extern "C" fn on_menu() {
    unsafe {
        hcall(0, EV, Id::Prevent as u32, 0, 0);
    }
}

#[export_name = "35"]
pub extern "C" fn on_frame() {
    let s = world();
    s.pending = false;
    if frame(unsafe { num(TS) }) {
        schedule();
    }
}
#[export_name = "47"]
pub extern "C" fn on_resize() {
    let s = world();
    unsafe {
        get(ENT, ENTS, Id::Zero as u32);
        let dpr = getn(WIN, Id::Dpr as u32);
        get(ARR, ENT, Id::DpSize as u32);
        let (w, h) = if truthy(ARR) != 0 {
            get(SZ, ARR, Id::Zero as u32);
            (getn(SZ, Id::Inline as u32), getn(SZ, Id::Block as u32))
        } else {
            (
                (getn(WIN, Id::InnerW as u32) * dpr + 0.5) as i64 as f64,
                (getn(WIN, Id::InnerH as u32) * dpr + 0.5) as i64 as f64,
            )
        };
        setn(CANVAS, Id::Width as u32, w);
        setn(CANVAS, Id::Height as u32, h);
        sets(CTX, Id::Baseline as u32, Id::Middle as u32);
        sets(CTX, Id::Align as u32, Id::Center as u32);
        s.w = w;
        s.h = h;
        s.d = dpr;
        hcall(NOW, PERF, Id::Now as u32, 0, 0);
        if frame(num(NOW)) {
            schedule();
        }
    }
}
