use colored::*;
use std::{thread, time, io};

fn main() {
	let mut board = [[Piece::None; 6]; 7];
	// board[0][5] = Piece::Red;
	// board[0][6] = Piece::Blue;


	// print_board(&board);

	let mut red_turn = true;
	let mut chosen = (8,8);
	let mut warning = "";
	let mut ghost_board = board.clone();
	'outer: loop {
		// print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
		print!("{esc}c", esc = 27 as char);

		println!("");

		println!(">> It's {} turn!", if red_turn {"Red's".bright_red()} else {"Blue's".bright_blue()}.bold());
		println!("\n {}", "1  2  3  4  5  6  7".white().underline());
		print_board(&board, chosen);
		println!("{}", warning.red());

		println!("{} ", "Select the column (1 to 7):");
		let mut column = String::new();
		io::stdin()
			.read_line(&mut column)
			.expect("Failed to read line");

		let column: u32 = match column.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				warning = "[Please enter a number]";
				continue 'outer;
			}
		};
		if column > 7 {
			warning = "[Number must be less than 7]";
			continue 'outer;
		};
		if board[(column - 1) as usize][0] != Piece::None {
			warning = "[This column is full!]";
			continue 'outer;
		};

		ghost_board = board.clone();
		for i in (0..board[(column - 1) as usize].len()).rev() {
			// println!("{:?}", board[(column - 1) as usize][i]);1
			match board[(column - 1) as usize][i] {
				Piece::None => {
					board[(column - 1) as usize][i] = if red_turn {
						Piece::Red
					} else {
						Piece::Blue
					};
					chosen = ((column - 1) as u8, i as u8);
					warning = "";
					let wait = time::Duration::from_millis(20);
					ghost_board[(column - 1) as usize][0 as usize] = if red_turn {
						Piece::Red
					} else {
						Piece::Blue
					};
					'animation: loop {
						print!("{esc}c", esc = 27 as char);
						thread::sleep(wait);

						'drop: for j in 0..board[(column - 1) as usize].len() {
							if j == chosen.1 as usize {
								break 'animation;
							}
							if ghost_board[(column - 1) as usize][j as usize] != Piece::None {
								ghost_board[(column - 1) as usize][j as usize] = Piece::None;
								ghost_board[(column - 1) as usize][(j+1) as usize] = if red_turn {
									Piece::Red
								} else {
									Piece::Blue
								};
								break 'drop;
							};
						};

						println!("\n>>\n\n {}", "1  2  3  4  5  6  7".white().underline());
						print_board(&ghost_board, chosen);
						// break 'animation;
					};
					break;
				},
				_ => ()
			};
		};

		red_turn = !red_turn;
	}
}

fn print_board(board: &[[Piece; 6]; 7], (x, y): (u8, u8) ) {
	for i in 0..board[0].len() {
		let mut j = 0;
		for column in board {
			// print!("{} ", column[i]);
			if x == (j as u8) && y == (i as u8) {//&& (column[i] != Piece::None || y != 0)   {
				print!("{0}{1}{0}", " ".on_yellow(), piece_to_string(column[i as usize]).on_yellow());
			} else {
				print!("{0}{1}{0}", " ", piece_to_string(column[i as usize]));
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

fn check_board(board: &[[Piece; 6]; 7]) {

}