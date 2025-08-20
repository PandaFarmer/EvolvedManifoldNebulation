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

use crate::models::{item::Item, item_detail::ItemDetail, item_aggregate::ItemAggregate};

use crate::{RgbSwatch, THEME};

use std::collections::HashMap;

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

const ITEM_DETAILS: &[ItemDetail] = &[
    ItemDetail {
        name: "galvanic_screw_faring",
        mass_per_unit: ".25",
        volume_per_unit: ".05",
        credits_per_unit: ".2",
    },
    ItemDetail {
        name: "anodized_metal_plate",
        mass_per_unit: "10",
        volume_per_unit: "5",
        credits_per_unit: "20",
    },
    ItemDetail {
        name: "insulated_wire_spool",
        mass_per_unit: "2.5",
        volume_per_unit: "3",
        credits_per_unit: "15",
    },
    ItemDetail {
        name: "synthetic_foliated_kalkite",
        mass_per_unit: "3.6",
        volume_per_unit: "6",
        credits_per_unit: "232",
    },
    ItemDetail {
        name: "rotor",
        mass_per_unit: "20",
        volume_per_unit: "8.2",
        credits_per_unit: "112",
    },
];

const ITEMS: &[Item] = &[
    Item {
        quantity: "1000",
        name: "galvanic_screw_faring",
    },
    Item {
        quantity: "22",
        name: "anodized_metal_plate",
    },
    Item {
        quantity: "5",
        name: "insulated_wire_spool",
    },
    Item {
        quantity: "10",
        name: "synthetic_foliated_kalkite",
    },
    Item {
        quantity: "5",
        name: "rotor",
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

        let [items] =
            Layout::horizontal([Constraint::Min(30)]).areas(area);

        render_items(self.row_index, items, buf);
    }
}

fn parse_f32(field_name: &str, value: &str, item_name: &str) -> Result<f32, String> {
    value.parse::<f32>().map_err(|e| {
        format!(
            "Failed to parse '{}' for item '{}': value '{}': {}",
            field_name, item_name, value, e
        )
    })
}

fn render_items(selected_row: usize, area: Rect, buf: &mut Buffer) {
    let mut state = TableState::default().with_selected(Some(selected_row));
    let items = ITEMS.iter().copied();
    let item_details = ITEM_DETAILS.iter().copied();

    // Build lookup map from name to Properties
    let prop_map: HashMap<_, _> = item_details
        .into_iter()
        .map(|p| (p.name, p))
        .collect();

    // Join and parse values
    let combined: Vec<ItemAggregate> = items
        .into_iter()
        .filter_map(|item| {
            prop_map.get(item.name).and_then(|prop| {
                // Try to parse all fields
                let quantity: f32 = item.quantity.parse::<f32>().ok()?;
                let mass: f32 = prop.mass_per_unit.parse().ok()?;
                let volume: f32 = prop.volume_per_unit.parse().ok()?;
                let credits: f32 = prop.credits_per_unit.parse().ok()?;
                

                Some(vec![ItemAggregate {
                    name: item.name,
                    quantity: item.quantity,
                    mass: Box::leak(Box::new((quantity * mass).to_string())),
                    volume: Box::leak(Box::new((quantity * volume).to_string())),
                    credits: Box::leak(Box::new((quantity * credits).to_string())),
                }])
            })
        })
        .flatten()
        .collect();
    
    let theme = THEME.inventory;
    StatefulWidget::render(
        Table::new(combined, [Constraint::Length(30), Constraint::Length(13), Constraint::Length(13), Constraint::Length(13), Constraint::Length(13)])
            .block(Block::new().style(theme.items))
            .header(Row::new(vec!["Item", "Units", "Mass(kg)", "Volume(Mcu))", "Credits(ZK)" ]).style(theme.items_header))
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
