use ark_ec::pairing::Pairing;
use serde::Deserialize;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr; // Import IntoDeserializer trait

#[derive(Deserialize)]
#[serde(tag = "tag", rename_all = "lowercase")]
pub struct InputJson {
    var: usize,
    value: String,
}

#[derive(Debug)]
pub struct Inputs<E: Pairing> {
    pub inputs: Vec<(usize, E::ScalarField)>,
}

pub fn parse_inputs_file<E: Pairing>(reader: BufReader<File>) -> io::Result<Inputs<E>> {
    let lines = reader.lines();

    let mut inputs_data = Vec::new();
    for line in lines {
        let line = line.expect("Error reading line from inputs file");
        let input = serde_json::from_str::<InputJson>(&line).expect("Error parsing JSON to Input");
        match input {
            InputJson { var, value } => {
                let val = E::ScalarField::from_str(&value).map_err(|_| {
                    io::Error::new(io::ErrorKind::Other, "Error parsing field element")
                })?;
                inputs_data.push((var, val));
            }
        }
    }

    Ok(Inputs {
        inputs: inputs_data,
    })
}
