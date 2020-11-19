use tui::backend::{CrosstermBackend, Backend};
use tui::widgets::{Widget, Block, Borders, Paragraph, Wrap};
use tui::layout::{Layout, Constraint, Direction, Alignment};
use tui::text::{Text};
use tui::Frame;
use std::string::String;
use crate::model;

pub fn draw<B: Backend>(f: &mut Frame<B>, model: &model::Model) {
    let size = f.size();
    let window = Layout::default()
    	.direction(Direction::Vertical)
    	.margin(1)
    	.constraints(
    		[
    		Constraint::Percentage(85),
    		Constraint::Percentage(15),
    		].as_ref()
    	)
    	.split(f.size());

    let topsection = Layout::default()
    	.direction(Direction::Horizontal)
    	.margin(0)
    	.constraints(
    		[
    		Constraint::Percentage(50),
			Constraint::Percentage(50),
			].as_ref()
		)
		.split(window[0]);

    let block = Block::default()
        .title("infosechonors")
        .borders(Borders::ALL);
    f.render_widget(block,topsection[0]);

    let blocks = Block::default()
        .title("infosechonors")
        .borders(Borders::ALL);
    f.render_widget(blocks,topsection[1]);

    let textbox = Paragraph::new(Text::from(String::from(&model.input)))
    	.block(Block::default().title("Paragraph").borders(Borders::ALL))
    	.alignment(Alignment::Left)
    	.wrap(Wrap { trim: true });
    f.render_widget(textbox, window[1]);
}