use gloo::{
    storage::{LocalStorage, Storage},
    timers::callback::Interval,
};
use std::{cell::RefCell, rc::Rc, vec};

use yew::prelude::*;

use crate::{
    interpreter::types::{BF5DContext, Timeline},
    parser::bf5d,
};

pub const RAW_PROGRAM_KEY: &str = "raw";

#[derive(Debug, Clone)]
pub struct Model {
    // interior mutability pattern
    // https://github.com/rust-lang/book/blob/main/src/ch15-05-interior-mutability.md
    pub timelines: Rc<RefCell<Vec<Timeline>>>,
    pub context: Rc<RefCell<BF5DContext>>,
    pub error: Option<String>,
    pub interval: Rc<RefCell<Option<Interval>>>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            // raw_program: "".to_string(),
            context: Rc::new(RefCell::new(BF5DContext {
                raw_program: LocalStorage::get(RAW_PROGRAM_KEY)
                    .unwrap_or("(>^)@-[>,.<]".to_string()),
                tokens: vec![],
                program_input: "hello".to_string(),
                program_output: "".to_string(),
                total_timelines: 0,
                metadata: vec![],
                need_history: true,
            })),
            error: None,
            timelines: Rc::new(RefCell::new(vec![Timeline::new()])),
            interval: Rc::new(RefCell::new(None)),
        }
    }
}

pub enum Msg {
    RawProgram(String),
    ProgramInput(String),
    ParseUserInput,
    StepProgram,
    RunProgram(UseReducerHandle<Model>),
    PauseProgram,
    ResetProgram,
}

impl Reducible for Model {
    type Action = Msg;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        use Msg::*;
        match action {
            RawProgram(raw_program) => {
                let context = self.context.clone();
                let mut context = context.borrow_mut();
                context.raw_program = raw_program;
                Self { ..(*self).clone() }.into()
            }
            ProgramInput(program_input) => {
                let context = self.context.clone();
                let mut context = context.borrow_mut();
                context.program_input = program_input;
                Self { ..(*self).clone() }.into()
            }
            ParseUserInput => {
                let parsed = bf5d::parse(self.context.clone().borrow_mut().raw_program.as_str());

                match parsed {
                    Ok(tokens) => {
                        let context = self.context.clone();
                        let mut context = context.borrow_mut();
                        context.need_history =
                            tokens.contains(&crate::parser::types::Token::Rewind);
                        context.tokens = tokens;
                        Self { ..(*self).clone() }.into()
                    }
                    Err(e) => Self {
                        error: Some(format!("{:?}", e)),
                        ..(*self).clone()
                    }
                    .into(),
                }
            }
            StepProgram => {
                let context = self.context.clone();
                let mut context = context.borrow_mut();
                let timelines = self.timelines.clone();

                context.collect_timeline_metadata(&(*timelines.clone()).borrow());

                let commands = timelines
                    .borrow_mut()
                    .iter_mut()
                    .map(|t| t.update(&mut context))
                    .map(|(_, cmd)| cmd)
                    .collect::<Vec<_>>();

                for cmd in commands {
                    context.execute_command(cmd, &mut (timelines.clone().borrow_mut()));
                }

                Self { ..(*self).clone() }.into()
            }
            ResetProgram => {
                let context = self.context.clone();
                let mut context = context.borrow_mut();
                context.program_output = "".to_string();
                let interval = self.interval.clone();
                let mut interval = interval.borrow_mut();
                *interval = None;
                Self {
                    context: self.context.clone(),
                    error: self.error.clone(),
                    timelines: Rc::new(RefCell::new(vec![(Timeline::new())])),
                    interval: Rc::new(RefCell::new(None)),
                }
            }
            .into(),
            RunProgram(dispatcher) => {
                let interval = self.interval.clone();
                let mut interval = interval.borrow_mut();
                *interval = Some(Interval::new(100, move || dispatcher.dispatch(StepProgram)));
                Self { ..(*self).clone() }.into()
            }
            PauseProgram => {
                let interval = self.interval.clone();
                let mut interval = interval.borrow_mut();
                *interval = None;
                Self { ..(*self).clone() }.into()
            }
            _ => todo!(),
        }
    }
}
