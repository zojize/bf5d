use std::{cell::Cell, num::Wrapping, vec};

use itertools::Itertools;

use crate::parser::types::{JumpType, MoveDirection, Token, UpdateType};

type ID = usize;

// https://stackoverflow.com/a/32936064/14835397
thread_local!(static ID_GEN: Cell<ID> = Cell::new(0));

#[derive(Debug, Clone, PartialEq)]
pub struct Timeline {
    pub id: ID,
    pub data: Vec<Wrapping<u8>>,
    pub data_backwards: Vec<Wrapping<u8>>,
    pub pointers: Vec<isize>,
    // pub history: Vec<MutationRecord>, // not sure about this
    pub instruction_pointer: usize,
}

pub enum Command {
    None,
    MovePointer { id: ID, direction: MoveDirection },
    SpawnAt { id: ID, instruction_start: usize },
    RemoveAt(ID),
}

impl Timeline {
    pub fn new() -> Self {
        ID_GEN.with(|thread_id| {
            let id = thread_id.get();
            thread_id.set(id + 1);
            Timeline {
                id,
                data: vec![Wrapping(0)],
                data_backwards: vec![],
                pointers: vec![0],
                instruction_pointer: 0,
            }
        })
    }

    pub fn clone_new_id(&self) -> Self {
        ID_GEN.with(|thread_id| {
            let id = thread_id.get();
            thread_id.set(id + 1);
            Self {
                id,
                data: self.data.clone(),
                data_backwards: self.data_backwards.clone(),
                pointers: self.pointers.clone(),
                instruction_pointer: self.instruction_pointer.clone(),
            }
        })
    }

    pub fn update(self: &mut Self, context: &mut BF5DContext) -> (&Self, Command) {
        use JumpType::*;
        use Token::*;
        use UpdateType::*;

        let action = context.tokens.get(self.instruction_pointer);

        if let Some(action) = action {
            // handle actions that don't dispatch commands
            match action {
                Move(dir) => match dir {
                    MoveDirection::Left => {
                        for ptr in &mut self.pointers {
                            *ptr -= 1;
                        }
                    }
                    MoveDirection::Right => {
                        for ptr in &mut self.pointers {
                            *ptr += 1;
                        }
                    }
                    _ => (),
                },
                Update(type_) => {
                    match type_ {
                        Increment => {
                            for ptr in self.pointers.clone() {
                                let data = self.data_at_mut(ptr);
                                *data += Wrapping(1);
                            }
                            // why is this an error ⬇️
                            // self.pointers.clone().iter().map(|index| self.get_data_at(*index));
                        }
                        Decrement => {
                            for ptr in self.pointers.clone() {
                                let data = self.data_at_mut(ptr);
                                *data -= Wrapping(1);
                            }
                        }
                    }
                }
                Write => {
                    context.program_output.push_str(
                        self.pointers
                            .iter()
                            .map(|ptr| self.data_at(*ptr).unwrap().0 as char)
                            .collect::<String>()
                            .as_str(),
                    );
                }
                Read => {
                    let c = context.program_input.remove(0);
                    for ptr in self.pointers.clone() {
                        let x = self.data_at_mut(ptr);
                        *x = Wrapping(c as u8);
                    }
                }
                Rewind => todo!(),
                _ => (),
            }

            // handle instruction pointer related actions
            match action {
                Jump { type_, index } => match type_ {
                    IfZero
                        if self
                            .pointers
                            .iter()
                            .cloned()
                            .map(|index| self.data_at(index))
                            .all(|x| *x.unwrap() == Wrapping(0)) =>
                    {
                        self.instruction_pointer = *index;
                    }
                    IfNotZero
                        if self
                            .pointers
                            .iter()
                            .cloned()
                            .map(|index| self.data_at(index))
                            .any(|x| *x.unwrap() != Wrapping(0)) =>
                    {
                        self.instruction_pointer = *index;
                    }
                    _ => {
                        self.instruction_pointer += 1;
                    }
                },
                Await => {
                    let (timeline_index, _) = context
                        .metadata
                        .iter()
                        .find_position(|meta| meta.id == self.id)
                        .unwrap();
                    if let Some(meta) = context.metadata.get(timeline_index + 1) {
                        // if timeline below has no pointers
                        if meta.pointers_count == 0 {
                            self.instruction_pointer += 1;
                        }
                    } else {
                        // or their is no timeline below this one
                        self.instruction_pointer += 1;
                    }
                }
                _ => {
                    self.instruction_pointer += 1;
                }
            }

            // handle command dispatching actions
            match action {
                Kill => (self, Command::RemoveAt(self.id)),
                Move(dir) => match dir {
                    MoveDirection::Up | MoveDirection::Down => (
                        self,
                        Command::MovePointer {
                            id: self.id,
                            direction: *dir,
                        },
                    ),
                    _ => (self, Command::None),
                },
                Spawn { index } => (
                    self,
                    Command::SpawnAt {
                        id: self.id,
                        instruction_start: *index,
                    },
                ),
                _ => (self, Command::None),
            }
        } else {
            (self, Command::RemoveAt(self.id))
        }
    }

