use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use std::io::{stdout, Write};

// Simple task structure
#[derive(Debug, Clone)]
struct Task {
    id: usize,
    title: String,
}

// Task validation function
fn validate_task_title(title: &str) -> Result<String, String> {
    let trimmed = title.trim();
    
    if title.is_empty() {
        return Err("Task title cannot be empty".to_string());
    }
    
    if trimmed.is_empty() {
        return Err("Task title cannot contain only whitespace".to_string());
    }
    
    Ok(trimmed.to_string())
}

// Simple task manager
struct TaskManager {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TaskManager {
    fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }
    
    fn add_task(&mut self, title: String) -> Result<usize, String> {
        let validated_title = validate_task_title(&title)?;
        let task = Task {
            id: self.next_id,
            title: validated_title,
        };
        let id = task.id;
        self.tasks.push(task);
        self.next_id += 1;
        Ok(id)
    }
    
    fn list_tasks(&self) -> &[Task] {
        &self.tasks
    }
}

fn main() -> std::io::Result<()> {
    // Clear the terminal screen
    execute!(stdout(), Clear(ClearType::All))?;
    
    print_banner()?;
    print_tool_info()?;
    
    // Interactive task management
    run_task_manager()?;
    
    Ok(())
}

fn run_task_manager() -> std::io::Result<()> {
    let mut task_manager = TaskManager::new();
    
    execute!(
        stdout(),
        SetForegroundColor(Color::Cyan),
        Print("\n🎮 Task Manager Commands:\n"),
        Print("  [a] Add task\n"),
        Print("  [l] List tasks\n"),
        Print("  [q] Quit\n"),
        ResetColor
    )?;
    
    // Enable raw mode for key input
    terminal::enable_raw_mode()?;
    
    loop {
        execute!(
            stdout(),
            SetForegroundColor(Color::White),
            Print("Enter command (a/l/q): "),
            ResetColor
        )?;
        stdout().flush()?;
        
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char('a') => {
                        execute!(stdout(), Print("a\n"))?;
                        add_task_interactive(&mut task_manager)?;
                    }
                    KeyCode::Char('l') => {
                        execute!(stdout(), Print("l\n"))?;
                        list_tasks_interactive(&task_manager)?;
                    }
                    KeyCode::Char('q') => {
                        execute!(stdout(), Print("q\n"))?;
                        execute!(
                            stdout(),
                            SetForegroundColor(Color::Green),
                            Print("👋 Goodbye! Thanks for using TaskFlow!\n"),
                            ResetColor
                        )?;
                        break;
                    }
                    _ => {
                        execute!(
                            stdout(),
                            SetForegroundColor(Color::Red),
                            Print("\n❌ Invalid command. Use a/l/q\n\n"),
                            ResetColor
                        )?;
                    }
                }
            }
            _ => {}
        }
    }
    
    // Disable raw mode
    terminal::disable_raw_mode()?;
    
    Ok(())
}

