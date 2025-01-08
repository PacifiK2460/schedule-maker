use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Weekday};
use clap::Parser;
use ics::{
    properties::{Description, DtEnd, DtStart, Summary},
    Event, ICalendar,
};
use serde::Deserialize;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    path::PathBuf,
    vec,
};
use toml::value::Time;

/// A simple Scheduler Maker
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Input file to process
    #[arg(short, long)]
    input_file: String,

    /// Output file to export to (defaults to current directory)
    #[arg(short, long, default_value = ".")]
    output_directory: PathBuf,
}

#[derive(Deserialize, Debug, Clone)]
struct Day {
    day: String,
    start: Time,
    end: Time,
}
#[derive(Deserialize, Debug, Clone)]
struct Subject {
    name: String,
    class_id: String,
    professor: String,
    days: Vec<Day>,
}
#[derive(Deserialize, Debug, Clone)]
struct Subjects {
    subject: Vec<Subject>,
}

impl Deref for Subjects {
    type Target = Vec<Subject>;

    fn deref(&self) -> &Self::Target {
        &self.subject
    }
}

impl DerefMut for Subjects {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.subject
    }
}

fn sort_subjects_by_name(subjects: Subjects) -> HashMap<String, Subjects> {
    // There could be subjects that have the same name, but different class_id.
    // We need to sort them so we have a vector of subjects with the same name
    // and different class_id.

    let mut subjects_by_name: HashMap<String, Subjects> = HashMap::new();

    for subject in subjects.iter() {
        let name = subject.name.clone();

        if subjects_by_name.contains_key(&name) {
            subjects_by_name
                .get_mut(&name)
                .unwrap()
                .subject
                .push(subject.clone());
        } else {
            subjects_by_name.insert(
                name,
                Subjects {
                    subject: vec![subject.clone()],
                },
            );
        }
    }

    subjects_by_name
}

