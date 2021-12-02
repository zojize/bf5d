use gloo::storage::{LocalStorage, Storage};
use std::{cell::RefCell, rc::Rc, vec};

use yew::prelude::*;

use crate::{
    interpreter::types::{BF5DContext, Timeline},
    parser::bf5d,
};

pub const RAW_PROGRAM_KEY: &str = "raw";

#[derive(Debug)]
pub struct Model {
    // interior mutability pattern
    // https://github.com/rust-lang/book/blob/main/src/ch15-05-interior-mutability.md
    pub timelines: Rc<RefCell<Vec<Timeline>>>, // generational index?
    pub context: Rc<RefCell<BF5DContext>>,
    pub error: Option<String>,
    pub parsed: bool,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            // raw_program: "".to_string(),
            context: Rc::new(RefCell::new(BF5DContext {
                raw_program: LocalStorage::get(RAW_PROGRAM_KEY).unwrap_or("".to_string()),
                tokens: vec![],
                program_input: "hello".to_string(),
                program_output: "".to_string(),
                total_timelines: 0,
                metadata: vec![],
            })),
            error: None,
            timelines: Rc::new(RefCell::new(vec![Timeline::new()])),
            parsed: false,
        }
    }
}

pub enum Msg {
    RawProgram(String),
    ProgramInput(String),
    ParseUserInput,
    StepProgram,
    RunProgram,
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
                Self {
                    context: self.context.clone(),
                    error: self.error.clone(),
                    timelines: self.timelines.clone(),
                    parsed: false,
                }
                .into()
            }
            ProgramInput(program_input) => {
                let context = self.context.clone();
                let mut context = context.borrow_mut();
                context.program_input = program_input;
                Self {
                    context: self.context.clone(),
                    error: self.error.clone(),
                    timelines: self.timelines.clone(),
                    parsed: self.parsed,
                }
                .into()
            }
            ParseUserInput => {
                //
                let parsed = bf5d::parse(self.context.clone().borrow().raw_program.as_str());
                match parsed {
                    Ok(tokens) => {
                        let context = self.context.clone();
                        let mut context = context.borrow_mut();
                        context.tokens = tokens;
                        Self {
                            context: self.context.clone(),
                            error: self.error.clone(),
                            timelines: self.timelines.clone(),
                            parsed: self.parsed,
                        }
                        .into()
                    }
                    Err(e) => Self {
                        error: Some(format!("{:?}", e)),
                        context: self.context.clone(),
                        timelines: self.timelines.clone(),
                        parsed: true,
                    }
                    .into(),
                }
            }
            StepProgram => {
                let context = self.context.clone();
                let mut context = context.borrow_mut();
                let timelines = self.timelines.clone();

                context.collect_timeline_metadata(&timelines.clone().borrow());

                let commands = timelines
                    .borrow_mut()
                    .iter_mut()
                    .map(|t| t.update(&mut context))
                    .map(|(_, cmd)| cmd)
                    .collect::<Vec<_>>();

                for cmd in commands {
                    context.execute_command(cmd, &mut timelines.clone().borrow_mut());
                }

                Self {
                    context: self.context.clone(),
                    timelines,
                    error: self.error.clone(),
                    parsed: self.parsed,
                }
                .into()
            }
            ResetProgram => Self {
                context: self.context.clone(),
                error: self.error.clone(),
                timelines: Rc::new(RefCell::new(vec![(Timeline::new())])),
                parsed: false,
            }
            .into(),
            RunProgram => {
                self
            },
            PauseProgram => todo!(),
            _ => todo!(),
        }
    }
}
