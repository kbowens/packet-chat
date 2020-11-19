use tui::backend::{CrosstermBackend, Backend};
use tui::widgets::{Widget, Block, Borders, Paragraph, Wrap, Row, Table};
use tui::layout::{Layout, Constraint, Direction, Alignment};
use tui::style::{Style, Color};
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

	let row_style = Style::default().fg(Color::White);
	let time:String = *(model.get_packet_view());
	let headers = vec!["Time", "Length"].into_iter();
	let rows = model.packets.iter().map(|p| {
		Row::StyledData(vec![p.header.ts.tv_sec, p.header.len as i64].into_iter(), row_style)
	}).rev();
    let packet_view = Table::new(headers, rows)
        .block(Block::default().title("Table").borders(Borders::ALL))
	    .header_style(Style::default().fg(Color::Yellow))
	    .widths(&[Constraint::Length(10), Constraint::Length(10)])
	    .style(Style::default().fg(Color::White))
	    .column_spacing(1);
    f.render_widget(packet_view,topsection[0]);

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