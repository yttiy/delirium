use engine::Message;

pub struct MessageManager{
    messages: Vec<Message>
}

impl MessageManager {
    pub fn new() -> MessageManager {
        MessageManager {
            messages: Vec::new()
        }
    }

    pub fn add(&mut self, m: Message){
        self.messages.push(m);
    }

    pub fn get(&self) -> &Vec<Message>{
        &self.messages
    }

    pub fn clear(&mut self){
        self.messages.clear();
    }
}
