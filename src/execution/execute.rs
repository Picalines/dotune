use crate::model::Command;

pub fn execute_commands(commands: impl IntoIterator<Item = Command>) {
    for command in commands {
        dbg!(command);
    }
}
