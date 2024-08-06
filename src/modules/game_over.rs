use std::io::Stdout;

use ratatui::{layout::Margin, prelude::CrosstermBackend, style::Stylize, widgets::{self, block::Title, Block, Paragraph}, Terminal};

pub fn render(terminal : &mut Terminal<CrosstermBackend<Stdout>>, points : u32) -> std::io::Result<()> {
    terminal.draw(|frame| {
        let area = frame.size();
        let title = Title::from(" You have lost. ").alignment(ratatui::layout::Alignment::Center);
        let b = widgets::Block::bordered().title(title);

        let paragraph = Paragraph::new(format!("You got {} points!", points)).alignment(ratatui::layout::Alignment::Center).block(Block::bordered()).yellow();

        frame.render_widget(b, area);
        frame.render_widget(paragraph, area.inner(Margin::new(area.width / 5, area.height / 3)));
    })?;

    Ok(())
}