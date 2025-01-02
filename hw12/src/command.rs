use std::collections::VecDeque;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Command<T> {
    fn execute(&self) -> Result<T>;
    fn undo(&self) -> Result<T>;
}

pub struct CommandA;
pub struct CommandB;
pub struct CommandC;

impl Command<()> for CommandA {
    fn execute(&self) -> Result<()> {
        println!("Command A executed");
        Ok(())
    }
    fn undo(&self) -> Result<()> {
        println!("Command A undone");
        Ok(())
    }
}

impl Command<()> for CommandB {
    fn execute(&self) -> Result<()> {
        println!("Command B executed");
        Ok(())
    }
    fn undo(&self) -> Result<()> {
        println!("Command B undone");
        Ok(())
    }
}

type ConcreteCommand = Box<dyn Command<()>>;

pub struct CommandFactory;

impl CommandFactory {
    pub fn a() -> Box<CommandA> {
        Box::new(CommandA)
    }
    pub fn b() -> Box<CommandB> {
        Box::new(CommandB)
    }
    pub fn c() -> Box<CommandC> {
        Box::new(CommandC)
    }
}

impl Command<()> for CommandC {
    fn execute(&self) -> Result<()> {
        println!("Command C executed");
        Ok(())
    }
    fn undo(&self) -> Result<()> {
        println!("Command C undone");
        Ok(())
    }
}

#[derive(Default)]
pub struct Client {
    command_history: VecDeque<ConcreteCommand>,
}

impl Client {
    pub fn new() -> Self {
        Client::default()
    }
    pub fn execute_command(&mut self, command: ConcreteCommand) {
        match command.execute() {
            Ok(_) => self.command_history.push_back(command),
            Err(e) => println!("Execution command fail: {e}"),
        }
    }

    pub fn undo_last_command(&mut self) {
        if self.command_history.is_empty() {
            return;
        }

        let command = self.command_history.pop_back().unwrap();
        if let Err(e) = command.undo() {
            println!("Undo command fail: {e}")
        }
    }
}
