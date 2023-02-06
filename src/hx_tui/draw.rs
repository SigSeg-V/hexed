use std::io::{Stdout, Error};

use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    layout::{Rect},
    Terminal,
};
use tui::terminal::{CompletedFrame};

fn draw_screen(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<CompletedFrame<'_>, Error> {

    terminal.draw(|f| {

    let left_size = Rect{
        width: f.size().width / 2, 
        height: f.size().height,
        x: f.size().x,
        y: f.size().y,
    };

    let right_size = Rect{
        width: f.size().width / 2, 
        height: f.size().height,
        x: f.size().x + f.size().width / 2,
        y: f.size().y,
    };

    let left_block = Block::default()
        .title("Left Block")
        .borders(Borders::ALL);

    let right_block = Block::default()
        .title("Right Block")
        .borders(Borders::ALL);
    f.render_widget(left_block, left_size);
    f.render_widget(right_block, right_size);
})
}
