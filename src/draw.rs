use tui::backend::{CrosstermBackend, Backend};
use tui::widgets::{Widget, Block, Borders};
use tui::Frame;
use crate::model;

pub fn draw<B: Backend>(f: &mut Frame<B>, model: &model::Model) {
    let size = f.size();
    let block = Block::default()
        .title("infosechonors")
        .borders(Borders::ALL);
    f.render_widget(block, size);
}