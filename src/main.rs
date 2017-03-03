#[macro_use] extern crate clap;
extern crate reqwest;
extern crate rss;

use std::io::{Write, Read, BufReader};
use std::fs::File;
use std::path::Path;
use rss::Channel;
use clap::App;
use reqwest::{Url, IntoUrl, get};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let output_folder = Path::new(matches.value_of("output_folder").unwrap_or("/tmp/"));
    let feed = matches.value_of("feed").unwrap();

    let response = get(feed).unwrap();

    let reader = BufReader::new(response);
    let channel = Channel::read_from(reader).unwrap();
    for item in channel.items {
        println!("Episode: {}", item.title.unwrap());
        let url = item.enclosure.unwrap().url.into_url().unwrap();
        let cloned_url = url.clone();
        let filename = cloned_url.path_segments().unwrap().last().unwrap();
        println!("Downloading -> {}", filename);
        let mut response = get(url).unwrap();

        let path = Path::join(output_folder, Path::new(filename));
        let mut file = File::create(&path).unwrap();
        let mut buffer = Vec::new();
        response.read_to_end(&mut buffer).unwrap();
        file.write_all(&buffer).unwrap();
    }
}
