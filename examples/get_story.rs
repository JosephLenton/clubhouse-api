use clubhouse_api;

#[macro_use]
extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "get_story")]
struct Args {
    #[structopt(short = "o", long = "oauth")]
    oauth_token: String,

    #[structopt(name = "TICKET ID")]
    ticket_id: u64,
}

pub fn main() -> Result<(), clubhouse_api::Error> {
    let args = Args::from_args();
    let clubhouse = clubhouse_api::Clubhouse::new(&args.oauth_token);

    let story = clubhouse
        .get()
        .stories()
        .story_public_id(args.ticket_id)
        .run()?;

    println!("");
    println!("# ch{} {}", story.id, story.name);
    println!("{}", story.app_url);
    if story.labels.len() > 0 {
        let label_names: Vec<&str> = story.labels.iter().map(|l| l.name.as_str()).collect();
        let label_str = label_names.join(", ");
        println!("{}", label_str);
    }
    println!("");
    println!("{}", story.description);

    Ok(())
}
