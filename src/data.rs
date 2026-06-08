//! Harbor mock filesystem + formatting helpers (ported from data.js).
use chrono::{Local, TimeZone};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub const KB: f64 = 1024.0;
pub const MB: f64 = KB * 1024.0;
pub const GB: f64 = MB * 1024.0;
const DAY_MS: f64 = 86_400_000.0;
const HOUR_MS: f64 = 3_600_000.0;

#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // App/Font kinds are part of the kind table even if absent from the mock tree
pub enum Kind {
    Folder, Image, Raw, Pdf, Doc, Sheet, Slides, Text, Code, Audio, Video, Archive, Dmg, App, Font,
}

impl Kind {
    pub fn label(self) -> &'static str {
        match self {
            Kind::Folder => "Folder",
            Kind::Image => "Image",
            Kind::Raw => "RAW Image",
            Kind::Pdf => "PDF Document",
            Kind::Doc => "Document",
            Kind::Sheet => "Spreadsheet",
            Kind::Slides => "Presentation",
            Kind::Text => "Plain Text",
            Kind::Code => "Source Code",
            Kind::Audio => "Audio",
            Kind::Video => "Video",
            Kind::Archive => "Archive",
            Kind::Dmg => "Disk Image",
            Kind::App => "Application",
            Kind::Font => "Font",
        }
    }
    pub fn icon(self) -> &'static str {
        match self {
            Kind::Folder => "folder",
            Kind::Image | Kind::Raw => "image",
            Kind::Pdf => "pdf",
            Kind::Doc => "doc",
            Kind::Sheet => "sheet",
            Kind::Slides => "slides",
            Kind::Text => "text",
            Kind::Code => "code",
            Kind::Audio => "audio",
            Kind::Video => "video",
            Kind::Archive => "archive",
            Kind::Dmg => "disk",
            Kind::App => "app",
            Kind::Font => "font",
        }
    }
    pub fn is_media(self) -> bool {
        matches!(self, Kind::Image | Kind::Raw | Kind::Video | Kind::Audio)
    }
}

pub type NodeRef = Rc<RefCell<Node>>;

pub struct Node {
    pub id: i32,
    pub name: String,
    pub kind: Kind,
    pub size: f64,
    pub modified: f64, // ms since epoch
    pub is_folder: bool,
    pub tags: Vec<i32>, // color indices 0..6
    pub children: Option<Vec<NodeRef>>,
    pub downloading: bool,
    pub progress: f32,
    pub dims: Option<String>,
}

pub struct Drive {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub total: f64,
    pub used: f64,
    pub system: bool,
    pub ejectable: bool,
}

pub fn tag_index(key: &str) -> i32 {
    match key {
        "red" => 0, "orange" => 1, "yellow" => 2, "green" => 3,
        "blue" => 4, "purple" => 5, "graphite" => 6, _ => 6,
    }
}
pub fn tag_key(i: i32) -> &'static str {
    ["red", "orange", "yellow", "green", "blue", "purple", "graphite"]
        .get(i as usize).copied().unwrap_or("graphite")
}
pub fn tag_name(i: i32) -> &'static str {
    ["Red", "Orange", "Yellow", "Green", "Blue", "Purple", "Graphite"]
        .get(i as usize).copied().unwrap_or("")
}

/// Builder context carrying the current time and an id counter.
pub struct Builder {
    pub now: f64,
    pub next_id: Cell<i32>,
}

impl Builder {
    fn ago(&self, d: f64, h: f64) -> f64 {
        self.now - d * DAY_MS - h * HOUR_MS
    }
    fn id(&self) -> i32 {
        let n = self.next_id.get() + 1;
        self.next_id.set(n);
        n
    }
    fn file(&self, name: &str, kind: Kind, size: f64, modified: f64) -> NodeRef {
        Rc::new(RefCell::new(Node {
            id: self.id(),
            name: name.into(),
            kind,
            size,
            modified,
            is_folder: kind == Kind::Folder,
            tags: vec![],
            children: if kind == Kind::Folder { Some(vec![]) } else { None },
            downloading: false,
            progress: 0.0,
            dims: None,
        }))
    }
}

