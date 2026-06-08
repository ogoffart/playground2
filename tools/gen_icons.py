#!/usr/bin/env python3
# Codegen: translate Harbor's icons.jsx geometric glyphs into Slint Path command
# strings. Emits ui/icons.slint with a global Icons exposing path()/fill().
import math

def fmt(n):
    if isinstance(n, float) and n.is_integer():
        n = int(n)
    return str(n)

STROKE = "s"
FILL = "f"

def P(d): return (STROKE, d)
def L(x1,y1,x2,y2): return (STROKE, f"M{fmt(x1)} {fmt(y1)}L{fmt(x2)} {fmt(y2)}")
def C(cx,cy,r, fill=False):
    d = (f"M{fmt(cx-r)} {fmt(cy)}"
         f"a{fmt(r)} {fmt(r)} 0 1 0 {fmt(2*r)} 0"
         f"a{fmt(r)} {fmt(r)} 0 1 0 {fmt(-2*r)} 0z")
    return (FILL if fill else STROKE, d)
def R(x,y,w,h,rx=1.5):
    return (STROKE,
        f"M{fmt(x+rx)} {fmt(y)}"
        f"h{fmt(w-2*rx)}a{fmt(rx)} {fmt(rx)} 0 0 1 {fmt(rx)} {fmt(rx)}"
        f"v{fmt(h-2*rx)}a{fmt(rx)} {fmt(rx)} 0 0 1 {fmt(-rx)} {fmt(rx)}"
        f"h{fmt(-(w-2*rx))}a{fmt(rx)} {fmt(rx)} 0 0 1 {fmt(-rx)} {fmt(-rx)}"
        f"v{fmt(-(h-2*rx))}a{fmt(rx)} {fmt(rx)} 0 0 1 {fmt(rx)} {fmt(-rx)}z")

def docBase(): return P("M7 3h7l4 4v13a1 1 0 0 1-1 1H7a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1z M14 3v4h4")

def sun_rays():
    out=[C(12,12,4)]
    for a in [0,45,90,135,180,225,270,315]:
        r=math.radians(a)
        out.append(L(round(12+math.cos(r)*7,3),round(12+math.sin(r)*7,3),
                     round(12+math.cos(r)*9,3),round(12+math.sin(r)*9,3)))
    return out

