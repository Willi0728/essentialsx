use core::ops::{Deref, DerefMut};
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;

pub struct ReplCore<S> {
    pub state: S,
    pub evaluator: Evaluator<S>,
}

pub struct Evaluator<S>(pub Box<dyn FnMut(&mut S, String) -> (Option<Evaluator<S>>, Option<String>)>);

#[macro_export]
macro_rules! deref_impl {
    ($($struct:ident ( <$($t:ident),* $(,)?> ) . $field:tt : $type:ty),* $(,)?) => {
        $(
            impl<$($t),*> Deref for $struct<$($t),*> {
                type Target = $type;

                #[inline(always)]
                fn deref(&self) -> &Self::Target {
                    &self.$field
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! deref_mut_impl {
    ($($struct:ident ( <$($t:ident),* $(,)?> ) . $field:tt : $type:ty),* $(,)?) => {
        $(
            impl<$($t),*> DerefMut for $struct<$($t),*> {
                #[inline(always)]
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.$field
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! deref_and_deref_mut_impl {
    ($($struct:ident ( <$($t:ident),* $(,)?> ) . $field:tt : $type:ty),* $(,)?) => {
        $(
            $crate::deref_impl!($struct(<$($t),*>).$field: $type);
            $crate::deref_mut_impl!($struct(<$($t),*>).$field: $type);
        )*
    };
}

deref_and_deref_mut_impl!(
    Evaluator(<S>).0: Box<dyn FnMut(&mut S, String) -> (Option<Evaluator<S>>, Option<String>)>,
    ReplCore(<S>).state: S
);

impl<S> ReplCore<S> {
    pub fn new(state: S, evaluator: Evaluator<S>) -> Self {
        ReplCore { state, evaluator }
    }
    pub fn with_evaluator(evaluator: Evaluator<S>) -> Self
    where
        S: Default,
    {
        ReplCore { state: Default::default(), evaluator }
    }

    pub fn handle_input(&mut self, input: String) -> Option<String> {
        let mut orig = core::mem::replace(&mut self.evaluator, Evaluator(Box::new(|_, _|(None, None))));
        let evaluated = (*orig)(&mut self.state, input);
        let result = evaluated.1;
        self.evaluator = evaluated.0.unwrap_or_else(|| orig);
        result
    }
}

mod repl_bytes {
    #![cfg(feature = "__bytes")]

    use std::io::{self, Read, Write};
    use std::sync::RwLock;
    use std::thread::JoinHandle;
    use crossterm::event;
    use crossterm::event::{Event, KeyModifiers};
    use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
    use crate::io::repl::ReplCore;

    /// A pre-configured wrapper around [`ReplCore`]. May have defaults that you don't like.
    pub struct Repl<R: Read, W: Write, S> {
        core: ReplCore<S>,
        reader: R,
        writer: W,
        pub history: Vec<String>,
        history_index: usize,
        current: String,
        cursor: usize,
        pub prompt: String,
    }

    impl<R: Read, W: Write, S> Repl<R, W, S> {
        /// Exits on EOF.
        pub fn read_loop(&mut self) -> io::Result<()> {
            enable_raw_mode()?;
            loop {
                let evt = event::read()?;
                match evt {
                    Event::FocusGained => {}
                    Event::FocusLost => {}
                    Event::Key(keyevt) => {
                        // ^C?
                        if keyevt.modifiers == KeyModifiers::CONTROL &&
                            (keyevt.code.is_char('c') || keyevt.code.is_char('C')) {
                            break;
                        }

                    }
                    Event::Mouse(mouseevt) => {}
                    Event::Paste(pasted) => {
                        self.current.push_str(&pasted);
                        self.cursor += pasted.len();
                    }
                    Event::Resize(cols, rows) => {}
                }
            }
            disable_raw_mode()
        }
    }
}

#[cfg(feature = "__bytes")]
pub use repl_bytes::*;