use std::collections::HashMap;

use gpui::{Bounds, Hsla, Path, Pixels, Window, fill, point, px, size};

use super::RenderCell;

pub fn is_custom_block_supported(c: char) -> bool {
    match c as u32 {
        0x2580..=0x258F | 0x2590 | 0x2594..=0x259F => true, // Block Elements
        0x2500 | 0x2502 | 0x250C | 0x2510 | 0x2514 | 0x2518 | 0x251C | 0x2524 | 0x252C | 0x2534
        | 0x253C => true, // Light lines
        0x2501 | 0x2503 | 0x250F | 0x2513 | 0x2517 | 0x251B | 0x2523 | 0x252B | 0x2533 | 0x253B
        | 0x254B => true, // Heavy lines
        0xE0B0..=0xE0B6 => true,                            // Powerline
        _ => false,
    }
}

pub fn paint_custom_block(
    window: &mut Window,
    c: char,
    raw_x: f32,
    raw_y: f32,
    raw_w: f32,
    raw_h: f32,
    color: Hsla,
) -> bool {
    let scale = window.scale_factor();

    let x = px((raw_x * scale).round() / scale);
    let y = px((raw_y * scale).round() / scale);
    let w = px(((raw_x + raw_w) * scale).round() / scale) - x;
    let h = px(((raw_y + raw_h) * scale).round() / scale) - y;

    let mut painted = false;

    // Helper to paint a quad snapping its true physical boundaries to integer physical pixels
    let mut paint_quad = |qx: Pixels, qy: Pixels, qw: Pixels, qh: Pixels| {
        let l = px((qx.as_f32() * scale).round() / scale);
        let t = px((qy.as_f32() * scale).round() / scale);
        let r = px(((qx + qw).as_f32() * scale).round() / scale);
        let b = px(((qy + qh).as_f32() * scale).round() / scale);
        window.paint_quad(fill(Bounds::new(point(l, t), size(r - l, b - t)), color));
        painted = true;
    };

    let px_1 = px(1.0);
    let px_2 = px(2.0);

    // Box drawing thickness
    let light = (w * 0.1).max(px_1);
    let heavy = (w * 0.2).max(px_2);
    let cx = x + w / 2.0;
    let cy = y + h / 2.0;

    match c as u32 {
        // --- Block Elements (U+2580..=U+259F) ---
        0x2580 => paint_quad(x, y, w, h / 2.0),
        0x2581 => paint_quad(x, y + h * 7.0 / 8.0, w, h / 8.0),
        0x2582 => paint_quad(x, y + h * 3.0 / 4.0, w, h / 4.0),
        0x2583 => paint_quad(x, y + h * 5.0 / 8.0, w, h * 3.0 / 8.0),
        0x2584 => paint_quad(x, y + h / 2.0, w, h / 2.0),
        0x2585 => paint_quad(x, y + h * 3.0 / 8.0, w, h * 5.0 / 8.0),
        0x2586 => paint_quad(x, y + h / 4.0, w, h * 3.0 / 4.0),
        0x2587 => paint_quad(x, y + h / 8.0, w, h * 7.0 / 8.0),
        0x2588 => paint_quad(x, y, w, h),
        0x2589 => paint_quad(x, y, w * 7.0 / 8.0, h),
        0x258A => paint_quad(x, y, w * 3.0 / 4.0, h),
        0x258B => paint_quad(x, y, w * 5.0 / 8.0, h),
        0x258C => paint_quad(x, y, w / 2.0, h),
        0x258D => paint_quad(x, y, w * 3.0 / 8.0, h),
        0x258E => paint_quad(x, y, w / 4.0, h),
        0x258F => paint_quad(x, y, w / 8.0, h),
        0x2590 => paint_quad(x + w / 2.0, y, w / 2.0, h),
        0x2594 => paint_quad(x, y, w, h / 8.0),
        0x2595 => paint_quad(x + w * 7.0 / 8.0, y, w / 8.0, h),
        0x2596 => paint_quad(x, y + h / 2.0, w / 2.0, h / 2.0),
        0x2597 => paint_quad(x + w / 2.0, y + h / 2.0, w / 2.0, h / 2.0),
        0x2598 => paint_quad(x, y, w / 2.0, h / 2.0),
        0x2599 => {
            paint_quad(x, y, w / 2.0, h);
            paint_quad(x + w / 2.0, y + h / 2.0, w / 2.0, h / 2.0);
        }
        0x259A => {
            paint_quad(x, y, w / 2.0, h / 2.0);
            paint_quad(x + w / 2.0, y + h / 2.0, w / 2.0, h / 2.0);
        }
        0x259B => {
            paint_quad(x, y, w, h / 2.0);
            paint_quad(x, y + h / 2.0, w / 2.0, h / 2.0);
        }
        0x259C => {
            paint_quad(x, y, w, h / 2.0);
            paint_quad(x + w / 2.0, y + h / 2.0, w / 2.0, h / 2.0);
        }
        0x259D => paint_quad(x + w / 2.0, y, w / 2.0, h / 2.0),
        0x259E => {
            paint_quad(x + w / 2.0, y, w / 2.0, h / 2.0);
            paint_quad(x, y + h / 2.0, w / 2.0, h / 2.0);
        }
        0x259F => {
            paint_quad(x + w / 2.0, y, w / 2.0, h);
            paint_quad(x, y + h / 2.0, w / 2.0, h / 2.0);
        }

        // --- Basic Box Drawing (U+2500..=U+257F) ---
        // Light lines
        0x2500 => paint_quad(x, cy - light / 2.0, w, light), // Horizontal
        0x2502 => paint_quad(cx - light / 2.0, y, light, h), // Vertical
        0x250C => {
            // Down + Right
            paint_quad(
                cx - light / 2.0,
                cy - light / 2.0,
                light,
                h / 2.0 + light / 2.0,
            );
            paint_quad(
                cx - light / 2.0,
                cy - light / 2.0,
                w / 2.0 + light / 2.0,
                light,
            );
        }
        0x2510 => {
            // Down + Left
            paint_quad(
                cx - light / 2.0,
                cy - light / 2.0,
                light,
                h / 2.0 + light / 2.0,
            );
            paint_quad(x, cy - light / 2.0, w / 2.0 + light / 2.0, light);
        }
        0x2514 => {
            // Up + Right
            paint_quad(cx - light / 2.0, y, light, h / 2.0 + light / 2.0);
            paint_quad(
                cx - light / 2.0,
                cy - light / 2.0,
                w / 2.0 + light / 2.0,
                light,
            );
        }
        0x2518 => {
            // Up + Left
            paint_quad(cx - light / 2.0, y, light, h / 2.0 + light / 2.0);
            paint_quad(x, cy - light / 2.0, w / 2.0 + light / 2.0, light);
        }
        0x251C => {
            // Vertical + Right
            paint_quad(cx - light / 2.0, y, light, h);
            paint_quad(
                cx - light / 2.0,
                cy - light / 2.0,
                w / 2.0 + light / 2.0,
                light,
            );
        }
        0x2524 => {
            // Vertical + Left
            paint_quad(cx - light / 2.0, y, light, h);
            paint_quad(x, cy - light / 2.0, w / 2.0 + light / 2.0, light);
        }
        0x252C => {
            // Horizontal + Down
            paint_quad(x, cy - light / 2.0, w, light);
            paint_quad(
                cx - light / 2.0,
                cy - light / 2.0,
                light,
                h / 2.0 + light / 2.0,
            );
        }
        0x2534 => {
            // Horizontal + Up
            paint_quad(x, cy - light / 2.0, w, light);
            paint_quad(cx - light / 2.0, y, light, h / 2.0 + light / 2.0);
        }
        0x253C => {
            // Vertical + Horizontal (Cross)
            paint_quad(x, cy - light / 2.0, w, light);
            paint_quad(cx - light / 2.0, y, light, h);
        }

        // Heavy lines
        0x2501 => paint_quad(x, cy - heavy / 2.0, w, heavy), // Heavy Horizontal
        0x2503 => paint_quad(cx - heavy / 2.0, y, heavy, h), // Heavy Vertical
        0x250F => {
            // Heavy Down + Right
            paint_quad(
                cx - heavy / 2.0,
                cy - heavy / 2.0,
                heavy,
                h / 2.0 + heavy / 2.0,
            );
            paint_quad(
                cx - heavy / 2.0,
                cy - heavy / 2.0,
                w / 2.0 + heavy / 2.0,
                heavy,
            );
        }
        0x2513 => {
            // Heavy Down + Left
            paint_quad(
                cx - heavy / 2.0,
                cy - heavy / 2.0,
                heavy,
                h / 2.0 + heavy / 2.0,
            );
            paint_quad(x, cy - heavy / 2.0, w / 2.0 + heavy / 2.0, heavy);
        }
        0x2517 => {
            // Heavy Up + Right
            paint_quad(cx - heavy / 2.0, y, heavy, h / 2.0 + heavy / 2.0);
            paint_quad(
                cx - heavy / 2.0,
                cy - heavy / 2.0,
                w / 2.0 + heavy / 2.0,
                heavy,
            );
        }
        0x251B => {
            // Heavy Up + Left
            paint_quad(cx - heavy / 2.0, y, heavy, h / 2.0 + heavy / 2.0);
            paint_quad(x, cy - heavy / 2.0, w / 2.0 + heavy / 2.0, heavy);
        }
        0x2523 => {
            // Heavy Vertical + Right
            paint_quad(cx - heavy / 2.0, y, heavy, h);
            paint_quad(
                cx - heavy / 2.0,
                cy - heavy / 2.0,
                w / 2.0 + heavy / 2.0,
                heavy,
            );
        }
        0x252B => {
            // Heavy Vertical + Left
            paint_quad(cx - heavy / 2.0, y, heavy, h);
            paint_quad(x, cy - heavy / 2.0, w / 2.0 + heavy / 2.0, heavy);
        }
        0x2533 => {
            // Heavy Horizontal + Down
            paint_quad(x, cy - heavy / 2.0, w, heavy);
            paint_quad(
                cx - heavy / 2.0,
                cy - heavy / 2.0,
                heavy,
                h / 2.0 + heavy / 2.0,
            );
        }
        0x253B => {
            // Heavy Horizontal + Up
            paint_quad(x, cy - heavy / 2.0, w, heavy);
            paint_quad(cx - heavy / 2.0, y, heavy, h / 2.0 + heavy / 2.0);
        }
        0x254B => {
            // Heavy Vertical + Horizontal (Cross)
            paint_quad(x, cy - heavy / 2.0, w, heavy);
            paint_quad(cx - heavy / 2.0, y, heavy, h);
        }

        // --- Powerline ---
        0xE0B0 => {
            // Rightward Solid Arrow
            let mut path = Path::new(point(x, y));
            path.line_to(point(x + w, y + h / 2.0));
            path.line_to(point(x, y + h));
            path.line_to(point(x, y));
            window.paint_path(path, color);
            painted = true;
        }
        0xE0B2 => {
            // Leftward Solid Arrow
            let mut path = Path::new(point(x + w, y));
            path.line_to(point(x, y + h / 2.0));
            path.line_to(point(x + w, y + h));
            path.line_to(point(x + w, y));
            window.paint_path(path, color);
            painted = true;
        }
        0xE0B1 => {
            // Rightward Line Arrow
            let t = px(1.0);
            let mut p = Path::new(point(x, y));
            p.line_to(point(x + w, y + h / 2.0));
            p.line_to(point(x + w - t, y + h / 2.0));
            p.line_to(point(x, y + t));
            p.line_to(point(x, y));
            window.paint_path(p, color);
            let mut p2 = Path::new(point(x, y + h));
            p2.line_to(point(x + w, y + h / 2.0));
            p2.line_to(point(x + w - t, y + h / 2.0));
            p2.line_to(point(x, y + h - t));
            p2.line_to(point(x, y + h));
            window.paint_path(p2, color);
            painted = true;
        }
        0xE0B3 => {
            // Leftward Line Arrow
            let t = px(1.0);
            let mut p = Path::new(point(x + w, y));
            p.line_to(point(x, y + h / 2.0));
            p.line_to(point(x + t, y + h / 2.0));
            p.line_to(point(x + w, y + t));
            p.line_to(point(x + w, y));
            window.paint_path(p, color);
            let mut p2 = Path::new(point(x + w, y + h));
            p2.line_to(point(x, y + h / 2.0));
            p2.line_to(point(x + t, y + h / 2.0));
            p2.line_to(point(x + w, y + h - t));
            p2.line_to(point(x + w, y + h));
            window.paint_path(p2, color);
            painted = true;
        }

        // Half arches (Powerline rounded)
        0xE0B4 => {
            // Rightward Solid Semicircle
            let mut path = Path::new(point(x, y));
            path.curve_to(point(x, y + h), point(x + w * 2.0, y + h / 2.0));
            path.line_to(point(x, y));
            window.paint_path(path, color);
            painted = true;
        }
        0xE0B6 => {
            // Leftward Solid Semicircle
            let mut path = Path::new(point(x + w, y));
            path.curve_to(point(x + w, y + h), point(x - w, y + h / 2.0));
            path.line_to(point(x + w, y));
            window.paint_path(path, color);
            painted = true;
        }

        _ => {}
    }

    painted
}