    fn data_at_mut(&mut self, index: isize) -> &mut Wrapping<u8> {
        // if index negative
        let data = if index < 0 {
            // use backwards data
            &mut self.data_backwards
        } else {
            // use normal data
            &mut self.data
        };

        // if index negative
        let index = if index < 0 {
            // use backwards index
            backwards_index(index)
        } else {
            // use normal index
            index as usize
        };

        let len = data.len();

        // if index is out of bounds
        if index >= len {
            // extend data to fill up to index
            data.extend((len..index + 1).map(|_| Wrapping(0)));
        };

        (*data).get_mut(index).unwrap()
    }

    pub fn data_at(&self, index: isize) -> Option<&Wrapping<u8>> {
        let data = if index < 0 {
            &self.data_backwards
        } else {
            &self.data
        };

        let index = if index < 0 {
            backwards_index(index)
        } else {
            index as usize
        };

        (*data).get(index)
    }
}

#[derive(Debug, Clone)]
pub struct TimelineMeta {
    id: usize,
    pointers_count: usize,
}

fn backwards_index(index: isize) -> usize {
    if index < 0 {
        -(index + 1) as usize
    } else {
        index as usize
    }
}

#[derive(Debug, Clone)]
pub struct BF5DContext {
    pub raw_program: String,
    pub tokens: Vec<Token>,
    pub program_input: String,
    pub program_output: String,
    pub total_timelines: usize,
    pub metadata: Vec<TimelineMeta>,
}

impl BF5DContext {
    pub fn new() -> Self {
        BF5DContext {
            raw_program: "".to_string(),
            tokens: vec![],
            program_input: "hello".to_string(),
            program_output: "".to_string(),
            total_timelines: 0,
            metadata: vec![],
        }
    }

    pub fn collect_timeline_metadata(self: &mut Self, timelines: &Vec<Timeline>) {
        self.total_timelines = timelines.len();
        self.metadata = timelines
            .iter()
            .map(|t| TimelineMeta {
                id: t.id,
                pointers_count: t.pointers.len(),
            })
            .collect();
    }

    pub fn execute_command(self: &Self, command: Command, timelines: &mut Vec<Timeline>) {
        match command {
            Command::MovePointer { id, direction } => match direction {
                MoveDirection::Up => {
                    let (index, timeline) =
                        timelines.iter_mut().find_position(|t| t.id == id).unwrap();

                    if index != 0 {
                        let pointers = timeline.pointers.clone();
                        timeline.pointers.clear();
                        let target = timelines.get_mut(index - 1).unwrap();
                        target.pointers.extend(pointers.clone());
                    } else {
                        timeline.pointers.clear();
                    }
                }
                MoveDirection::Down => {
                    let (index, timeline) =
                        timelines.iter_mut().find_position(|t| t.id == id).unwrap();

                    if index != 0 {
                        let pointers = timeline.pointers.clone();
                        timeline.pointers.clear();
                        let target = timelines.get_mut(index + 1).unwrap();
                        target.pointers.extend(pointers.clone());
                    } else {
                        timeline.pointers.clear();
                    }
                }
                _ => panic!("undefined command direction"),
            },
            Command::SpawnAt {
                id,
                instruction_start,
            } => {
                let (index, timeline) = timelines.iter_mut().find_position(|t| t.id == id).unwrap();
                let new_timeline = timeline.clone_new_id();
                timeline.instruction_pointer = instruction_start;
                timelines.insert(index + 1, new_timeline);
            }
            Command::RemoveAt(id) => {
                let (index, _) = timelines.iter().find_position(|t| t.id == id).unwrap();
                if index != 0 {
                    timelines.remove(index);
                }
            }
            Command::None => (),
        }
    }
}
