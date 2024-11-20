use crate::admin;
use crate::utils;

use std::fs;

pub fn testing_ballot() {
    println!("Creating new ballot");
    _ = admin::create_ballot();

    let metadata_file = fs::File::open(&"./ballot/metadata.json").unwrap();
    let mut metadata: utils::ElectionMetadata = serde_json::from_reader(&metadata_file).unwrap();


    println!("Adding example candidates to ballot");
    admin::write_candidate(&mut metadata, "president", "Alice Johnson", "Republican");
    admin::write_candidate(&mut metadata, "president", "Robert Smith", "Democratic");
    admin::write_candidate(&mut metadata, "president", "Julia Davis", "Independent");

    admin::write_candidate(&mut metadata, "senate", "Michael Brown", "Green");
    admin::write_candidate(&mut metadata, "senate", "Susan Wilson", "Progressive");
    admin::write_candidate(&mut metadata, "senate", "David Martinez", "Libertarian");

    admin::write_candidate(&mut metadata, "judge", "Emily Clark", "Republican");
    admin::write_candidate(&mut metadata, "judge", "James Taylor", "Democratic");
    admin::write_candidate(&mut metadata, "judge", "Joe Rogan", "Libertarian");

    let file = fs::File::create(&"./ballot/metadata.json").unwrap();
    serde_json::to_writer_pretty(&file, &metadata).unwrap();
}


pub fn testing_voter_reg() {
    println!("Creating new voter registry");
    _ = fs::File::create("./voter_db.csv");

    println!("Adding example voters to voter registry");
    _= utils::add_new_voter("./voter_db.csv", &"daniel thompson", &"03/12/1987");
    _= utils::add_new_voter("./voter_db.csv", &"laura anderson", &"06/18/1991");
    _= utils::add_new_voter("./voter_db.csv", &"peter robinson", &"10/07/1984");
    _= utils::add_new_voter("./voter_db.csv", &"linda carter", &"11/22/1993");
    _= utils::add_new_voter("./voter_db.csv", &"george evans", &"02/05/1980");
    _= utils::add_new_voter("./voter_db.csv", &"karen mitchell", &"09/19/1977");
    _= utils::add_new_voter("./voter_db.csv", &"thomas wright", &"01/23/1996");
    _= utils::add_new_voter("./voter_db.csv", &"sarah lewis", &"07/14/1989");
    _= utils::add_new_voter("./voter_db.csv", &"jason walker", &"05/28/2002");
    _= utils::add_new_voter("./voter_db.csv", &"megan hall", &"08/03/1998");
}

pub fn testing_tally_votes() {
    _=admin::tally_votes();
    _=admin::declare_winners();
}