// ---------------------------------------------------------------------------
// Terminal keyword / pattern highlighting
// ---------------------------------------------------------------------------

fn hsla(r: u8, g: u8, b: u8) -> Hsla {
    Hsla {
        h: 0.0,
        s: 0.0,
        l: 0.0,
        a: 1.0,
    }
    .into_rgba_like(r, g, b)
}

trait HslaExt {
    fn into_rgba_like(self, r: u8, g: u8, b: u8) -> Self;
}

impl HslaExt for Hsla {
    fn into_rgba_like(self, r: u8, g: u8, b: u8) -> Self {
        let rf = r as f32 / 255.0;
        let gf = g as f32 / 255.0;
        let bf = b as f32 / 255.0;
        let max = rf.max(gf).max(bf);
        let min = rf.min(gf).min(bf);
        let l = (max + min) / 2.0;
        if max == min {
            return Hsla {
                h: 0.0,
                s: 0.0,
                l,
                a: 1.0,
            };
        }
        let d = max - min;
        let s = if l > 0.5 {
            d / (2.0 - max - min)
        } else {
            d / (max + min)
        };
        let h = if max == rf {
            ((gf - bf) / d + if gf < bf { 6.0 } else { 0.0 }) / 6.0
        } else if max == gf {
            ((bf - rf) / d + 2.0) / 6.0
        } else {
            ((rf - gf) / d + 4.0) / 6.0
        };
        Hsla { h, s, l, a: 1.0 }
    }
}

