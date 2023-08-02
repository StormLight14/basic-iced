use iced::futures;
use iced::theme::ProgressBar;
use iced::widget::{column, container, row, text, Text, button, slider, progress_bar};
use iced::{
    Alignment, Sandbox, Color, Command, Element, Length, Settings, Theme,
};

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

enum App {
    Counter {value: i32},
    ProgressBar {value: f32},
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
    SliderMoved(f32),
    PageBack,
    PageForward,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self::Counter { value: 0 } // Initialize the App with the Counter variant and value 0
    }

    fn title(&self) -> String {
        let subtitle = match self {
            App::Counter { .. } => "Counter",
            App::ProgressBar { .. }=> "Progress Bar"
        };

        format!("{subtitle} - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                if let App::Counter { value } = self {
                    *value += 1; // Access and update the value when in Counter variant
                }
            }
            Message::DecrementPressed => {
                if let App::Counter { value } = self {
                    *value -= 1; // Access and update the value when in Counter variant
                }
            }
            Message::SliderMoved(slider_value) => {
                if let App::ProgressBar { value } = self {
                    *value = slider_value
                }
            }
            Message::PageForward => {
                if let App::ProgressBar { value } = self {
                    *self = App::ProgressBar { value: *value}
                } else {
                    *self = App::ProgressBar { value: 0.0 }
                }
            }
            Message::PageBack => {
                if let App::Counter { value } = self {
                    *self = App::Counter { value: *value }
                } else {
                    *self = App::Counter { value: 0 }
                }
            }
                
        }
    }

    fn view(&self) -> Element<Message> {
        let content = match self {
            App::Counter { value } => { // Pattern match the variant and destructure the value
                column![
                    button("Increment").on_press(Message::IncrementPressed),
                    text(value.to_string()).size(50),
                    button("Decrement").on_press(Message::DecrementPressed),
                    PageButton::view(&PageButton {button_type: ButtonType::NextPage})
                
                ]
                .padding(20)
                .align_items(Alignment::Center)
            }

            App::ProgressBar { value }=> {
                column![
                    column![
                        progress_bar(0.0..=100.0, *value),
                        slider(0.0..=100.0, *value, Message::SliderMoved).step(0.01),
                        container(PageButton::view(&PageButton {button_type: ButtonType::PreviousPage})).width(Length::Fill).center_x()
                    ]
                    
                    
                ]
                .padding(20)
                    .into()
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
                .padding(30)
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