use std::mem;

use liom_node::NodeKind;

use crate::{ParseError, TreeSink};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Event {
    StartNode {
        kind: NodeKind,
        forward_parent: Option<usize>,
    },
    FinishNode,
    Token,
    Error(ParseError),
    Placeholder,
}

pub fn process(mut events: Vec<Event>, sink: &mut impl TreeSink) {
    let mut kinds = Vec::new();

    for i in 0..events.len() {
        match mem::replace(&mut events[i], Event::Placeholder) {
            Event::StartNode {
                kind,
                forward_parent,
            } => {
                kinds.push(kind);

                let mut i = i;
                let mut forward_parent = forward_parent;

                while let Some(fp) = forward_parent {
                    i += fp;

                    forward_parent = match mem::replace(&mut events[i], Event::Placeholder) {
                        Event::StartNode {
                            kind,
                            forward_parent,
                        } => {
                            kinds.push(kind);
                            forward_parent
                        }
                        _ => {
                            unreachable!();
                        }
                    };
                }

                for kind in kinds.drain(..).rev() {
                    sink.start_node(kind);
                }
            }
            Event::FinishNode => sink.finish_node(),
            Event::Token => sink.token(),
            Event::Error(error) => sink.error(error),
            Event::Placeholder => {}
        }
    }
}
