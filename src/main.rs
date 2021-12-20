use colored::*;
use std::io;

fn main() {
	let mut board = [[Piece::None; 7]; 7];
	board[0][5] = Piece::Red;
	board[0][6] = Piece::Blue;

	// print_board(&board);

	let mut red_turn = true;
	loop {
		// print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
		print!("{esc}c", esc = 27 as char);

		println!("");

		if red_turn {
			println!("It's {} turn!", "Red's".bold().bright_red());
		} else {
			println!("It's {} turn!", "Blue's".bold().bright_blue());
		}
		println!("{}", "1 2 3 4 5 6 7".white().underline());
		print_board(&board);

		println!("{} ", "Select the collumn (1 to 7):");
		let mut collumn = String::new();
		io::stdin()
			.read_line(&mut collumn)
			.expect("Failed to read line");

		let collumn: u32 = match collumn.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("{}", "\nThat is not a valid collumn\n".bold().underline().bright_yellow());
				continue;
			}
		};
		if collumn > 7 {
			println!("{}", "\nThat is not a valid collumn\n".bold().underline().bright_yellow());
			continue;
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
					break;
				},
				_ => ()
			}
		};

		red_turn = !red_turn;
	}
}

fn print_board(board: &[[Piece; 7]; 7]) {
	for i in 0..board[0].len() {
		for collumn in board {
			// print!("{} ", collumn[i]);
			print!("{} ", piece_to_string(collumn[i]));
		}
		print!("\n");
	}
}

#[derive(Copy, Clone, Debug)]
enum Piece {
	None,
	Red,
	Blue
}

fn piece_to_string(piece: Piece) -> String {
	match piece {
		Piece::None => "|".white().to_string(),
		Piece::Red => "●".bright_red().to_string(),
		Piece::Blue => "●".bright_blue().to_string(),
	}
}