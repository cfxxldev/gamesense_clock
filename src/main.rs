#![windows_subsystem = "windows"]
extern crate anyhow;
extern crate gamesense;
extern crate serde_json;

use std::time::SystemTime;

use anyhow::Result;
use chrono::{DateTime, Local};
use gamesense::client::GameSenseClient;
use gamesense::handler::screen;
use serde_json::json;

fn main() -> Result<()> {
    let client = GameSenseClient::new("MY_CLOCK", "Current Time and Date", "cfxxldev", None)?;

    let handler = screen::ScreenHandler::new(
        "screened",
        "one",
        screen::ScreenDataDefinition::StaticScreenDataDefinition(
            screen::StaticScreenDataDefinition(vec![screen::ScreenFrameData::MultiLineFrameData(
                screen::MultiLineFrameData {
                    frame_modifiers_data: Some(screen::FrameModifiersData {
                        length_millis: None,
                        icon_id: Some(screen::Icon::Clock),
                        repeats: Some(screen::Repeat::Bool(true)),
                    }),
                    lines: vec![
                        screen::LineData {
                            type_options: screen::LineDataType::TextModifiersData(
                                screen::TextModifiersData {
                                    has_text: true,
                                    prefix: None,
                                    suffix: None,
                                    bold: Some(true),
                                    wrap: None,
                                },
                            ),
                            data_accessor_data: Some(screen::DataAccessorData {
                                arg: None,
                                context_frame_key: Some(String::from("time")),
                            }),
                        },
                        screen::LineData {
                            type_options: screen::LineDataType::TextModifiersData(
                                screen::TextModifiersData {
                                    has_text: true,
                                    prefix: None,
                                    suffix: None,
                                    bold: None,
                                    wrap: None,
                                },
                            ),
                            data_accessor_data: Some(screen::DataAccessorData {
                                arg: None,
                                context_frame_key: Some(String::from("date")),
                            }),
                        },
                    ],
                },
            )]),
        ),
    );

    client.bind_event("CLOCK_EVENT", None, None, None, None, vec![handler])?;
    loop {
        let system_time = SystemTime::now();
        let datetime: DateTime<Local> = system_time.into();

        client.trigger_event_frame(
            "CLOCK_EVENT",
            (datetime.timestamp_millis() % 10000) as isize,
            json!({
                "time": datetime.format("%H:%M:%S").to_string(),
                "date": datetime.format("%d.%m.%Y").to_string(),
            }),
        )?;
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
