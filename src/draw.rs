use std::sync::{Arc, Mutex};

use tui::backend::{Backend};
use tui::widgets::{Block, Borders, Paragraph, Wrap, Row, Table, Gauge, List, ListItem};
use tui::layout::{Layout, Constraint, Direction, Alignment};
use tui::style::{Style, Color, Modifier};
use tui::text::{Text};
use tui::Frame;
use std::string::String;
use crate::model;

pub fn draw<B: Backend>(f: &mut Frame<B>, model: Arc<Mutex<model::Model>>) {
    //Useful constants for styling sections
    let selected_area_style = Style::default().fg(Color::Cyan);
    let mut model = model.lock().unwrap();

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
    let right_side_of_topsection = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(topsection[1]);

/*Delete this, I think. Or make it messages?
	let progress_bar = Gauge::default()
		.block(Block::default().borders(Borders::ALL).title("Progress"))
		.gauge_style(Style::default().fg(Color::White).bg(Color::Black).add_modifier(Modifier::ITALIC))
		.percent(model.get_gauge_ratio());
	f.render_widget(progress_bar, window[0]);
*/
    // Create the Table showing packets as they are added
	let row_style = Style::default().fg(Color::White);
	let headers = vec!["Time", "Length"].into_iter();
    if let Some(packets_to_draw) = model.get_packets_to_draw() {
        let rows = packets_to_draw.iter().map(|p| {
            Row::StyledData(vec![p.header.ts.format("%H:%M:%S").to_string(), p.header.len.to_string()].into_iter(), row_style)
        });
        let packet_view = Table::new(headers, rows)
            .block(Block::default().title("Packets").borders(Borders::ALL).border_style(Style::default()))
    	    .header_style(Style::default().fg(Color::Yellow))
    	    .widths(&[Constraint::Length(10), Constraint::Length(10)])
    	    .style(Style::default().fg(Color::White))
            .highlight_symbol(">")
            .highlight_style(Style::default().fg(Color::Green))
    	    .column_spacing(1);
        f.render_stateful_widget(packet_view, topsection[0], &mut model.packet_table_state);

        if let Some(index) = model.packet_table_state.selected() {
            let focused_packet = &packets_to_draw[index];
            let items = [
                ListItem::new(format!("time: {}", focused_packet.header.ts)),
                ListItem::new(format!("len: {}", focused_packet.header.len)),
                ListItem::new(format!("caplen: {}", focused_packet.header.caplen)),
                ListItem::new(format!("mac_dst: {}", focused_packet.mac_dst.as_ref().unwrap_or(&"".to_string()))),
                ListItem::new(format!("mac_src: {}", focused_packet.mac_src.as_ref().unwrap_or(&"".to_string()))),
                ListItem::new(format!("ip_type: {}", focused_packet.ip_type.as_ref().unwrap_or(&"".to_string()))),
                ListItem::new(format!("ip_dst: {}", focused_packet.ip_dst.as_ref().unwrap_or(&"".to_string()))),
                ListItem::new(format!("ip_src: {}", focused_packet.ip_src.as_ref().unwrap_or(&"".to_string())))
            ];//For Data: String from utf8 lossy?

            let packet_detail_list = List::new(items)
                .block(Block::default().title("details").borders(Borders::ALL));
            f.render_widget(packet_detail_list, right_side_of_topsection[0]);

            let packet_payload_paragraph = Paragraph::new(Text::from(focused_packet.payload.as_ref()))
                .block(Block::default().title("Payload").borders(Borders::ALL))
                .alignment(Alignment::Left)
                .wrap(Wrap {trim: true });
            f.render_widget(packet_payload_paragraph, right_side_of_topsection[1]);
        }
        
    }
    //Textbox for entering commands
    let textbox = Paragraph::new(Text::from(String::from(&model.input)))
    	.block(Block::default().title("Search").borders(Borders::ALL))
    	.alignment(Alignment::Left)
    	.wrap(Wrap { trim: true });
    f.render_widget(textbox, window[1]);
}