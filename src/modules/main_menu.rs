use std::io::Stdout;

use ratatui::{layout::Margin, prelude::CrosstermBackend, style::{Color, Modifier, Style}, widgets::{self, block::Title, Block, ListState}, Terminal};

pub fn render(terminal : &mut Terminal<CrosstermBackend<Stdout>>, main_menu_items : [&str; 3], selected : &mut usize) -> std::io::Result<()> {
    terminal.draw(|frame| {
        let area = frame.size();
        let title = Title::from(" Press ESC to exit ").alignment(ratatui::layout::Alignment::Center);
        let b = widgets::Block::bordered().title(title);

        
        let inner_b = widgets::List::new(main_menu_items)
            .block(Block::bordered().title(" Selection Menu ").title_alignment(ratatui::layout::Alignment::Center))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .direction(widgets::ListDirection::BottomToTop);

        if *selected > main_menu_items.len() {*selected = main_menu_items.len();}

        frame.render_widget(b, area);
        frame.render_stateful_widget(inner_b, area.inner(Margin::new(area.width / 5, area.height / 5)), &mut ListState::default().with_selected(Some(*selected)));
    })?;

    Ok(())
}