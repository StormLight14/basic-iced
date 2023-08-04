use iced::futures;
use iced::theme::ProgressBar;
use iced::widget::{column, container, row, text, Text, button, slider, progress_bar, text_input};
use iced::{
    Alignment, Sandbox, Color, Command, Element, Length, Settings, Theme,
};

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

enum State{
    Counter,
    ProgressBar,
    NameInput,
    NotFound
}

struct App {
    state: State,
    counter_value: i32,
    progress_bar_value: f32,
    name: String,
    page: u8,
    pages: u8,
}

#[derive(Debug, Clone)]
enum Message {
    IncrementPressed,
    DecrementPressed,
    SliderMoved(f32),
    NameInputted(String),
    PageBack,
    PageForward,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            state: State::Counter,
            counter_value: 0,
            progress_bar_value: 0.0,
            name: String::new(),
            page: 1,
            pages: 3,
        }
    }

    fn title(&self) -> String {
        let subtitle = match self.state {
            State::Counter => "Counter",
            State::ProgressBar => "Progress Bar",
            State::NameInput => "Name Input",
            State::NotFound => "Page not found"
        };

        format!("{subtitle} - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.counter_value += 1;
            }
            Message::DecrementPressed => {
                self.counter_value -= 1;
            }
            Message::SliderMoved(slider_value) => {
                self.progress_bar_value = slider_value
            }
            Message::NameInputted(name) => {
                self.name = name
            }
            Message::PageForward => {
                if self.page < self.pages {
                    self.page += 1;
                } else {
                    self.page = 1;
                }
            }
            Message::PageBack => {
                if self.page > 1 {
                    self.page -= 1;
                } else {
                    self.page = self.pages;
                }
            }
        }

        match self.page {
            1 => {
                self.state = State::Counter
            }
            2 => {
                self.state = State::ProgressBar
            }
            3 => {
                self.state = State::NameInput
            }
            _ => {
                self.state = State::NotFound
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = match self.state {
            State::Counter => { // Pattern match the variant and destructure the value
                column![
                    button("Increment").on_press(Message::IncrementPressed),
                    text(self.counter_value.to_string()).size(50),
                    button("Decrement").on_press(Message::DecrementPressed),
                    container(PageButton::view(&PageButton {button_type: ButtonType::NextPage})).width(Length::Fill).center_x(),
                    container(PageButton::view(&PageButton {button_type: ButtonType::PreviousPage})).width(Length::Fill).center_x(),
                
                ]
                .padding(20)
                .align_items(Alignment::Center)
            }

            State::ProgressBar => {
                column![
                    column![
                        progress_bar(0.0..=100.0, self.progress_bar_value),
                        slider(0.0..=100.0, self.progress_bar_value, Message::SliderMoved).step(0.01),
                        container(PageButton::view(&PageButton {button_type: ButtonType::NextPage})).width(Length::Fill).center_x(),
                        container(PageButton::view(&PageButton {button_type: ButtonType::PreviousPage})).width(Length::Fill).center_x(),
                    ]
                    
                    
                ]
                .padding(20)
                    .into()
            }

            State::NameInput => {
                column![
                    text_input("Enter your name...", &self.name).on_input(Message::NameInputted),
                    text(format!("Hello, {}", self.name)).size(15),
                    container(PageButton::view(&PageButton {button_type: ButtonType::NextPage})).width(Length::Fill).center_x(),
                    container(PageButton::view(&PageButton {button_type: ButtonType::PreviousPage})).width(Length::Fill).center_x(),
                ]
            }
            
            State::NotFound => {
                column![].into()
            }
        };

        container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}

#[derive(Debug, Clone)]
enum ButtonType {
    NextPage,
    PreviousPage,
}


#[derive(Debug, Clone)]
struct PageButton {
    button_type: ButtonType
}

impl PageButton {
    fn view(&self) -> Element<Message> {
        match self.button_type {
            ButtonType::NextPage => {
                column![
                    button("Next Page").on_press(Message::PageForward)
                ]
                .padding(20)
                .align_items(Alignment::Center)
                .into()
            }
            ButtonType::PreviousPage => {
                column![
                    button("Previous Page").on_press(Message::PageBack)
                ]
                .padding(0)
                .align_items(Alignment::Center)
                .into()
            }
        }
        
    }
}

#[derive(Debug, Clone)]
struct PageButtons {

}

impl PageButtons {
    fn view(&self) -> Element<Message> {
        container(
            column![
                PageButton::view(&PageButton {button_type: ButtonType::NextPage}),
                PageButton::view(&PageButton {button_type: ButtonType::PreviousPage}),
            ]
        ).into()
    }
    
}