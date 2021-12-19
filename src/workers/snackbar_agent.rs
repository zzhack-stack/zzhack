use yew::worker::Agent;
use yew::worker::AgentLink;
use yew::worker::Context;
use yew::worker::HandlerId;

pub struct SnackbarAgent {
    pub link: AgentLink<Self>,
    handler_ids: Vec<HandlerId>,
}

pub enum SnackbarAgentMessage {
    Notify(String),
}

pub enum SnackbarAgentInput {
    Dispatch(String),
}

impl Agent for SnackbarAgent {
    type Reach = Context<Self>;
    type Message = SnackbarAgentMessage;
    type Input = SnackbarAgentInput;
    type Output = String;

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
            SnackbarAgentMessage::Notify(text) => {
                for id in &self.handler_ids {
                    self.link.respond(*id, text.clone());
                }
            }
        }
    }

    fn handle_input(&mut self, input: Self::Input, _: HandlerId) {
        match input {
            SnackbarAgentInput::Dispatch(text) => {
                self.link.send_message(SnackbarAgentMessage::Notify(text))
            }
        };
    }
}