ICONS = {
 "sidebar": [R(3,4,18,16,2), L(9,4,9,20)],
 "back": [P("M15 5l-7 7 7 7")],
 "forward": [P("M9 5l7 7-7 7")],
 "up": [P("M12 19V6"), P("M6 12l6-6 6 6")],
 "download": [P("M12 4v10"), P("M7 11l5 5 5-5"), P("M5 20h14")],
 "gear": [C(12,12,3.2), P("M12 4.5v2 M12 17.5v2 M4.5 12h2 M17.5 12h2 M6.8 6.8l1.4 1.4 M15.8 15.8l1.4 1.4 M17.2 6.8l-1.4 1.4 M8.2 15.8l-1.4 1.4")],
 "newfolder": [P("M3 7a2 2 0 0 1 2-2h4l2 2h5a2 2 0 0 1 2 2v3"), P("M3 7v10a2 2 0 0 0 2 2h7"), P("M17 15v6 M14 18h6")],
 "copy": [R(8,8,12,12,2), P("M16 8V6a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v8a2 2 0 0 0 2 2h2")],
 "rename": [P("M4 7V5h16v2"), L(12,5,12,19), L(9,19,15,19)],
 "search": [C(11,11,7), L(16.5,16.5,21,21)],
 "list": [L(8,6,20,6), L(8,12,20,12), L(8,18,20,18), C(4,6,0.6,True), C(4,12,0.6,True), C(4,18,0.6,True)],
 "grid": [R(4,4,7,7,1.5), R(13,4,7,7,1.5), R(4,13,7,7,1.5), R(13,13,7,7,1.5)],
 "columns": [R(4,4,16,16,2), L(10,4,10,20), L(15,4,15,20)],
 "sort": [P("M7 4v16"), P("M4 8l3-4 3 4"), P("M17 4v16"), P("M14 16l3 4 3-4")],
 "info": [C(12,12,8.5), L(12,11,12,16), C(12,7.6,0.7,True)],
 "sun": sun_rays(),
 "moon": [P("M20 14.5A8 8 0 1 1 9.5 4a6.5 6.5 0 0 0 10.5 10.5z")],
 "system": [R(3,4,18,12,2), L(8,20,16,20), L(12,16,12,20)],
 "ellipsis": [C(5,12,1.1,True), C(12,12,1.1,True), C(19,12,1.1,True)],
 "chevronRight": [P("M9 6l6 6-6 6")],
 "chevronDown": [P("M6 9l6 6 6-6")],
 "chevronUp": [P("M6 15l6-6 6 6")],
 "close": [L(6,6,18,18), L(18,6,6,18)],
 "min": [L(6,12,18,12)],
 "max": [R(6,6,12,12,2)],
 "eject": [P("M6 14h12L12 6z"), L(6,18,18,18)],
 "plus": [L(12,6,12,18), L(6,12,18,12)],
 "check": [P("M5 12l5 5L19 7")],
 "refresh": [P("M20 11a8 8 0 1 0-2.3 6.1"), P("M20 5v6h-6")],
 "share": [C(6,12,2.5), C(17,6,2.5), C(17,18,2.5), L(8.2,10.8,14.8,7.2), L(8.2,13.2,14.8,16.8)],
 "star": [P("M12 4l2.4 5 5.6.6-4 3.8 1.1 5.6L12 16.4 6.9 19l1.1-5.6-4-3.8 5.6-.6z")],
 "trash": [P("M5 7h14"), P("M9 7V5h6v2"), P("M7 7l1 13h8l1-13")],
 "tag": [P("M4 12V5a1 1 0 0 1 1-1h7l8 8-8 8z"), C(8,8,1.2)],
 "clock": [C(12,12,8.5), P("M12 7.5V12l3 2")],
 "airdrop": [C(12,12,8.5), P("M8.5 12a3.5 3.5 0 0 1 7 0"), P("M6 12a6 6 0 0 1 12 0"), C(12,12,0.8,True)],
 "apps": [R(4,4,7,7,1.5), R(13,4,7,7,1.5), R(4,13,7,7,1.5), C(16.5,16.5,3.5)],
 "home": [P("M4 11l8-7 8 7"), P("M6 10v9h12v-9")],
 "ssd": [R(4,6,16,12,2), L(8,18,8,14), C(16,12,1.4)],
 "hdd": [R(4,6,16,12,2), C(12,12,4), C(12,12,0.8,True)],
 "usb": [R(7,9,10,11,2), P("M9 9V6a3 3 0 0 1 6 0v3"), L(12,13,12,16)],
 "cloud": [P("M7 18a4 4 0 0 1 .4-8 5.5 5.5 0 0 1 10.6 1.4A3.5 3.5 0 0 1 17.5 18z")],
 "folder": [P("M3 7a2 2 0 0 1 2-2h4l2 2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z")],
 "folderOpen": [P("M3 7a2 2 0 0 1 2-2h4l2 2h8a2 2 0 0 1 2 2"), P("M3 9h18l-2 8a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1z")],
 "doc": [docBase(), L(9,12,15,12), L(9,15,15,15), L(9,18,13,18)],
 "text": [docBase(), L(9,11,15,11), L(9,14,15,14), L(9,17,12,17)],
 "pdf": [docBase(), P("M9 17v-4h1.5a1.2 1.2 0 0 1 0 2.4H9")],
 "sheet": [docBase(), L(9,12,15,12), L(9,15,15,15), L(12,11,12,18)],
 "slides": [docBase(), R(9,12,6,5,1)],
 "code": [docBase(), P("M10.5 12.5L9 15l1.5 2.5"), P("M13.5 12.5L15 15l-1.5 2.5")],
 "image": [R(4,5,16,14,2), C(9,10,1.6), P("M5 18l4.5-4.5L13 17l3-3 3 3")],
 "audio": [P("M9 17V6l9-2v11"), C(7,17,2), C(16,15,2)],
 "video": [R(3,6,18,12,2), P("M10 9.5l5 2.5-5 2.5z")],
 "archive": [P("M5 7a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2z"), L(12,5,12,9), R(10.5,9,3,3,0.6)],
 "disk": [C(12,12,8.5), C(12,12,2.4), L(12,3.5,12,7)],
 "app": [R(4,4,16,16,4), C(12,12,3.4)],
 "font": [docBase(), P("M9.5 17l2.5-6 2.5 6"), L(10.3,15,13.7,15)],
}

def collect(parts, kind):
    return " ".join(d for (k,d) in parts if k==kind)

lines = []
lines.append("// AUTO-GENERATED from Harbor icons.jsx by tools/gen_icons.py. Do not edit by hand.")
lines.append("// 24x24 geometric line glyphs as Slint Path command strings.")
lines.append("export global Icons {")
for fn, kind in [("path", STROKE), ("fill", FILL)]:
    lines.append(f"    pure public function {fn}(name: string) -> string {{")
    for name, parts in ICONS.items():
        d = collect(parts, kind)
        if d:
            lines.append(f'        if (name == "{name}") {{ return "{d}"; }}')
    lines.append('        return "";')
    lines.append("    }")
lines.append("}")

import os
os.makedirs("/home/user/playground2/ui", exist_ok=True)
with open("/home/user/playground2/ui/icons.slint","w") as fh:
    fh.write("\n".join(lines)+"\n")
print("wrote ui/icons.slint with", len(ICONS), "icons")
