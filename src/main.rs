#[macro_use]
extern crate clap;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
use clap::{App, Arg};
use std::io::{Error, Write};
use std::fs::{File, OpenOptions};
use std::time::Instant;

#[derive(Deserialize)]
struct Thing {
	participants: Vec<String>,
	conversation: Vec<Message>,
}

#[derive(Deserialize)]
struct Message {
	sender: String,
	created_at: String,
	story_share: Option<String>,
	text: Option<String>,
	media: Option<String>,
	// to-do: include "Shared <username>'s post"
}

fn main() -> Result<(), Error> {
	// clap app creation, with macros that read project information from Cargo.toml.
	let matches = App::new(crate_name!())
		.version(crate_version!())
		.about(crate_description!())
		.author(crate_authors!())
		.arg(Arg::with_name("filepath")
			.help("The .json file you wish to convert to .txt files.")
			.required(true) // Make argument required.
			.index(1))
		.get_matches();

	// Define variables.
	let now = Instant::now();
	let filepath = matches.value_of("filepath").unwrap();
	let file = File::open(&filepath)?;

	println!("Loading JSON.. this might take a while, depending on the size.");
	let mut things: Vec<Thing> = serde_json::from_reader(file)?;

	println!("Starting extraction from {}.", filepath);

	for thing in things.iter_mut().rev() { // reverse to sort properly
		let mut name = thing.participants.iter().fold(String::new(), |mut acc, s| { // get participants
			if !acc.is_empty() {
				acc.push_str(", ");
			}
			acc + s
		});
		name.push_str(".txt");
		let mut file = OpenOptions::new().write(true).create(true).append(true).open(name)?;

		for message in &mut thing.conversation.iter_mut().rev() { // reverse to sort properly (again)
			(0..13).for_each(|_| { message.created_at.pop(); }); // bodges to make the time look nice
			let timestamp = message.created_at.replace("T", " "); // more bodges
			let string = format!(
				"({}) {}: {}{}\n",
				timestamp,
				message.sender,
				if let Some(ref story_share) = message.story_share { story_share.to_owned() + " - " } else { String::new() },
				if let Some(ref media) = message.media { media } else { message.text.as_ref().map(|s| &**s).unwrap_or("") }
			);
			file.write_all(string.as_bytes())?; // write the results to the .txt
		}
	}
	
	println!("Finished in {} seconds.", now.elapsed().as_secs());
	Ok(()) // ok
}