/// Highlight colors for common terminal keywords and patterns.
struct HighlightColors {
    error: Hsla,      // ERROR, FATAL, CRITICAL, PANIC
    success: Hsla,    // SUCCESS, OK, PASS
    warning: Hsla,    // WARN, WARNING
    info: Hsla,       // INFO, NOTICE
    failure: Hsla,    // FAIL, FAILED, DENIED, REJECTED, TIMEOUT
    network: Hsla,    // IP addresses
    url: Hsla,        // http://, https://
    port: Hsla,       // :22, :443, etc.
    debug: Hsla,      // DEBUG, DBG, TRACE
}

fn highlight_colors() -> HighlightColors {
    HighlightColors {
        error:   hsla(224,  96,  96), // #E06060 red
        success: hsla(126, 198, 153), // #7EC699 green
        warning: hsla(232, 201, 122), // #E8C97A yellow
        info:    hsla(108, 180, 238), // #6CB4EE blue
        failure: hsla(232, 168, 124), // #E8A87C orange
        network: hsla(199, 146, 234), // #C792EA purple
        url:     hsla( 86, 212, 199), // #56D4C7 teal
        port:    hsla(130, 170, 200), // #82AAC8 muted teal
        debug:   hsla(130, 140, 155), // #828C9B gray
    }
}