fn with(node: &NodeRef, f: impl FnOnce(&mut Node)) -> NodeRef {
    f(&mut node.borrow_mut());
    node.clone()
}
fn tags(keys: &[&str]) -> Vec<i32> {
    keys.iter().map(|k| tag_index(k)).collect()
}
fn folder(b: &Builder, name: &str, modified: f64, children: Vec<NodeRef>) -> NodeRef {
    let n = b.file(name, Kind::Folder, 0.0, modified);
    n.borrow_mut().children = Some(children);
    n
}

/// Build the believable home tree. Returns (root, drives, builder-with-next-id).
pub fn build() -> (NodeRef, Vec<Drive>, i32, f64) {
    let now = Local::now().timestamp_millis() as f64;
    let b = Builder { now, next_id: Cell::new(0) };

    macro_rules! f {
        ($name:expr, $kind:expr, $size:expr, $d:expr, $h:expr) => {{
            let m = b.ago($d as f64, $h as f64);
            b.file($name, $kind, $size, m)
        }};
    }

    let tree = folder(&b, "Harbor", b.ago(0.0, 0.0), vec![
        {
            let desktop = folder(&b, "Desktop", b.ago(0.0, 2.0), vec![
                with(&f!("Screenshot 2026-06-08 at 09.14.png", Kind::Image, 2.1 * MB, 0, 3),
                    |n| { n.tags = tags(&["blue"]); n.dims = Some("2880 × 1800".into()); }),
                with(&f!("Screenshot 2026-06-07 at 17.40.png", Kind::Image, 1.7 * MB, 1, 0),
                    |n| { n.dims = Some("2880 × 1800".into()); }),
                f!("scratch.txt", Kind::Text, 412.0, 0, 6),
                folder(&b, "Inbox", b.ago(2.0, 0.0), vec![
                    with(&f!("offer-letter.pdf", Kind::Pdf, 88.0 * KB, 2, 0), |n| n.tags = tags(&["green"])),
                    f!("nda-signed.pdf", Kind::Pdf, 240.0 * KB, 3, 0),
                ]),
            ]);
            desktop
        },
        folder(&b, "Documents", b.ago(1.0, 0.0), vec![
            folder(&b, "Invoices", b.ago(4.0, 0.0), vec![
                with(&f!("INV-2026-031.pdf", Kind::Pdf, 64.0 * KB, 4, 0), |n| n.tags = tags(&["yellow"])),
                f!("INV-2026-030.pdf", Kind::Pdf, 61.0 * KB, 12, 0),
                f!("INV-2026-029.pdf", Kind::Pdf, 59.0 * KB, 20, 0),
                with(&f!("summary-Q2.numbers", Kind::Sheet, 1.2 * MB, 5, 0), |n| n.tags = tags(&["green"])),
            ]),
            folder(&b, "Contracts", b.ago(9.0, 0.0), vec![
                with(&f!("vendor-agreement.pdf", Kind::Pdf, 320.0 * KB, 9, 0), |n| n.tags = tags(&["red"])),
                f!("lease-2026.pdf", Kind::Pdf, 1.1 * MB, 30, 0),
            ]),
            with(&f!("Roadmap 2026.key", Kind::Slides, 24.0 * MB, 2, 0), |n| n.tags = tags(&["purple"])),
            with(&f!("Budget.numbers", Kind::Sheet, 880.0 * KB, 3, 0), |n| n.tags = tags(&["green"])),
            f!("Meeting notes.docx", Kind::Doc, 48.0 * KB, 0, 5),
            f!("Resume.pdf", Kind::Pdf, 96.0 * KB, 40, 0),
            f!("README.md", Kind::Text, 3.4 * KB, 6, 0),
        ]),
        folder(&b, "Downloads", b.ago(0.0, 1.0), vec![
            with(&f!("Sintel-4K-master.mov", Kind::Video, 4.7 * GB, 0, 0),
                |n| { n.downloading = true; n.progress = 0.34; n.tags = tags(&["blue"]); }),
            f!("rust-1.89-x86_64.dmg", Kind::Dmg, 312.0 * MB, 0, 2),
            with(&f!("design-assets.zip", Kind::Archive, 1.8 * GB, 0, 4), |n| n.tags = tags(&["orange"])),
            f!("slint-1.7.0.tar.gz", Kind::Archive, 42.0 * MB, 1, 0),
            f!("invoice-template.xlsx", Kind::Sheet, 120.0 * KB, 2, 0),
            folder(&b, "wallpaper-pack", b.ago(3.0, 0.0), vec![
                with(&f!("ridge.jpg", Kind::Image, 6.2 * MB, 3, 0), |n| n.dims = Some("5120 × 2880".into())),
                with(&f!("dunes.jpg", Kind::Image, 5.1 * MB, 3, 0), |n| n.dims = Some("5120 × 2880".into())),
            ]),
        ]),
        folder(&b, "Pictures", b.ago(2.0, 0.0), vec![
            folder(&b, "2026", b.ago(2.0, 0.0), vec![
                with(&f!("DSC_4021.RAF", Kind::Raw, 48.0 * MB, 2, 0), |n| { n.dims = Some("6240 × 4160".into()); n.tags = tags(&["orange"]); }),
                with(&f!("DSC_4022.RAF", Kind::Raw, 47.0 * MB, 2, 0), |n| n.dims = Some("6240 × 4160".into())),
                with(&f!("DSC_4023.RAF", Kind::Raw, 49.0 * MB, 2, 0), |n| n.dims = Some("6240 × 4160".into())),
                with(&f!("edit-01.jpg", Kind::Image, 8.8 * MB, 1, 0), |n| { n.dims = Some("6240 × 4160".into()); n.tags = tags(&["purple"]); }),
            ]),
            folder(&b, "Avatars", b.ago(15.0, 0.0), vec![
                with(&f!("me-round.png", Kind::Image, 420.0 * KB, 15, 0), |n| n.dims = Some("512 × 512".into())),
            ]),
            with(&f!("favicon.png", Kind::Image, 12.0 * KB, 22, 0), |n| n.dims = Some("256 × 256".into())),
        ]),
        folder(&b, "Projects", b.ago(0.0, 4.0), vec![
            with(&folder(&b, "harbor-ui", b.ago(0.0, 4.0), vec![
                folder(&b, "src", b.ago(0.0, 4.0), vec![
                    f!("main.rs", Kind::Code, 6.1 * KB, 0, 4),
                    with(&f!("window.slint", Kind::Code, 14.0 * KB, 0, 4), |n| n.tags = tags(&["blue"])),
                    f!("sidebar.slint", Kind::Code, 9.2 * KB, 0, 5),
                    f!("theme.slint", Kind::Code, 4.8 * KB, 0, 6),
                ]),
                f!("Cargo.toml", Kind::Text, 1.1 * KB, 0, 4),
                f!("README.md", Kind::Text, 5.6 * KB, 1, 0),
                f!(".gitignore", Kind::Text, 280.0, 8, 0),
            ]), |n| n.tags = tags(&["blue"])),
            folder(&b, "atlas-api", b.ago(3.0, 0.0), vec![
                f!("server.ts", Kind::Code, 12.0 * KB, 3, 0),
                f!("schema.sql", Kind::Code, 8.4 * KB, 4, 0),
                f!("package.json", Kind::Code, 2.2 * KB, 3, 0),
            ]),
            with(&f!("sketches.fig", Kind::Doc, 18.0 * MB, 7, 0), |n| n.tags = tags(&["purple"])),
        ]),
        folder(&b, "Music", b.ago(11.0, 0.0), vec![
            folder(&b, "Tycho — Weather", b.ago(11.0, 0.0), vec![
                f!("01 Easy.flac", Kind::Audio, 38.0 * MB, 11, 0),
                f!("02 Pink & Blue.flac", Kind::Audio, 41.0 * MB, 11, 0),
                f!("03 Skate.flac", Kind::Audio, 36.0 * MB, 11, 0),
                with(&f!("cover.jpg", Kind::Image, 1.4 * MB, 11, 0), |n| n.dims = Some("1400 × 1400".into())),
            ]),
            f!("voice-memo-03.m4a", Kind::Audio, 4.2 * MB, 5, 0),
        ]),
        folder(&b, "Movies", b.ago(6.0, 0.0), vec![
            with(&f!("interview-final.mov", Kind::Video, 2.4 * GB, 6, 0), |n| { n.tags = tags(&["red"]); n.dims = Some("3840 × 2160".into()); }),
            with(&f!("b-roll-01.mp4", Kind::Video, 880.0 * MB, 8, 0), |n| n.dims = Some("1920 × 1080".into())),
            with(&f!("export-draft.mp4", Kind::Video, 1.2 * GB, 7, 0), |n| n.dims = Some("3840 × 2160".into())),
        ]),
        folder(&b, "Fonts", b.ago(60.0, 0.0), vec![
            f!("Inter-Variable.ttf", Kind::Font, 1.1 * MB, 60, 0),
            f!("IBMPlexMono.ttf", Kind::Font, 480.0 * KB, 60, 0),
        ]),
    ]);

    let drives = vec![
        Drive { id: "macintosh".into(), name: "Macintosh HD".into(), icon: "ssd".into(), total: 1000.0 * GB, used: 612.0 * GB, system: true, ejectable: false },
        Drive { id: "timecap".into(), name: "Time Capsule".into(), icon: "hdd".into(), total: 4000.0 * GB, used: 2870.0 * GB, system: false, ejectable: false },
        Drive { id: "field".into(), name: "FIELD-SSD".into(), icon: "usb".into(), total: 2000.0 * GB, used: 340.0 * GB, system: false, ejectable: true },
        Drive { id: "icloud".into(), name: "iCloud Drive".into(), icon: "cloud".into(), total: 200.0 * GB, used: 142.0 * GB, system: false, ejectable: false },
    ];

    let next_id = b.next_id.get();
    (tree, drives, next_id, now)
}

