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

pub const num_items : usize = 5;

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


#[derive(Debug, Default, Clone, Copy)]
pub struct InventoryTab {
    pub(crate) row_index: usize,
    pub(crate) items: [Item ; num_items],
}

// impl InventoryTab {
//     /// Select the previous item in the items list (with wrap around)
//     pub fn prev(&mut self) {
//         self.row_index = self.row_index.saturating_add(ITEMS.len() - 1) % ITEMS.len();
//     }

//     /// Select the next item in the items list (with wrap around)
//     pub fn next(&mut self) {
//         self.row_index = self.row_index.saturating_add(1) % ITEMS.len();
//     }
// }

impl InventoryTab {
    /// Select the previous item in the items list (with wrap around)
    pub fn prev(&mut self) {
        self.row_index = self.row_index.saturating_add(self.items.len() - 1) % self.items.len();
    }

    /// Select the next item in the items list (with wrap around)
    pub fn next(&mut self) {
        self.row_index = self.row_index.saturating_add(1) % self.items.len();
    }

    pub fn add(&mut self, chunksize : u32) {
        let item: Item = self.items[self.row_index];
        let new_quantity :u32 = chunksize + item.quantity.parse::<u32>().ok().unwrap();
        let new_item: Item = Item {
            quantity: Box::leak(Box::new(new_quantity.to_string())),
            name: item.name,
        };
        self.items[self.row_index] = new_item;
    }

    pub fn remove(&mut self, chunksize : u32) {
        let item: Item = self.items[self.row_index];
        let mut new_quantity : u32 = item.quantity.parse::<u32>().ok().unwrap() - chunksize;
        new_quantity = if new_quantity > 0 { new_quantity } else { 0 };
        let new_item: Item = Item {
            quantity: Box::leak(Box::new(new_quantity.to_string())),
            name: item.name,
        };
        self.items[self.row_index] = new_item;
    }
}


pub fn parse_f32(field_name: &str, value: &str, item_name: &str) -> Result<f32, String> {
    value.parse::<f32>().map_err(|e| {
        format!(
            "Failed to parse '{}' for item '{}': value '{}': {}",
            field_name, item_name, value, e
        )
    })
}

pub fn render_items(items: [Item; num_items], selected_row: usize, area: Rect, buf: &mut Buffer) {
    let mut state = TableState::default().with_selected(Some(selected_row));
    let items = items.iter().copied();
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

pub fn render_scrollbar(items: [Item; num_items], position: usize, area: Rect, buf: &mut Buffer) {
    let mut state = ScrollbarState::default()
        .content_length(items.len())
        .viewport_content_length(6)
        .position(position);
    Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(None)
        .end_symbol(None)
        .track_symbol(None)
        .thumb_symbol("▐")
        .render(area, buf, &mut state);
}
