use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct FragranceData {
    pub results: Vec<Result>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Result {
    pub hits: Vec<Hit>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hit {
    pub naslov: String,
    pub dizajner: String,
    pub godina: u32,
    pub thumbnail: String,
    pub url: HashMap<String, Vec<String>>,
    pub objectID: String,
}

pub fn read_from_file(file_path: &str) -> io::Result<FragranceData> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data: FragranceData = serde_json::from_str(&contents)?;
    Ok(data)
}

pub fn write_to_file(file_path: &str, data: &FragranceData) -> io::Result<()> {
    let serialized_data = serde_json::to_string_pretty(&data)?;
    let mut file = File::create(file_path)?;
    file.write_all(serialized_data.as_bytes())?;
    Ok(())
}

impl FragranceData {
    pub fn merge(&mut self, other: &FragranceData) {
        if let Some(first_result) = self.results.first_mut() {
            // Collect objectIDs of all current hits for comparison
            let existing_ids: Vec<String> = first_result
                .hits
                .iter()
                .map(|hit| hit.objectID.clone())
                .collect();

            // Collect new hits to be added to avoid mutable-immutable borrow conflict
            let new_hits: Vec<Hit> = other.results.first().map_or(vec![], |other_first_result| {
                other_first_result
                    .hits
                    .iter()
                    .filter(|hit| !existing_ids.contains(&hit.objectID))
                    .cloned() // Clone the hits to be added
                    .collect()
            });

            // Now, append the new hits
            first_result.hits.extend(new_hits);
        } else if let Some(other_first_result) = other.results.first() {
            // If there are no results in self, just clone the first result of other
            self.results.push(other_first_result.clone());
        }
    }

    pub fn get_houses(&self) -> Vec<String> {
        let mut houses: Vec<String> = self.results[0]
            .hits
            .iter()
            .map(|current| current.dizajner.clone())
            .collect();
        houses.sort_unstable();
        houses.dedup();
        houses
    }
}
