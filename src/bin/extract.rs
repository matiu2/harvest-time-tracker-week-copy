//! Extracts data from data.json

use std::{
    collections::{HashMap, HashSet},
    fs::File,
};

use harvest_time_tracker::model::{ExternalReference, Project, TimeEntries, TimeEntry};
fn main() {
    pretty_env_logger::init();

    // Get the start and end of the week we want to extract
    let (start, end) = {
        let vars: Vec<&str> = std::env::vars().skip(1).collect();
        match vars.as_slice() {
            [start, end] => (start, end),
            other => panic!(
                "Expected start and end dates as parameters but got: {:?}",
                other
            ),
        }
    };

    // Read the data
    let f = File::open("data.json").unwrap();
    let data: Vec<TimeEntries> = serde_json::de::from_reader(&f).unwrap();
    let entries = || data.iter().flat_map(|entries| &entries.time_entries);

    // Get all the dates
    let mut dates: Vec<&str> = entries().map(|e| e.spent_date.as_str()).collect();
    dates.sort();
    dates.iter().for_each(|date| {
        println!("{}", date);
    });

    // Get all the projects
    let projects: HashSet<&Project> = entries().map(|e| &e.project).collect();
    projects.iter().for_each(|project| {
        println!("{:?}", project);
    });
    let f = File::create("projects.json").unwrap();
    serde_json::ser::to_writer_pretty(f, &projects).unwrap();

    // Get all the external references - there are many with the same ID.
    // Per ID, we only want the one with the shortest permalink
    let external_references: HashMap<&str, &ExternalReference> = entries()
        .flat_map(|e| &e.external_reference)
        .fold(HashMap::new(), |mut map, r: &ExternalReference| {
            match map.get(r.id.as_str()) {
                Some(shortest) if r.permalink.len() < shortest.permalink.len() => {
                    map.insert(&r.id, r);
                }
                None => {
                    map.insert(&r.id, r);
                }
                _ => (),
            };
            map
        });

    let mut external_references: Vec<&ExternalReference> =
        external_references.values().cloned().collect();
    external_references.sort_by_key(|reference| reference.id.as_str());
    for reference in &external_references {
        println!("{:?}", reference);
    }
    // Save the external references to a file
    let f = File::create("external_refrences.json").unwrap();
    serde_json::ser::to_writer_pretty(f, &external_references).unwrap();

    // Now get all the entries that are on the week starting Mon 25 October
    let week: Vec<&TimeEntry> = entries()
        .filter(|e| e.spent_date.as_str() >= *start && e.spent_date.as_str() <= *end)
        .collect();
    let f = File::create("source_week.json").unwrap();
    serde_json::ser::to_writer_pretty(f, &week).unwrap();
}
