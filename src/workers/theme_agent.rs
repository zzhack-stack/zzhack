use crate::console_log;
use crate::services::theme_service::Theme;
use crate::ThemeService;
use yew::worker::Agent;
use yew::worker::AgentLink;
use yew::worker::Context;
use yew::worker::HandlerId;

pub struct ThemeAgent {
    pub link: AgentLink<Self>,
    handler_ids: Vec<HandlerId>,
}

pub enum ThemeAgentInput {
    ChangeTheme(Theme),
}

pub enum ThemeAgentMessage {
    NotifyChangeTheme(Theme),
}

impl Agent for ThemeAgent {
    type Reach = Context<Self>;
    type Message = ThemeAgentMessage;
    type Input = ThemeAgentInput;
    type Output = Theme;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            handler_ids: vec![],
        }
    }
    fn update(&mut self, msg: Self::Message) {
        match msg {
            ThemeAgentMessage::NotifyChangeTheme(theme) => {
                let handler_ids = self.handler_ids.clone();

                for id in handler_ids {
                    self.link.respond(id, theme.clone())
                }
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.handler_ids.push(id);
    }

    fn handle_input(&mut self, input: ThemeAgentInput, _: HandlerId) {
        match input {
            ThemeAgentInput::ChangeTheme(theme) => {
                ThemeService::set_theme(theme);
                self.link
                    .send_message(ThemeAgentMessage::NotifyChangeTheme(theme))
            }
        };
    }
}
