use tui::backend::{Backend};
use tui::widgets::{Block, Borders, Paragraph, Wrap, Row, Table};
use tui::layout::{Layout, Constraint, Direction, Alignment};
use tui::style::{Style, Color};
use tui::text::{Text};
use tui::Frame;
use std::string::String;
use crate::model;

pub fn draw<B: Backend>(f: &mut Frame<B>, model: &mut model::Model) {
    //Useful constants for styling sections
    let selected_area_style = Style::default().fg(Color::Cyan);

    //Split the terminal into sections
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

    // Create the Table showing packets as they are added
	let row_style = Style::default().fg(Color::White);
	let headers = vec!["Time", "Length"].into_iter();
    if let Some(packets_to_draw) = model.get_packets_to_draw(){
    	let rows = packets_to_draw.iter().map(|p| {
    		Row::StyledData(vec![p.header.ts.tv_sec, p.header.len as i64].into_iter(), row_style)
    	});
        let packet_view = Table::new(headers, rows)
            .block(Block::default().title("Packets").borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)))
    	    .header_style(Style::default().fg(Color::Yellow))
    	    .widths(&[Constraint::Length(10), Constraint::Length(10)])
    	    .style(Style::default().fg(Color::White))
            .highlight_symbol(">")
            .highlight_style(Style::default().fg(Color::Green))
    	    .column_spacing(1);
        f.render_stateful_widget(packet_view, topsection[0], &mut model.packet_table_state);
    }

    //Table showing conversations
    let blocks = Block::default()
        .title("infosechonors")
        .borders(Borders::ALL);
    f.render_widget(blocks,topsection[1]);

    //Textbox for entering commands
    let textbox = Paragraph::new(Text::from(String::from(&model.input)))
    	.block(Block::default().title("Paragraph").borders(Borders::ALL))
    	.alignment(Alignment::Left)
    	.wrap(Wrap { trim: true });
    f.render_widget(textbox, window[1]);
}