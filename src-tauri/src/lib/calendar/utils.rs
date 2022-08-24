use std::{fs::{self, File}, env, io::BufReader};
use ical::parser::{ical::component::IcalCalendar, ParserError};
use uuid::Uuid;

fn gen_uuid() -> String {
    let uuid = Uuid::new_v4().to_string();
    let tag = String::from("@lanex.com");
    let uuid =format!(
        "{}{}",
        String::from(&uuid[..(uuid.len() - tag.len())]),
        tag
    );
    uuid
}


fn write_calendar_to_file(calendar: String){
    let path_to_current_dir = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    let path_to_file = format!("{}{}",path_to_current_dir,"/tmp/created_cal.ics");

    // TODO: Error handling
    fs::write(path_to_file, calendar).expect("Unable to write file");
}

fn read_calender(buf : BufReader<File>) -> Result<IcalCalendar,ParserError> {
    let mut reader = ical::IcalParser::new(buf);
    let result_cal = reader.nth(0).unwrap();
    result_cal
}