/// Check if `c` is a word boundary (not alphanumeric or underscore).
fn is_boundary(c: char) -> bool {
    !c.is_ascii_alphanumeric() && c != '_'
}

/// Scan terminal cells for common keywords and patterns, returning a map of
/// `(row, col) -> highlight_color` for cells that should be recolored.
/// If `search_map` is provided, search match colors take priority over keyword colors.
pub fn highlight_cells(
    cells: &[RenderCell],
    rows: usize,
    search_map: Option<&HashMap<(i32, i32), Hsla>>,
) -> HashMap<(i32, i32), Hsla> {
    let colors = highlight_colors();

    // Build a per-row char array with cell column tracking.
    // row_chars[row] = Vec<(col, char)>
    let mut row_chars: Vec<Vec<(i32, char)>> = vec![vec![]; rows];
    for rc in cells {
        if rc.row < 0 || (rc.row as usize) >= rows {
            continue;
        }
        row_chars[rc.row as usize].push((rc.col, rc.cell.c));
    }
    for row in row_chars.iter_mut() {
        row.sort_by_key(|&(col, _)| col);
    }

    let mut map = HashMap::new();

    for (row_idx, row) in row_chars.iter().enumerate() {
        if row.is_empty() {
            continue;
        }
        let row_i32 = row_idx as i32;

        // Build the text string and a byte-offset → column index lookup.
        let mut chars_buf = String::with_capacity(row.len());
        let mut byte_to_col: Vec<i32> = Vec::new();
        for &(col, c) in row {
            chars_buf.push(c);
            // Pad the mapping so every byte of this char points to `col`.
            while byte_to_col.len() < chars_buf.len() {
                byte_to_col.push(col);
            }
        }
        let text = chars_buf.as_str();

        // ── 1. Error keywords (highest priority) ──────────────────────────
        for kw in &["EMERGENCY", "CRITICAL", "FATAL", "PANIC", "ERROR", "ERR"] {
            for m in find_keyword(text, kw) {
                let start_col = byte_to_col[m];
                let end_col = byte_to_col[(m + kw.len()).min(byte_to_col.len() - 1)];
                for c in start_col..=end_col {
                    map.entry((row_i32, c)).or_insert(colors.error);
                }
            }
        }

        // ── 2. Success keywords ───────────────────────────────────────────
        for kw in &["SUCCESS", "SUCCEEDED", "PASSED", "PASS", "OK"] {
            for m in find_keyword(text, kw) {
                let start_col = byte_to_col[m];
                let end_col = byte_to_col[(m + kw.len()).min(byte_to_col.len() - 1)];
                for c in start_col..=end_col {
                    map.entry((row_i32, c)).or_insert(colors.success);
                }
            }
        }

        // ── 3. Failure keywords ───────────────────────────────────────────
        for kw in &["FAILED", "FAILURE", "DENIED", "REJECTED", "TIMEOUT", "FAIL"] {
            for m in find_keyword(text, kw) {
                let start_col = byte_to_col[m];
                let end_col = byte_to_col[(m + kw.len()).min(byte_to_col.len() - 1)];
                for c in start_col..=end_col {
                    map.entry((row_i32, c)).or_insert(colors.failure);
                }
            }
        }

        // ── 4. Warning keywords ───────────────────────────────────────────
        for kw in &["WARNING", "WARN"] {
            for m in find_keyword(text, kw) {
                let start_col = byte_to_col[m];
                let end_col = byte_to_col[(m + kw.len()).min(byte_to_col.len() - 1)];
                for c in start_col..=end_col {
                    map.entry((row_i32, c)).or_insert(colors.warning);
                }
            }
        }

        // ── 5. Info keywords ──────────────────────────────────────────────
        for kw in &["NOTICE", "INFO"] {
            for m in find_keyword(text, kw) {
                let start_col = byte_to_col[m];
                let end_col = byte_to_col[(m + kw.len()).min(byte_to_col.len() - 1)];
                for c in start_col..=end_col {
                    map.entry((row_i32, c)).or_insert(colors.info);
                }
            }
        }

        // ── 6. Debug keywords ─────────────────────────────────────────────
        for kw in &["DEBUG", "DBG", "TRACE"] {
            for m in find_keyword(text, kw) {
                let start_col = byte_to_col[m];
                let end_col = byte_to_col[(m + kw.len()).min(byte_to_col.len() - 1)];
                for c in start_col..=end_col {
                    map.entry((row_i32, c)).or_insert(colors.debug);
                }
            }
        }

        // ── 7. IP addresses ───────────────────────────────────────────────
        for m in find_ip_addresses(text) {
            let start_col = byte_to_col[m];
            let end_col = byte_to_col[(m + find_ip_len(&text[m..])).min(byte_to_col.len() - 1)];
            for c in start_col..=end_col {
                map.entry((row_i32, c)).or_insert(colors.network);
            }
        }

        // ── 8. URLs ───────────────────────────────────────────────────────
        for m in find_urls(text) {
            let url_len = find_url_len(&text[m..]);
            let start_col = byte_to_col[m];
            let end_col = byte_to_col[(m + url_len).min(byte_to_col.len() - 1)];
            for c in start_col..=end_col {
                map.entry((row_i32, c)).or_insert(colors.url);
            }
        }

        // ── 9. Port numbers (:digits) ─────────────────────────────────────
        for m in find_ports(text) {
            let port_len = find_port_len(&text[m..]);
            let start_col = byte_to_col[m];
            let end_col = byte_to_col[(m + port_len).min(byte_to_col.len() - 1)];
            for c in start_col..=end_col {
                map.entry((row_i32, c)).or_insert(colors.port);
            }
        }
    }

    // Merge search highlight map — search colors override keyword colors.
    if let Some(sm) = search_map {
        for (key, color) in sm {
            map.insert(*key, *color);
        }
    }

    map
}

