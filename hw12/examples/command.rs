use hw12::command::{Client, CommandFactory};
fn main() {
    let mut client = Client::new();
    client.execute_command(CommandFactory::a());
    client.execute_command(CommandFactory::b());
    client.execute_command(CommandFactory::c());

    client.undo_last_command();
    client.undo_last_command();
    client.undo_last_command();
}
