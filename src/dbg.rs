use crate::voting;
use crate::admin;
use crate::utils;

use std::fs;

pub fn testing_ballot() {
    println!("Creating new ballot");
    admin::create_ballot();

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
    utils::add_new_voter("./voter_db.csv", &"Daniel Thompson", &"03/12/1987");
    utils::add_new_voter("./voter_db.csv", &"Laura Anderson", &"06/18/1991");
    utils::add_new_voter("./voter_db.csv", &"Peter Robinson", &"10/07/1984");
    utils::add_new_voter("./voter_db.csv", &"Linda Carter", &"11/22/1993");
    utils::add_new_voter("./voter_db.csv", &"George Evans", &"02/05/1980");
    utils::add_new_voter("./voter_db.csv", &"Karen Mitchell", &"09/19/1977");
    utils::add_new_voter("./voter_db.csv", &"Thomas Wright", &"01/23/1996");
    utils::add_new_voter("./voter_db.csv", &"Sarah Lewis", &"07/14/1989");
    utils::add_new_voter("./voter_db.csv", &"Jason Walker", &"05/28/2002");
    utils::add_new_voter("./voter_db.csv", &"Megan Hall", &"08/03/1998");
}