// ---- helpers ----
pub fn format_bytes(b: f64) -> String {
    let fixed = |v: f64, dp: usize| -> String { format!("{:.*}", dp, v) };
    if b == 0.0 {
        return "—".into();
    }
    if b < KB {
        return format!("{} B", b.round() as i64);
    }
    if b < MB {
        return format!("{} KB", fixed(b / KB, if b < 10.0 * KB { 1 } else { 0 }));
    }
    if b < GB {
        return format!("{} MB", fixed(b / MB, if b < 10.0 * MB { 1 } else { 0 }));
    }
    format!("{} GB", fixed(b / GB, if b < 10.0 * GB { 2 } else { 1 }))
}

pub fn folder_size(node: &Node) -> f64 {
    if !node.is_folder {
        return node.size;
    }
    match &node.children {
        Some(ch) => ch.iter().map(|c| {
            let c = c.borrow();
            if c.is_folder { folder_size(&c) } else { c.size }
        }).sum(),
        None => 0.0,
    }
}

pub fn count_items(node: &Node) -> usize {
    node.children.as_ref().map(|c| c.len()).unwrap_or(0)
}

pub fn format_date(modified: f64, now: f64) -> String {
    let diff = now - modified;
    let mins = (diff / 60000.0).floor() as i64;
    if mins < 1 {
        return "Just now".into();
    }
    if mins < 60 {
        return format!("{} min ago", mins);
    }
    let hrs = mins / 60;
    if hrs < 24 {
        return format!("{} {} ago", hrs, if hrs == 1 { "hour" } else { "hours" });
    }
    let days = hrs / 24;
    if days < 7 {
        return format!("{} {} ago", days, if days == 1 { "day" } else { "days" });
    }
    Local.timestamp_millis_opt(modified as i64).single()
        .map(|dt| dt.format("%d %b %Y").to_string())
        .unwrap_or_default()
}

pub fn full_date(modified: f64) -> String {
    Local.timestamp_millis_opt(modified as i64).single()
        .map(|dt| dt.format("%a, %b %d, %Y, %I:%M %p").to_string())
        .unwrap_or_default()
}
