// use itertools::Itertools;
// use ratatui::{
//     buffer::Buffer,
//     layout::{Alignment, Constraint, Layout, Margin, Rect},
//     style::{Style, Stylize},
//     text::Line,
//     widgets::{
//         Block, Clear, Padding, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState,
//         StatefulWidget, Table, TableState, Widget, Wrap,
//     },
// };

use chrono::{DateTime, NaiveDateTime, Utc};

use ratatui::widgets::Row;

#[derive(Debug, Default, Clone, Copy)]
pub struct Item {
    pub quantity: &'static str,
    pub name: &'static str,
    // pub timestamp: &'static str,
}

impl Item {
    #[allow(clippy::cast_possible_truncation)]
    fn height(&self) -> u16 {
        self.name.lines().count() as u16
    }
}

impl<'a> From<Item> for Row<'a> {
    fn from(i: Item) -> Self {
        Row::new(vec![i.quantity, i.name]).height(i.height())
    }
}