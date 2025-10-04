use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::stdout;

fn main() -> std::io::Result<()> {
    // Clear the terminal screen
    execute!(stdout(), Clear(ClearType::All))?;
    
    print_banner()?;
    print_tool_info()?;
    
    Ok(())
}

fn print_banner() -> std::io::Result<()> {
    execute!(
        stdout(),
        SetForegroundColor(Color::Cyan),
        Print(r#"
 ████████╗ █████╗ ███████╗██╗  ██╗███████╗██╗      ██████╗ ██╗    ██╗
 ╚══██╔══╝██╔══██╗██╔════╝██║ ██╔╝██╔════╝██║     ██╔═══██╗██║    ██║
    ██║   ███████║███████╗█████╔╝ █████╗  ██║     ██║   ██║██║ █╗ ██║
    ██║   ██╔══██║╚════██║██╔═██╗ ██╔══╝  ██║     ██║   ██║██║███╗██║
    ██║   ██║  ██║███████║██║  ██╗██║     ███████╗╚██████╔╝╚███╔███╔╝
    ╚═╝   ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝     ╚══════╝ ╚═════╝  ╚══╝╚══╝ 

"#),
        ResetColor
    )?;
    
    Ok(())
}

fn print_tool_info() -> std::io::Result<()> {
    execute!(
        stdout(),
        SetForegroundColor(Color::Green),
        Print("📋 TaskFlow - Terminal Task Manager\n"),
        SetForegroundColor(Color::White),
        Print("═══════════════════════════════════\n\n"),
        ResetColor
    )?;

    execute!(
        stdout(),
        SetForegroundColor(Color::Yellow),
        Print("🛠️  Tool Information:\n"),
        ResetColor,
        Print("   Name: TaskFlow\n"),
        Print("   Version: 0.1.0\n"),
        Print("   Description: A modern terminal-based task management tool\n"),
        Print("   Language: "),
        SetForegroundColor(Color::Magenta),
        Print("Rust 🦀\n"),
        ResetColor,
        Print("   Author: yashksaini-coder\n\n")
    )?;

    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        Print("📝 Status: "),
        SetForegroundColor(Color::Green),
        Print("Basic CLI setup complete\n"),
        ResetColor,
        SetForegroundColor(Color::Blue),
        Print("🏠 Repository: "),
        SetForegroundColor(Color::Cyan),
        Print("https://github.com/yashksaini-coder/taskflow\n\n"),
        ResetColor
    )?;

    execute!(
        stdout(),
        SetForegroundColor(Color::Green),
        Print("🚀 Ready for development!\n\n"),
        ResetColor
    )?;

    Ok(())
}
