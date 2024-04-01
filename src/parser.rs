use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

use crate::{ParsedGame, Player};

fn insert_country_data(buffer: &Vec<String>, game_data: &mut ParsedGame) {
	// strips the whitespace from the raw input lines and groups each line in tags and igns
	// TODO: figure out why this can't be a oneliner in it's current form
	let stripped_buf: Vec<String> = buffer.into_iter().map(|x| x.trim().to_string()).collect();
	let chunked_buf = stripped_buf.chunks(2);

	for chunk in chunked_buf {
        game_data.players.push(Player {
            ign: chunk[0][1..chunk[0].len() - 1].to_string(),
            tag: chunk[1].to_string(),
            score: 0,
        });
	}
}

fn insert_score_data(buffer: &Vec<String>, game_data: &mut ParsedGame) {
	let mut iter = buffer.iter();

	while let Some(line) = iter.next() {
		if !line.contains("name") {
			continue;
		};

		for player in &mut game_data.players {
			if !line.contains(&player.tag) {
				continue;
			};

			// required because next line is blank
			iter.next();

			let score_sheet = if let Some(score_sheet) = iter.next() {
				score_sheet
			} else {
				panic!("Score sheet not available")
			};

			if score_sheet.contains("{") {
				player.score = 0;
				continue;
			}

			let mut scores_split = score_sheet.split(" ");
			let score_and_date = scores_split.nth(scores_split.clone().count() - 2).unwrap();

			let sd_split_point = score_and_date.find("=").unwrap();
			let score = &score_and_date[(sd_split_point + 1)..score_and_date.len()]
				.parse::<u32>()
				.unwrap();

			player.score = score.clone();
		}
	}
}

pub fn parse(filepath: &PathBuf, game_data: &mut ParsedGame) {
	let lines = read_lines(filepath).expect("lines extracted from file");

	let mut reading_player_countries = false;
	let mut player_countries_buf: Vec<String> = Vec::new();

	let mut reading_player_scores = false;
	let mut player_scores_buf: Vec<String> = Vec::new();

	for line in lines {
		if let Ok(ip) = line {
			if ip.contains("date") && game_data.date.is_empty() {
				let date_start = ip.find('=').unwrap_or(0);
				let date_end = ip.find('.').unwrap_or(0);
				game_data.date = ip[(date_start + 1)..date_end].to_string();
			}

			if reading_player_countries {
				if ip.contains("}") && (ip.chars().count() == 1) {
					reading_player_countries = false;
					insert_country_data(&player_countries_buf, game_data);
					continue;
				}

				player_countries_buf.push(ip.clone());
			}

			if ip.contains("players_countries") {
				reading_player_countries = true;
			}

			if reading_player_scores {
				if ip.contains("}") && (ip.chars().count() == 1) {
					reading_player_scores = false;
					insert_score_data(&player_scores_buf, game_data);
					continue;
				}

				player_scores_buf.push(ip.clone());
			}

			if ip.contains("score_statistics") {
				reading_player_scores = true;
			}
		}
	}
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
	P: AsRef<Path>,
{
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}
