use std::{error::Error, io};

use crossterm::{
    cursor::{Hide, Show},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    // Setup audio
    let mut audio = Audio::new();

    for filename in &["explode", "lose", "move", "pew", "startup", "win"] {
        audio.add(filename, format!("audio/{filename}.wav"));
    }

    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Cleanup
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