/// Find all occurrences of `keyword` as a whole word in `text`.
/// Returns byte offsets.
fn find_keyword(text: &str, keyword: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    let mut start = 0;
    while let Some(pos) = text[start..].find(keyword) {
        let abs = start + pos;
        let before_ok = abs == 0
            || text.as_bytes()[abs - 1] == b' '
            || is_boundary(text.as_bytes()[abs - 1] as char);
        let after_pos = abs + keyword.len();
        let after_ok = after_pos >= text.len()
            || text.as_bytes()[after_pos] == b' '
            || is_boundary(text.as_bytes()[after_pos] as char);
        if before_ok && after_ok {
            positions.push(abs);
        }
        start = abs + keyword.len();
    }
    positions
}

/// Check if `text` starts with a valid IP address. Returns byte length if so.
fn find_ip_len(text: &str) -> usize {
    let bytes = text.as_bytes();
    let mut dots = 0u8;
    let mut digits = 0u8;
    let mut len = 0usize;

    for &b in bytes {
        match b {
            b'0'..=b'9' => {
                digits += 1;
                if digits > 3 {
                    return 0;
                }
            }
            b'.' => {
                if digits == 0 {
                    return 0;
                }
                dots += 1;
                if dots > 3 {
                    return 0;
                }
                digits = 0;
            }
            _ => break,
        }
        len += 1;
    }

    if dots == 3 && digits > 0 {
        len
    } else {
        0
    }
}

