use crate::ThemeService;
use yew::worker::Agent;
use yew::worker::AgentLink;
use yew::worker::Context;

pub struct ThemeAgent {
    pub link: AgentLink<Self>,
    pub theme_service: ThemeService,
}

pub enum ThemeAgentMessage {}

impl Agent for ThemeAgent {
    type Reach = Context<Self>;
    type Message = ThemeAgentMessage;
    type Input = &'static str;
    type Output = &'static str;
    fn create(_: yew::agent::AgentLink<Self>) -> Self {
        todo!()
    }
    fn update(&mut self, _: <Self as yew::agent::Agent>::Message) {
        todo!()
    }
    fn handle_input(&mut self, _: <Self as yew::agent::Agent>::Input, _: yew::agent::HandlerId) {
        todo!()
    }
}
