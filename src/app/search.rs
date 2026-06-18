use std::collections::HashMap;

use gpui::{
    Context, Focusable as _, Hsla, IntoElement, InteractiveElement as _, ParentElement as _,
    Styled as _, Window, div, prelude::FluentBuilder as _, px, rems,
};
use gpui_component::{
    ActiveTheme as _, Disableable as _, IconName, Sizable as _,
    button::{Button, ButtonVariants as _},
    h_flex, input::Input,
};
use rust_i18n::t;

use crate::Ashell;

// ── Search highlight colors ──────────────────────────────────────────────
// Regular matches: semi-transparent red.
fn search_match_color() -> Hsla {
    Hsla {
        h: 0.0,
        s: 0.85,
        l: 0.50,
        a: 0.45,
    }
}

// Current match: fully opaque bright red.
fn search_current_color() -> Hsla {
    Hsla {
        h: 0.0,
        s: 0.90,
        l: 0.50,
        a: 0.70,
    }
}

impl Ashell {
    pub(crate) fn toggle_search(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.search_active {
            self.close_search(window, cx);
        } else {
            self.open_search(window, cx);
        }
    }

    pub(crate) fn open_search(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.search_active = true;
        self.refocus_search_input(window, cx);
        cx.notify();
    }

    pub(crate) fn close_search(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.search_active = false;
        self.search_query.clear();
        self.search_matches.clear();
        self.search_current = 0;
        self.focus_handle.focus(window, cx);
        cx.notify();
    }

    /// Move keyboard focus back to the search input so the user can keep typing.
    fn refocus_search_input(&self, window: &mut Window, cx: &mut Context<Self>) {
        self.search_input.update(cx, |state, cx| {
            state.focus_handle(cx).focus(window, cx);
        });
    }

    pub(crate) fn perform_search(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let query = self.search_input.read(cx).text().to_string();
        if query.is_empty() {
            self.search_query.clear();
            self.search_matches.clear();
            self.search_current = 0;
            self.refocus_search_input(window, cx);
            cx.notify();
            return;
        }

        let Some(tab) = self
            .active_tab
            .as_ref()
            .and_then(|id| self.tabs.iter().find(|t| &t.id == id))
        else {
            self.refocus_search_input(window, cx);
            return;
        };

        let snapshot = tab.render_snapshot();
        let query_lower = query.to_lowercase();
        let query_byte_len = query.len();

        // Build per-row cell lists.
        let mut row_cells: Vec<Vec<(i32, char)>> = vec![vec![]; snapshot.rows];
        for rc in &snapshot.cells {
            if rc.row >= 0 && (rc.row as usize) < snapshot.rows {
                row_cells[rc.row as usize].push((rc.col, rc.cell.c));
            }
        }
        for row in row_cells.iter_mut() {
            row.sort_by_key(|&(col, _)| col);
        }

        let mut matches: Vec<(i32, i32)> = Vec::new();

        for (row_idx, row) in row_cells.iter().enumerate() {
            if row.is_empty() {
                continue;
            }

            // Build text string and byte→column index mapping.
            let mut text = String::with_capacity(row.len());
            let mut byte_to_col: Vec<i32> = Vec::new();
            for &(col, c) in row {
                text.push(c);
                // Each byte of this character maps to `col`.
                while byte_to_col.len() < text.len() {
                    byte_to_col.push(col);
                }
            }
            let text_lower = text.to_lowercase();

            // Find all occurrences of the query in this row.
            let mut search_start = 0;
            while let Some(pos) = text_lower[search_start..].find(&query_lower) {
                let abs = search_start + pos;
                let start_col = byte_to_col[abs];
                let end_byte = (abs + query_byte_len).min(byte_to_col.len());
                let end_col = byte_to_col[end_byte - 1];
                for c in start_col..=end_col {
                    matches.push((row_idx as i32, c));
                }
                search_start = abs + query_byte_len;
            }
        }

        let match_count = count_match_groups(&matches);

        self.search_query = query;
        self.search_matches = matches;

        if match_count > 0 {
            self.search_current = 0;
            self.jump_to_current_match(cx);
        }

        self.status = format!(
            "{}: {} ({})",
            t!("search"),
            self.search_query,
            if match_count == 0 {
                t!("no_results").to_string()
            } else {
                format!("{}/{}", self.search_current + 1, match_count)
            }
        )
        .into();

        // Keep focus on the search input so the user can continue typing.
        self.refocus_search_input(window, cx);
        cx.notify();
    }

    pub(crate) fn search_goto_next(&mut self, cx: &mut Context<Self>) {
        let match_count = count_match_groups(&self.search_matches);
        if match_count == 0 {
            return;
        }
        self.search_current = (self.search_current + 1) % match_count;
        self.jump_to_current_match(cx);
        cx.notify();
    }

    pub(crate) fn search_goto_prev(&mut self, cx: &mut Context<Self>) {
        let match_count = count_match_groups(&self.search_matches);
        if match_count == 0 {
            return;
        }
        self.search_current = (self.search_current + match_count - 1) % match_count;
        self.jump_to_current_match(cx);
        cx.notify();
    }

    fn jump_to_current_match(&mut self, _cx: &mut Context<Self>) {
        let Some((target_row, _)) =
            find_nth_match_start(&self.search_matches, self.search_current)
        else {
            return;
        };

        if let Some(tab) = self
            .active_tab
            .as_ref()
            .and_then(|id| self.tabs.iter_mut().find(|t| &t.id == id))
        {
            let snapshot = tab.render_snapshot();
            let rows = snapshot.rows as i32;
            let history = snapshot.history_size as i32;
            let current_offset = snapshot.display_offset as i32;
            let visible_top = history - current_offset;
            let visible_bottom = visible_top + rows - 1;

            if target_row < visible_top || target_row > visible_bottom {
                let new_offset = (history + rows - 1 - target_row).max(0) as usize;
                if new_offset > snapshot.display_offset {
                    tab.scroll_up_by(new_offset - snapshot.display_offset);
                } else if new_offset < snapshot.display_offset {
                    tab.scroll_down_by(snapshot.display_offset - new_offset);
                }
            }
        }
    }

