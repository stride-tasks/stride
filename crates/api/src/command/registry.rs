use std::collections::HashMap;

use super::CommandHandler;

#[derive(Debug, Default)]
pub struct CommandRegistry {
    map: HashMap<Box<str>, Box<dyn CommandHandler>>,
}

impl CommandRegistry {
    pub fn insert(
        &mut self,
        command: Box<str>,
        handler: Box<dyn CommandHandler>,
    ) -> Option<Box<dyn CommandHandler>> {
        self.map.insert(command, handler)
    }

    pub fn get(&self, command: &str) -> Option<&dyn CommandHandler> {
        self.map.get(command).map(Box::as_ref)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &dyn CommandHandler)> {
        self.map
            .iter()
            .map(|(command, handler)| (command.as_ref(), handler.as_ref()))
    }
}