/// Find byte offsets of IP addresses in text.
fn find_ip_addresses(text: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    let bytes = text.as_bytes();
    let len = bytes.len();

    for i in 0..len {
        if bytes[i].is_ascii_digit()
            && (i == 0 || is_boundary(bytes[i - 1] as char))
        {
            let remaining = &text[i..];
            let ip_len = find_ip_len(remaining);
            if ip_len > 0 {
                // Validate each octet is 0-255.
                let ip_str = &remaining[..ip_len];
                let valid = ip_str
                    .split('.')
                    .all(|octet| octet.parse::<u8>().is_ok());
                if valid {
                    positions.push(i);
                }
            }
        }
    }
    positions
}

/// Find byte offsets of URLs starting with http:// or https://
fn find_urls(text: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    let mut start = 0;
    while let Some(pos) = text[start..].find("http") {
        let abs = start + pos;
        let remaining = &text[abs..];
        if remaining.starts_with("https://") || remaining.starts_with("http://") {
            if abs == 0 || is_boundary(text.as_bytes()[abs - 1] as char) {
                positions.push(abs);
            }
        }
        start = abs + 4;
    }
    positions
}

/// Get byte length of a URL token (until whitespace or end of string).
fn find_url_len(text: &str) -> usize {
    text.find(|c: char| c.is_ascii_whitespace())
        .unwrap_or(text.len())
}

/// Find byte offsets of port patterns like `:22`, `:443`, `:8080`.
fn find_ports(text: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    let bytes = text.as_bytes();
    let len = bytes.len();

    for i in 0..len {
        if bytes[i] == b':'
            && i + 1 < len
            && bytes[i + 1].is_ascii_digit()
            && (i == 0 || is_boundary(bytes[i - 1] as char) || bytes[i - 1] == b' ')
        {
            let mut j = i + 1;
            while j < len && bytes[j].is_ascii_digit() {
                j += 1;
            }
            let port_str = &text[i + 1..j];
            if let Ok(port) = port_str.parse::<u16>() {
                if port > 0 {
                    let after_ok = j >= len || is_boundary(bytes[j] as char);
                    if after_ok {
                        positions.push(i);
                    }
                }
            }
        }
    }
    positions
}

/// Get byte length of a port token starting at ':'.
fn find_port_len(text: &str) -> usize {
    if !text.starts_with(':') {
        return 0;
    }
    let mut len = 1;
    for b in text.as_bytes()[1..].iter() {
        if b.is_ascii_digit() {
            len += 1;
        } else {
            break;
        }
    }
    len
}
