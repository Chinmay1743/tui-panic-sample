use color_eyre::{eyre::bail, Result};

use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    text::Line,
    widgets::{Block, Paragraph},
    DefaultTerminal,
    Frame,
};

fn main() -> Result<()> {
    println!("Hello, world!");
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

struct App {
    hook_enabled: bool,
}

impl App {
    const fn new() -> Self {
        Self {
            hook_enabled: true
        }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('p') => panic!("Testing Panic"),
                    KeyCode::Char('e') => bail!("Testing Error"),
                    KeyCode::Char('h') => {
                        let _ = std::panic::take_hook();
                        self.hook_enabled = false;
                    },
                    _ => {}
                }
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let text = vec![
            if self.hook_enabled { Line::from("Hook is currently ENABLED") } else { Line::from("Hook is currently DISABLED") },
            Line::from(""),
            Line::from("Press p to cause panic"),
            Line::from("Press e to cause error"),
            Line::from("Press h to disable the panic hook"),
            Line::from("Press q to quit"),
            Line::from(""),
            Line::from("When your app panics without a panic hook, you will likely have to"),
            Line::from("reset your terminal afterwared with the reset command"),
            Line::from(""),
            Line::from("Try first with the panic handler enabled, and then with it disabled"),
            Line::from("to see the difference"),
        ];
    
        let paragraph = Paragraph::new(text)
            .block(Block::bordered().title("Panic Handler demo"))
            .centered();
    
        frame.render_widget(paragraph, frame.area());
    }
}