    /// Build a highlight map for search matches. The current match gets a
    /// distinct color.
    pub(crate) fn search_highlight_map(&self) -> Option<HashMap<(i32, i32), Hsla>> {
        if self.search_matches.is_empty() || self.search_query.is_empty() {
            return None;
        }

        let mut map = HashMap::new();
        let match_color = search_match_color();
        let current_color = search_current_color();

        let mut sorted: Vec<(i32, i32)> = self.search_matches.clone();
        sorted.sort();

        let mut group_idx = 0;
        let mut i = 0;
        while i < sorted.len() {
            let is_current = group_idx == self.search_current;
            let color = if is_current {
                current_color
            } else {
                match_color
            };

            // Color all consecutive cells in this group.
            let (r, _) = sorted[i];
            let mut j = i;
            while j < sorted.len() && sorted[j].0 == r {
                if j > i && sorted[j].1 != sorted[j - 1].1 + 1 {
                    break;
                }
                map.insert(sorted[j], color);
                j += 1;
            }

            group_idx += 1;
            i = j;
        }

        Some(map)
    }

    pub(crate) fn render_search_bar(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        if !self.search_active {
            // Semi-transparent search button in the top-right corner.
            return div()
                .absolute()
                .top(px(8.))
                .right(px(24.))
                .child(
                    Button::new("search-btn")
                        .ghost()
                        .icon(IconName::Search)
                        .opacity(0.35)
                        .hover(|s| s.opacity(1.0))
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.toggle_search(window, cx);
                        })),
                )
                .into_any_element();
        }

        // Expanded search bar.
        let match_count = count_match_groups(&self.search_matches);
        let has_query = !self.search_query.is_empty();
        let has_matches = match_count > 0;
        let current_display = if has_matches {
            format!("{}/{}", self.search_current + 1, match_count)
        } else if has_query {
            "0".to_string()
        } else {
            String::new()
        };

        div()
            .absolute()
            .top(px(8.))
            .right(px(24.))
            .on_key_down(cx.listener(|this, event: &gpui::KeyDownEvent, window, cx| {
                let key = event.keystroke.key.as_str();
                if key == "escape" {
                    this.close_search(window, cx);
                    window.prevent_default();
                    cx.stop_propagation();
                } else if key == "enter" {
                    if event.keystroke.modifiers.shift {
                        this.search_goto_prev(cx);
                    } else if this.search_query.is_empty()
                        || this.search_input.read(cx).text().to_string() != this.search_query
                    {
                        this.perform_search(window, cx);
                    } else {
                        this.search_goto_next(cx);
                    }
                    window.prevent_default();
                    cx.stop_propagation();
                }
            }))
            .child(
                h_flex()
                    .gap_1()
                    .items_center()
                    .p_1()
                    .rounded(px(6.))
                    .bg(cx.theme().popover)
                    .border_1()
                    .border_color(cx.theme().border)
                    .child(
                        div()
                            .w(px(200.))
                            .child(Input::new(&self.search_input).small()),
                    )
                    .when(!current_display.is_empty(), |this| {
                        this.child(
                            div()
                                .text_size(rems(0.75))
                                .text_color(cx.theme().muted_foreground)
                                .min_w(px(36.))
                                .text_center()
                                .child(current_display),
                        )
                    })
                    .child(
                        Button::new("search-prev")
                            .ghost()
                            .xsmall()
                            .icon(IconName::ChevronUp)
                            .disabled(!has_matches)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.search_goto_prev(cx);
                            })),
                    )
                    .child(
                        Button::new("search-next")
                            .ghost()
                            .xsmall()
                            .icon(IconName::ChevronDown)
                            .disabled(!has_matches)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.search_goto_next(cx);
                            })),
                    )
                    .child(
                        Button::new("search-close")
                            .ghost()
                            .xsmall()
                            .icon(IconName::Close)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.close_search(window, cx);
                            })),
                    ),
            )
            .into_any_element()
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────

/// Count distinct match groups in a sorted list of (row, col) positions.
/// A group is a run of consecutive columns in the same row.
fn count_match_groups(matches: &[(i32, i32)]) -> usize {
    if matches.is_empty() {
        return 0;
    }
    let mut sorted: Vec<(i32, i32)> = matches.to_vec();
    sorted.sort();
    let mut count = 0;
    let mut i = 0;
    while i < sorted.len() {
        count += 1;
        let (r, _) = sorted[i];
        i += 1;
        // Skip consecutive columns in the same row.
        while i < sorted.len() && sorted[i].0 == r && sorted[i].1 == sorted[i - 1].1 + 1 {
            i += 1;
        }
    }
    count
}

/// Find the (row, col) start of the Nth distinct match group.
fn find_nth_match_start(matches: &[(i32, i32)], n: usize) -> Option<(i32, i32)> {
    if matches.is_empty() {
        return None;
    }
    let mut sorted: Vec<(i32, i32)> = matches.to_vec();
    sorted.sort();
    let mut group_idx = 0;
    let mut i = 0;
    while i < sorted.len() {
        if group_idx == n {
            return Some(sorted[i]);
        }
        group_idx += 1;
        let (r, _) = sorted[i];
        i += 1;
        while i < sorted.len() && sorted[i].0 == r && sorted[i].1 == sorted[i - 1].1 + 1 {
            i += 1;
        }
    }
    None
}
