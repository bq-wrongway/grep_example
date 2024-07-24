use std::io::IsTerminal;

use grep::cli;
use grep::matcher::Matcher;
use grep::printer::{ColorSpecs, StandardBuilder};
use grep::regex::RegexMatcher;
use grep::searcher::sinks::UTF8;
use grep::searcher::{BinaryDetection, SearcherBuilder};
use iced::border::Radius;
use iced::widget::{button, column, text, text_input, Column};
use iced::{Background, Border, Center, Color, Theme};
use termcolor::ColorChoice;
use walkdir::WalkDir;

pub fn main() -> iced::Result {
    iced::application("A cool counter", Counter::update, Counter::view).run()
}

#[derive(Default)]
struct Counter {
    value: String,
}

#[derive(Debug, Clone)]
enum Message {
    Search(String),
    Submit,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Search(msg) => {
                self.value = msg;
            }
            Message::Submit => {
                println!("print{}", self.value);

                // search_with_cool_sink();
                search_and_add_to_vector();
            }
        }
    }

    fn view(&self) -> Column<Message> {
        column![
            text_input("placeholder", self.value.as_str())
                .on_input(Message::Search)
                .on_submit(Message::Submit)
                .width(150)
                .style(|t: &Theme, s| {
                    let pal = t.palette();
                    text_input::Style {
                        background: Background::Color(Color::TRANSPARENT),
                        border: Border {
                            color: pal.success,
                            width: 2.,
                            radius: Radius::new(10.0),
                        },
                        icon: pal.primary,
                        placeholder: pal.danger,
                        value: pal.primary,
                        selection: pal.primary,
                    }
                }),
            text(self.value.as_str()).size(50),
        ]
        .padding(20)
        .align_x(Center)
    }
}

fn search_and_add_to_vector() {
    let matcher = grep::regex::RegexMatcher::new_line_matcher("test").unwrap();
    let mut searcher = SearcherBuilder::new()
        .binary_detection(BinaryDetection::quit(b'\x00'))
        .line_number(true)
        .build();

    for result in WalkDir::new("/home/melnibone/Documents/trst/") {
        let dent = match result {
            Ok(dent) => dent,
            Err(_) => {
                println!("no dir");
                continue;
            }
        };
        if !dent.file_type().is_file() {
            continue;
        }
        let result = searcher.search_path(
            &matcher,
            dent.path(),
            UTF8(|lnum, line| {
                let mym = matcher.find(line.as_bytes())?.unwrap();
                println!("{:?} {}", mym, line.to_string());
                Ok(true)
            }),
        );

        if let Err(err) = result {
            eprintln!("{}: {}", dent.path().display(), err);
        }
    }
}
fn search_with_cool_sink() {
    let matcher = grep::regex::RegexMatcher::new_line_matcher("test").unwrap();

    let mut searcher = SearcherBuilder::new()
        .binary_detection(BinaryDetection::quit(b'\x00'))
        .line_number(false)
        .build();
    let mut printer = StandardBuilder::new()
        .color_specs(ColorSpecs::default_with_color())
        .build(cli::stdout(if std::io::stdout().is_terminal() {
            ColorChoice::Auto
        } else {
            ColorChoice::Never
        }));
    for result in WalkDir::new("/home/melnibone/Documents/trst/") {
        let dent = match result {
            Ok(dent) => dent,
            Err(_) => {
                println!("no dir");
                continue;
            }
        };
        if !dent.file_type().is_file() {
            continue;
        }
        let result = searcher
            .search_path(
                &matcher,
                dent.path(),
                printer.sink_with_path(&matcher, dent.path()),
            )
            .unwrap();
    }
}
