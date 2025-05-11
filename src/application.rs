use std::{rc::Rc, time::Duration};

use calloop::{EventLoop, EventSource, LoopHandle};
use smithay::backend::input::{AbsolutePositionEvent, InputBackend, InputEvent};

use crate::{
    core::{
        ui_node::AbsoluteUiNode,
        ui_tree::{AbsoluteUiTree, UiTree},
        widget::Widget,
    },
    geometry::Point,
};

struct Application<'a, LoopData, T> {
    event_loop: EventLoop<'a, LoopData>,
    ui_tree: UiTree<T>,
    focused_widget: Option<Rc<dyn Widget<Texture = T>>>,
    running: bool,
}

impl<'a, LoopData, T> Application<'a, LoopData, T> {
    pub fn new<B, F>(input_backend: B, ui_tree: UiTree<T>) -> Self
    where
        B: EventSource<Event = InputEvent<B>, Ret = ()> + InputBackend + 'a,
    {
        let mut event_loop = EventLoop::try_new().unwrap();

        event_loop
            .handle()
            .insert_source(input_backend, |event, metadata, loop_data| match event {
                InputEvent::DeviceAdded { device } => todo!(),
                InputEvent::DeviceRemoved { device } => todo!(),
                InputEvent::Keyboard { event } => todo!(),
                InputEvent::PointerMotion { event } => {
                    println!("ignoring pointer motion")
                }
                InputEvent::PointerMotionAbsolute { event } => {
                    println!("ignoring absolute pointer motion")
                }
                InputEvent::PointerButton { event } => todo!(),
                InputEvent::PointerAxis { event } => todo!(),
                _ => {
                    println!("Received unsupported InputEvent");
                }
            })
            .unwrap();

        Self {
            event_loop,
            ui_tree,
            focused_widget: None,
            running: false,
        }
    }

    /// Problem:
    /// I have no fucking clue why the whole PlacedELements and Widget stuff
    /// was working without Rc in the first place. In any case now im having
    /// issues cause of life times in this function. But i can't just use Rc
    /// cause of how the PlacedLayout stores PlacedElements in a Vec.
    /// TL;DR ovethink the entirety of memory management
    fn hit_test(
        &self,
        position: Point,
        root: Option<Rc<AbsoluteUiNode<T>>>,
    ) -> Option<Rc<AbsoluteUiNode<T>>> {
        match root {
            Some(element) => match *element {
                AbsoluteUiNode::Widget(placed_widget) => {
                    if placed_widget.rect.contains(position) {
                        Some(element)
                    } else {
                        None
                    }
                }
                AbsoluteUiNode::Layout(placed_layout) => todo!(),
            },
            None => match &self.ui_tree.placed_tree {
                Some(placed_tree) => {
                    if placed_tree.root.rect.contains(position) {
                        for child in &placed_tree.root.children {
                            return self.hit_test(position, Some(Into::into(child)));
                        }
                        None
                    } else {
                        None
                    }
                }
                None => None,
            },
        }
    }

    pub fn run(&mut self, data: &mut LoopData) {
        self.running = true;
        while self.running {
            match self
                .event_loop
                .dispatch(Some(Duration::from_millis(16)), data)
            {
                Ok(_) => {}
                Err(_) => self.running = false,
            }
        }
    }
}