fn add_task_interactive(task_manager: &mut TaskManager) -> std::io::Result<()> {
    execute!(
        stdout(),
        SetForegroundColor(Color::Yellow),
        Print("📝 Enter task title: "),
        ResetColor
    )?;
    stdout().flush()?;
    
    let mut input = String::new();
    
    loop {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Enter => {
                        // Try to add the task
                        match task_manager.add_task(input.clone()) {
                            Ok(id) => {
                                execute!(
                                    stdout(),
                                    Print("\n"),
                                    SetForegroundColor(Color::Green),
                                    Print(&format!("✅ Task #{} added successfully!\n", id)),
                                    ResetColor
                                )?;
                                return Ok(());
                            }
                            Err(error) => {
                                execute!(
                                    stdout(),
                                    Print("\n"),
                                    SetForegroundColor(Color::Red),
                                    Print(&format!("❌ Error: {}\n", error)),
                                    ResetColor,
                                    SetForegroundColor(Color::Yellow),
                                    Print("📝 Enter task title: "),
                                    ResetColor
                                )?;
                                input.clear();
                            }
                        }
                    }
                    KeyCode::Esc => {
                        execute!(
                            stdout(),
                            Print("\n"),
                            SetForegroundColor(Color::DarkGrey),
                            Print("❌ Task creation cancelled.\n"),
                            ResetColor
                        )?;
                        return Ok(());
                    }
                    KeyCode::Backspace => {
                        if !input.is_empty() {
                            input.pop();
                            execute!(
                                stdout(),
                                cursor::MoveLeft(1),
                                Clear(ClearType::UntilNewLine)
                            )?;
                        }
                    }
                    KeyCode::Char(c) => {
                        input.push(c);
                        execute!(stdout(), Print(c.to_string()))?;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

fn list_tasks_interactive(task_manager: &TaskManager) -> std::io::Result<()> {
    execute!(
        stdout(),
        SetForegroundColor(Color::Cyan),
        Print("📋 Task List:\n"),
        Print("────────────\n"),
        ResetColor
    )?;
    
    let tasks = task_manager.list_tasks();
    
    if tasks.is_empty() {
        execute!(
            stdout(),
            SetForegroundColor(Color::DarkGrey),
            Print("No tasks found. Press 'a' to add your first task.\n"),
            ResetColor
        )?;
    } else {
        for task in tasks {
            execute!(
                stdout(),
                SetForegroundColor(Color::Green),
                Print(&format!("#{}: ", task.id)),
                ResetColor,
                Print(&format!("{}\n", task.title))
            )?;
        }
    }
    
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
        Print("═══════════════════════════════════\n"),
        ResetColor
    )?;

    execute!(
        stdout(),
        SetForegroundColor(Color::Yellow),
        Print("🛠️  Tool Information:\n"),
        ResetColor,
        Print("   Name: TaskFlow\n"),
        Print("   Version: 0.1.0\n"),
        Print("   Description: A simple task manager with validation\n"),
        Print("   Language: "),
        SetForegroundColor(Color::Magenta),
        Print("Rust 🦀\n"),
        ResetColor,
        Print("   Author: yashksaini-coder\n")
    )?;

    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        Print("✅ Features: "),
        ResetColor,
        Print("Add tasks, List tasks, Input validation\n"),
        SetForegroundColor(Color::Blue),
        Print("🔧 Validation: "),
        ResetColor,
        Print("Prevents empty titles and whitespace-only titles\n"),
        SetForegroundColor(Color::Blue),
        Print("🏠 Repository: "),
        SetForegroundColor(Color::Cyan),
        Print("https://github.com/yashksaini-coder/taskflow\n"),
        ResetColor
    )?;

    
    Ok(())
}

// Tests for validation logic
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_empty_title() {
        let result = validate_task_title("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task title cannot be empty");
    }

    #[test]
    fn test_validate_whitespace_only_title() {
        let result = validate_task_title("   \t\n   ");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task title cannot contain only whitespace");
    }

    #[test]
    fn test_validate_valid_title() {
        let result = validate_task_title("  Valid task title  ");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Valid task title");
    }

    #[test]
    fn test_task_manager_add_valid_task() {
        let mut manager = TaskManager::new();
        let result = manager.add_task("Valid task".to_string());
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
        assert_eq!(manager.list_tasks().len(), 1);
        assert_eq!(manager.list_tasks()[0].title, "Valid task");
    }

    #[test]
    fn test_task_manager_add_empty_task() {
        let mut manager = TaskManager::new();
        let result = manager.add_task("".to_string());
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task title cannot be empty");
        assert_eq!(manager.list_tasks().len(), 0);
    }

    #[test]
    fn test_task_manager_add_whitespace_task() {
        let mut manager = TaskManager::new();
        let result = manager.add_task("   \t   ".to_string());
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task title cannot contain only whitespace");
        assert_eq!(manager.list_tasks().len(), 0);
    }

    #[test]
    fn test_task_manager_multiple_tasks() {
        let mut manager = TaskManager::new();
        
        manager.add_task("First task".to_string()).unwrap();
        manager.add_task("Second task".to_string()).unwrap();
        
        let tasks = manager.list_tasks();
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].id, 1);
        assert_eq!(tasks[1].id, 2);
        assert_eq!(tasks[0].title, "First task");
        assert_eq!(tasks[1].title, "Second task");
    }
}