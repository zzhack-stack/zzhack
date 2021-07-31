use crate::services::MarkdownService;
use crate::utils::theme::by_theme;
use yew::virtual_dom::VNode;
use yew::worker::Agent;
use yew::worker::AgentLink;
use yew::worker::HandlerId;
use yew::worker::Job;
use yew::Callback;
use yew::Html;

pub struct MarkdownWorker {
    link: AgentLink<Self>,
    who: Option<HandlerId>,
    callback: Option<Callback<VNode>>,
}

pub enum MarkdownInput {
    ParseContent(Callback<VNode>, String),
}

pub enum MarkdownOutput {
    ContentParsed,
}

pub enum MarkdownWorkerMessage {
    ParseMarkdownContent(String),
}

impl Agent for MarkdownWorker {
    type Reach = Job<Self>;
    type Message = MarkdownWorkerMessage;
    type Input = MarkdownInput;
    type Output = MarkdownOutput;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            who: None,
            callback: None,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            MarkdownWorkerMessage::ParseMarkdownContent(raw_markdown_content) => {
                let vnode = render_content(raw_markdown_content);
                match self.callback.clone() {
                    Some(callback) => callback.emit(vnode),
                    None => (),
                }
            }
        };
    }

    fn handle_input(&mut self, input: Self::Input, who: HandlerId) {
        self.who = Some(who);

        match input {
            MarkdownInput::ParseContent(callback, raw_markdown_content) => {
                self.callback = Some(callback);
                self.link.respond(who, MarkdownOutput::ContentParsed);
                self.link
                    .send_message(MarkdownWorkerMessage::ParseMarkdownContent(
                        raw_markdown_content,
                    ))
            }
        }
    }
}

fn render_content(content: String) -> VNode {
    let markdown_service = MarkdownService::new(content);
    let el = markdown_service.parse_to_element(by_theme("base16-ocean.light", "base16-ocean.dark"));

    Html::VRef(el.into())
}
