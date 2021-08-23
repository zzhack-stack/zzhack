use yew::worker::Agent;
use yew::worker::AgentLink;
use yew::worker::Context;
use yew::worker::HandlerId;

pub struct UserAgent {
    pub link: AgentLink<Self>,
    handler_ids: Vec<HandlerId>,
}

pub enum UserAgentMessage {
    Notify,
}

pub enum UserAgentInput {
    ChangeUser,
}

pub enum UserAgentOutput {
    None,
}

impl Agent for UserAgent {
    type Reach = Context<Self>;
    type Message = UserAgentMessage;
    type Input = UserAgentInput;
    type Output = UserAgentOutput;

    fn connected(&mut self, id: HandlerId) {
        self.handler_ids.push(id);
    }

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            handler_ids: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            UserAgentMessage::Notify => {
                let handler_ids = self.handler_ids.clone();

                for id in handler_ids {
                    self.link.respond(id, UserAgentOutput::None)
                }
            }
        }
    }
    fn handle_input(&mut self, input: Self::Input, _: HandlerId) {
        match input {
            UserAgentInput::ChangeUser => self.link.send_message(UserAgentMessage::Notify),
        };
    }
}
