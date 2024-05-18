use iced::{time, widget::{button, column, row, slider, text, text_editor}, Border, Color, Element, Length, Subscription, Theme};

use crate::{grid::{Direction, Grid}, instruction::{compile, Instruction}, level::Level};

#[derive(Debug, Default)]
pub struct SalmonRun {
    running: bool,
    level: u32,
    grid: Grid,
    code: text_editor::Content,
    code_is_good: bool,
    levels: Vec<Level>,
    last_good_code: Vec<Instruction>,
    won: bool
}

impl SalmonRun {
    pub fn new() -> Self {
        let levels = Level::load_all();
        let last_good_code = vec![Instruction::Move(Direction::Left), Instruction::Move(Direction::Right)];
        Self {
            running: false,
            level: 1,
            grid: Grid::load_level(&levels[0], last_good_code.clone()),
            code: text_editor::Content::with_text("move left;\nmove right;\n"),
            code_is_good: true,
            levels,
            last_good_code,
            won: false
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ToggleRunning => { self.running = !self.running },
            Message::Tick => {
                if self.grid.tick() {
                    self.won = true;
                    self.running = false;
                }
            },
            Message::CodeAction(action) => {
                let needs_reload = matches!(action, text_editor::Action::Edit(_));
                self.code.perform(action);
                if needs_reload {
                    let new_instructions = compile(&self.code.text()).unwrap_or(vec![]);
                    if new_instructions.len() > 0 {
                        self.last_good_code = new_instructions;
                        self.code_is_good = true;
                        dbg!(&self.last_good_code);
                    } else {
                        self.code_is_good = false;
                    }
                    self.restart();
                }
            }
            Message::SwitchLevel(level) => {
                self.level = level;
                self.restart();
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let run_button = if self.won { button("level complete") } else { button(if self.running { "stop" } else { "run" }).on_press(Message::ToggleRunning) };
        row!(
            column!(
                row!(
                    run_button,
                    text(format!("Current level: {}", self.level)),
                    slider(1..=(self.levels.len() as u32), self.level, Message::SwitchLevel)
                ).spacing(10),
                self.grid.view(),
            ).spacing(10),
            column!(
                text("code"), 
                text_editor(&self.code).on_action(Message::CodeAction).height(Length::Fill).style(|theme: &Theme, status| {
                    let class = <Theme as text_editor::Catalog>::default();
                    let mut style: text_editor::Style = text_editor::Catalog::style(theme, &class, status);
                    if !self.code_is_good {
                        style.border.color = Color::new(0.8, 0.2, 0.2, 1.);
                    }
                    style
                }),
                text(
r##"Instructions
Movement:
```
move right;
move left;
move down;
move up;
```

Loops:
```
loop {
 move up;
}
```"##
                ),
            )
        )   
            .padding(10)
            .spacing(10)
            .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        if self.running {
            time::every(time::Duration::from_millis(300)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }
    
    fn restart(&mut self) {
        self.won = false;
        self.grid = Grid::load_level(&self.levels[self.level as usize - 1], self.last_good_code.clone());
        self.running = false;
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleRunning,
    Tick,
    CodeAction(text_editor::Action),
    SwitchLevel(u32)
}
