#[macro_use]
extern crate clap;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
use clap::{App, Arg};
use std::io::{Error, Write};
use std::fs;
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
	media_owner: Option<String>,
}

fn main() -> Result<(), Error> {
	// clap app creation, with macros that read project information from Cargo.toml.
	let matches = App::new(crate_name!())
		.version(crate_version!())
		.about(crate_description!())
		.author(crate_authors!())
		.arg(Arg::with_name("filepath")
			.help("The .json file you wish to convert to .txt files.")
			.required(true)
			.index(1))
		.arg(Arg::with_name("folder")
			.help("Optional export path. Will export to current directory if not set.")
			.takes_value(true)
			.short("f")
			.long("folder"))
		.get_matches();

	// Define variables.
	let now = Instant::now();
	let filepath = matches.value_of("filepath").unwrap();
	let mut dirname = String::new();
	if matches.is_present("folder") { // if the folder arg exists
		let exdir = matches.value_of("folder").unwrap();
		fs::create_dir_all(exdir)?;
		let dirname_owned = exdir.to_owned() + "/";
		dirname = dirname_owned;
	}
	let file = File::open(&filepath)?;

	println!("Reading JSON.. this might take a while, depending on the size.");
	let mut things: Vec<Thing> = serde_json::from_reader(file)?;

	println!("Starting extraction from {}.", filepath);

	for thing in things.iter_mut().rev() { // reverse to sort properly
		let mut name = thing.participants.iter().fold(String::new(), |mut acc, s| { // get participants
			if !acc.is_empty() {
				acc.push_str(", ");
			}
			acc + s
		});
		let names = name.clone();
		name.push_str(".txt");
		let fulldir = dirname.to_owned() + &name;
		let mut file = OpenOptions::new().write(true).create(true).append(true).open(fulldir)?;

		file.write_all(format!("-- Start of conversation with {} --\n", names).as_bytes())?;

		for message in &mut thing.conversation.iter_mut().rev() { // reverse to sort properly (again)
			(0..13).for_each(|_| { message.created_at.pop(); }); // bodges to make the time look nice
			let timestamp = message.created_at.replace("T", " "); // more bodges
			let string = format!(
				"({}) {}: {}{}{}\n",
				timestamp,
				message.sender,
				if let Some(ref story_share) = message.story_share { // if a shared story exists
					if message.text.as_ref().map(|s| &**s).unwrap_or("") != "" { story_share.to_owned() + " - " } else { story_share.to_owned() }
				} else { String::new() },
				if let Some(ref media_owner) = message.media_owner { // if a shared post exists
					if media_owner == "Username unavailable." { // if the post is no longer available
						"Sent an unavailable post".to_owned()
					} else {
						"Sent a post by ".to_owned() + &media_owner.to_owned()
					}
				} else { String::new() },
				if let Some(ref media) = message.media { media } else { message.text.as_ref().map(|s| &**s).unwrap_or("") }
			);
			file.write_all(string.as_bytes())?; // write the results to the .txt
		}
		file.write_all("-- End --".as_bytes())?;
	}
	
	println!("Finished in {} seconds.", now.elapsed().as_secs());
	Ok(()) // ok
}
