use itertools::Itertools;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{
        Block, Clear, Padding, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState,
        StatefulWidget, Table, TableState, Widget, Wrap,
    },
};

use crate::models::item::Item;

use crate::{RgbSwatch, THEME};

// https://www.realsimple.com/food-inventorys/browse-all-inventorys/ratatouille
const INVENTORY: &[(&str, &str)] = &[
    (
        "Step 1: ",
        "Over medium-low heat, add the oil to a large skillet with the onion, garlic, and bay \
        leaf, stirring occasionally, until the onion has softened.",
    ),
    (
        "Step 2: ",
        "Add the eggplant and cook, stirring occasionally, for 8 minutes or until the eggplant \
        has softened. Stir in the zucchini, red bell pepper, tomatoes, and salt, and cook over \
        medium heat, stirring occasionally, for 5 to 7 minutes or until the vegetables are \
        tender. Stir in the basil and few grinds of pepper to taste.",
    ),
];

const ITEMS: &[Item] = &[
    Item {
        quantity: "4 tbsp",
        name: "olive oil",
    },
    Item {
        quantity: "1",
        name: "onion thinly sliced",
    },
    Item {
        quantity: "4",
        name: "cloves garlic\npeeled and sliced",
    },
    Item {
        quantity: "1",
        name: "small bay leaf",
    },
    Item {
        quantity: "1",
        name: "small eggplant cut\ninto 1/2 inch cubes",
    },
    Item {
        quantity: "1",
        name: "small zucchini halved\nlengthwise and cut\ninto thin slices",
    },
    Item {
        quantity: "1",
        name: "red bell pepper cut\ninto slivers",
    },
    Item {
        quantity: "4",
        name: "plum tomatoes\ncoarsely chopped",
    },
    Item {
        quantity: "1 tsp",
        name: "kosher salt",
    },
    Item {
        quantity: "1/4 cup",
        name: "shredded fresh basil\nleaves",
    },
    Item {
        quantity: "",
        name: "freshly ground black\npepper",
    },
];

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct InventoryTab {
    row_index: usize,
}

impl InventoryTab {
    /// Select the previous item in the items list (with wrap around)
    pub fn prev(&mut self) {
        self.row_index = self.row_index.saturating_add(ITEMS.len() - 1) % ITEMS.len();
    }

    /// Select the next item in the items list (with wrap around)
    pub fn next(&mut self) {
        self.row_index = self.row_index.saturating_add(1) % ITEMS.len();
    }
}

impl Widget for InventoryTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        RgbSwatch.render(area, buf);
        let area = area.inner(Margin {
            vertical: 1,
            horizontal: 2,
        });
        Clear.render(area, buf);
        Block::new()
            .title("Ratatouille Inventory".bold().white())
            .title_alignment(Alignment::Center)
            .style(THEME.content)
            .padding(Padding::new(1, 1, 2, 1))
            .render(area, buf);

        let scrollbar_area = Rect {
            y: area.y + 2,
            height: area.height - 3,
            ..area
        };
        render_scrollbar(self.row_index, scrollbar_area, buf);

        let area = area.inner(Margin {
            horizontal: 2,
            vertical: 1,
        });
        // let [inventory, items] =
        let [items] =
            Layout::horizontal([Constraint::Min(30)]).areas(area);

        // render_inventory(inventory, buf);
        render_items(self.row_index, items, buf);
    }
}

fn render_inventory(area: Rect, buf: &mut Buffer) {
    let lines = INVENTORY
        .iter()
        .map(|(step, text)| Line::from(vec![step.white().bold(), text.gray()]))
        .collect_vec();
    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .block(Block::new().padding(Padding::new(0, 1, 0, 0)))
        .render(area, buf);
}

fn render_items(selected_row: usize, area: Rect, buf: &mut Buffer) {
    let mut state = TableState::default().with_selected(Some(selected_row));
    let rows = ITEMS.iter().copied();
    let theme = THEME.inventory;
    StatefulWidget::render(
        Table::new(rows, [Constraint::Length(7), Constraint::Length(30)])
            .block(Block::new().style(theme.items))
            .header(Row::new(vec!["Qty", "Item"]).style(theme.items_header))
            .row_highlight_style(Style::new().light_yellow()),
        area,
        buf,
        &mut state,
    );
}

fn render_scrollbar(position: usize, area: Rect, buf: &mut Buffer) {
    let mut state = ScrollbarState::default()
        .content_length(ITEMS.len())
        .viewport_content_length(6)
        .position(position);
    Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(None)
        .end_symbol(None)
        .track_symbol(None)
        .thumb_symbol("▐")
        .render(area, buf, &mut state);
}
