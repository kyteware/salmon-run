use iced::{time, widget::{button, column, row, slider, text, text_editor}, Element, Length, Subscription};

use crate::{grid::Grid, instruction::{parse_code, Instruction}};

#[derive(Debug, Default)]
pub struct SalmonRun {
    running: bool,
    grid: Grid,
    code: text_editor::Content
}

impl SalmonRun {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ToggleRunning => { self.running = !self.running },
            Message::Tick => self.grid.tick(),
            Message::ShuffleRocks => { self.grid.shuffle() },
            Message::PlaceSalmon => { self.grid.reset_salmon() },
            Message::CodeAction(action) => {
                let needs_reload = matches!(action, text_editor::Action::Edit(_));
                self.code.perform(action);
                if needs_reload {
                    let new_instructions = parse_code(&self.code.text()).unwrap_or(vec![]);
                    if new_instructions.len() > 0 {
                        self.grid.instructions = new_instructions;
                    } else {
                        self.grid.instructions = vec![Instruction::Right];
                    }
                    self.grid.reset_salmon();
                }
            }
            Message::SwitchLevel(level) => {
                todo!("switch level")
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        row!(
            column!(
                row!(
                    button(if self.running { "stop" } else { "run" }).on_press(Message::ToggleRunning),
                    button("shuffle rocks").on_press(Message::ShuffleRocks),
                    button("place salmon").on_press(Message::PlaceSalmon),
                    slider(1..=1, 1, Message::SwitchLevel)
                ).spacing(10),
                self.grid.view(),
            ).spacing(10),
            column!(text("code"), text_editor(&self.code).on_action(Message::CodeAction).height(Length::Fill))
        )   
            .padding(10)
            .spacing(10)
            .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        if self.running {
            time::every(time::Duration::from_millis(1000)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleRunning,
    Tick,
    ShuffleRocks,
    PlaceSalmon,
    CodeAction(text_editor::Action),
    SwitchLevel(u32)
}
