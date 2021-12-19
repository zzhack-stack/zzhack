use crate::workers::snackbar_agent::{SnackbarAgent, SnackbarAgentInput};
use yew::prelude::*;
use yew::Callback;

pub struct SnackbarService {
    agent: Box<dyn Bridge<SnackbarAgent>>,
}

impl SnackbarService {
    pub fn new() -> SnackbarService {
        SnackbarService {
            agent: SnackbarAgent::bridge(Callback::noop()),
        }
    }

    pub fn send(&mut self, text: &str) {
        self.agent
            .send(SnackbarAgentInput::Dispatch(text.to_string()))
    }
}