fn subject_overlap(subjects: &Subjects, subject: &Subject) -> bool {
    // Check if the subject overlaps with any of the subjects in the subjects vector
    for s in subjects.subject.iter() {
        for day in s.days.iter() {
            for d in subject.days.iter() {
                if day.day == d.day {
                    if day.start < d.end && day.end > d.start {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn _get_combination_of_subjects(
    hashmap: &HashMap<String, Subjects>,
    current_key: &String,
    remaining_keys: Vec<&String>,
    possible_combination: &mut Vec<Subjects>,
    current_index: usize,
) {
    let current_key = current_key.to_string();

    let subjects_in_current_key = match hashmap.get(&current_key) {
        Some(s) => s,
        None => todo!("Unknown Case"),
    };

    for subject in subjects_in_current_key.iter() {
        let mut new_branch = possible_combination[current_index].clone();

        let overlap = subject_overlap(&new_branch, subject);
        let last_layer = remaining_keys.is_empty();
        new_branch.push(subject.clone());

        if !overlap && last_layer {
            possible_combination.push(new_branch);
        } else if overlap && last_layer {
            continue;
        } else if !overlap && !last_layer {
            possible_combination.push(new_branch);
            //recursive run i++

            let mut new_remaining_keys = remaining_keys.clone();
            let new_current_key = match new_remaining_keys.pop() {
                Some(nck) => nck,
                None => todo!("Unknown Case, since the keys were already checked to be non-empty"),
            };

            _get_combination_of_subjects(
                hashmap,
                new_current_key,
                new_remaining_keys,
                possible_combination,
                current_index + 1,
            );
        } else if overlap && !last_layer {
            // recursive
            let mut new_remaining_keys = remaining_keys.clone();
            let new_current_key = match new_remaining_keys.pop() {
                Some(nck) => nck,
                None => todo!("Unknown Case, since the keys were already checked to be non-empty"),
            };

            _get_combination_of_subjects(
                hashmap,
                new_current_key,
                new_remaining_keys,
                possible_combination,
                current_index,
            );
        }
    }
}

fn get_combination_of_subjects(hashmap: &HashMap<String, Subjects>) -> Vec<Subjects> {
    let mut valid_combinations: Vec<Subjects> = Vec::new();

    let keys = hashmap.keys().collect::<Vec<&String>>();

    // Iterate over the keys, so we get the combination of subjects
    for (i, root_key) in keys.iter().enumerate() {
        // Get the subjects for the current root key
        let mut vc: Vec<Subjects> = Vec::new();
        let remaining_keys = keys[i..].to_vec();

        // Append an empty vector of subjects to vc
        vc.push(Subjects { subject: vec![] });

        _get_combination_of_subjects(hashmap, root_key, remaining_keys, &mut vc, 0);

        valid_combinations.append(&mut vc);
    }

    return valid_combinations;
}

fn main() {
    let args = Args::parse();

    let input_file = match std::fs::read_to_string(&args.input_file) {
        Ok(file) => file,
        Err(e) => panic!("Error reading file: {}", e),
    };

    // Check if the output directory exists, if not create it
    if !args.output_directory.exists() {
        std::fs::create_dir_all(&args.output_directory).expect("Failed to create output directory");
    }

    let subjects: Subjects = match toml::from_str(&input_file) {
        Ok(subjects) => subjects,
        Err(e) => panic!("Error parsing TOML: {}", e),
    };

    let subjects_by_name = sort_subjects_by_name(subjects);

    let mut valid_combinations = get_combination_of_subjects(&subjects_by_name);

    // Sort the valid combinations by the number of subjects in each combination, where the first combination has the most subjects and the last combination has the least subjects
    valid_combinations.sort_by(|a, b| b.len().cmp(&a.len()));

    let day_map: HashMap<&str, Weekday> = [
        ("monday", Weekday::Mon),
        ("tuesday", Weekday::Tue),
        ("wednesday", Weekday::Wed),
        ("thursday", Weekday::Thu),
        ("friday", Weekday::Fri),
        ("saturday", Weekday::Sat),
        ("sunday", Weekday::Sun),
    ]
    .iter()
    .cloned()
    .collect();

    // Define a base date (e.g., the start of the current week) // Change so it's dynamic
    let current_year = 2025;
    let current_month = 1;
    let current_day = 6;

    // Example base date (Monday)
    let base_date = match NaiveDate::from_ymd_opt(current_year, current_month, current_day) {
        Some(date) => date,
        None => panic!("Invalid date"),
    };

    for (index, valid_combination) in valid_combinations.iter().enumerate() {
        let mut calendar = ICalendar::new("2.0", "-//Santiago Lara//Scheduler Maker//EN");

        for subject in valid_combination.iter() {
            for day in subject.days.clone() {
                if let Some(&weekday) = day_map.get(day.day.to_lowercase().as_str()) {
                    let event_date = base_date
                        + chrono::Duration::days(
                            (weekday.num_days_from_monday()
                                - base_date.weekday().num_days_from_monday())
                                as i64,
                        );

                    let start_time = match NaiveTime::from_hms_opt(
                        day.start.hour.into(),
                        day.start.minute.into(),
                        day.start.second.into(),
                    ) {
                        Some(time) => time,
                        None => panic!("Invalid time"),
                    };
                    let end_time = match NaiveTime::from_hms_opt(
                        day.end.hour.into(),
                        day.end.minute.into(),
                        day.end.second.into(),
                    ) {
                        Some(time) => time,
                        None => panic!("Invalid time"),
                    };

                    let start_datetime = NaiveDateTime::new(event_date, start_time);
                    let end_datetime = NaiveDateTime::new(event_date, end_time);

                    let mut event = Event::new(
                        format!("{}-{}", subject.class_id, day.day),
                        start_datetime.format("%Y%m%dT%H%M%S").to_string(),
                    );
                    event.push(DtStart::new(
                        start_datetime.format("%Y%m%dT%H%M%S").to_string(),
                    ));
                    event.push(DtEnd::new(end_datetime.format("%Y%m%dT%H%M%S").to_string()));
                    let desc = format!(
                        "{} ({}) - {}",
                        subject.name, subject.class_id, subject.professor
                    );
                    event.push(Summary::new(desc.clone()));
                    event.push(Description::new(desc));

                    calendar.add_event(event);
                }
            }
        }

        let filename = format!("events_{}.ics", index);

        // Save to the output directory with the filename
        let filename = args.output_directory.join(filename);

        // Save the calendar to a file
        match calendar.save_file(filename) {
            Ok(_) =>
                /* do nothing */
                {}
            Err(e) => println!("Error saving calendar: {}", e),
        }
    }

    println!("Scheduler Maker made {} files", valid_combinations.len());
}
