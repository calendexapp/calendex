use chrono::{Utc, DateTime};
use icalendar::{Event, Class, Property, Component, Calendar};

//use fake::{faker::{job::raw::Title, address::raw::{StreetName, CountryName}, company::raw::CatchPhase, chrono::raw::DateTimeBetween}, Fake, locales::EN};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EventFields{
    summary: String,
    description: String,
    location: String,
    uid: String,
    starts: String, //Should be DateTime<Utc>
    ends: String,
}

// TODO: 
pub fn create_calendar() -> Calendar {
    let mut calendar = Calendar::new()
    .name("example calendar")
    .done();
    calendar
}

#[tauri::command]
pub fn create_event(event: EventFields) {
      println!("{:?}", event);
    // let created_event = Event::new()
    //     .summary(event.summary.as_str())
    //     .description(event.description.as_str())
    //     .starts(event.starts)
    //     .ends(event.ends)
    //     .uid(event.uid.as_str())
    //     //.venue(event_param.location, "Test uid") //TODO: Look into VVenues 
    //     .location(event.location.as_str())
    //     .class(Class::Confidential)
    //     .append_property(Property::new("TEST", "FOOBAR")
    //               .add_parameter("IMPORTANCE", "very")
    //               .add_parameter("DUE", "tomorrow")
    //               .done())
    //     .done();

    //TODO send event and push it to calender
}

