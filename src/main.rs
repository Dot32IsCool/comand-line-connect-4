use colored::*;
use std::io;

fn main() {
	let mut board = [[Piece::None; 7]; 7];
	// board[0][5] = Piece::Red;
	// board[0][6] = Piece::Blue;


	// print_board(&board);

	let mut red_turn = true;
	let mut chosen = (0,0);
	let mut warning = "";
	'outer: loop {
		// print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
		print!("{esc}c", esc = 27 as char);

		println!("");

		println!(">> It's {} turn!", if red_turn {"Red's".bright_red()} else {"Blue's".bright_blue()}.bold());
		println!("\n {}", "1  2  3  4  5  6  7".white().underline());
		print_board(&board, chosen);
		println!("{}", warning.red());

		println!("{} ", "Select the collumn (1 to 7):");
		let mut collumn = String::new();
		io::stdin()
			.read_line(&mut collumn)
			.expect("Failed to read line");

		let collumn: u32 = match collumn.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				warning = "[Please enter a number]";
				continue 'outer;
			}
		};
		if collumn > 7 {
			warning = "[Number must be less than 7]";
			continue 'outer;
		}

		for i in (0..board[(collumn - 1) as usize].len()).rev() {
			// println!("{:?}", board[(collumn - 1) as usize][i]);1
			
			match board[(collumn - 1) as usize][i] {
				Piece::None => {
					board[(collumn - 1) as usize][i] = if red_turn {
						Piece::Red
					} else {
						Piece::Blue
					};
					chosen = ((collumn - 1) as u8, i as u8);
					warning = "";
					break;
				},
				_ => ()
			}
		};

		red_turn = !red_turn;
	}
}

fn print_board(board: &[[Piece; 7]; 7], (x, y): (u8, u8) ) {
	for i in 0..board[0].len() {
		let mut j = 0;
		for collumn in board {
			// print!("{} ", collumn[i]);
			if x == (j as u8) && y == (i as u8) && (collumn[i] != Piece::None)   {
				print!("{}{}{}", " ".on_yellow(), piece_to_string(collumn[i as usize]).on_yellow(), " ".on_yellow());
			} else {
				print!("{}{}{}", " ".white(), piece_to_string(collumn[i as usize]), " ".white());
			}
			j += 1;
		}
		print!("\n");
	}
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Piece {
	None,
	Red,
	Blue
}

fn piece_to_string(piece: Piece) -> String {
	match piece {
		Piece::None => "◦".white().dimmed().to_string(),
		Piece::Red => "●".bright_red().to_string(),
		Piece::Blue => "●".bright_blue().to_string(),
	}
}