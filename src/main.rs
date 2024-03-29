use colored::*;
use std::{thread, time, io};

fn main() {
	let debug = false;

	let mut board = [[Piece::None; 6]; 7];
	let mut red_turn = true;
	let mut chosen = (8,8);
	let mut warning = "";
	let mut ghost_board: [[Piece; 6]; 7];
	let mut pieces = 0;
	'outer: loop {
		if !debug {
			print!("{esc}c", esc = 27 as char);
		};

		println!();

		println!(">> It's {} turn!", if red_turn {"Red's".bright_red()} else {"Blue's".bright_blue()}.bold());
		println!("\n {}", "1  2  3  4  5  6  7".white().underline());
		print_board(&board, chosen);
		println!("{}", warning.red());

		println!("Select the column (1 to 7):");
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

		ghost_board = board;
		for i in (0..board[(column - 1) as usize].len()).rev() {
			if board[(column - 1) as usize][i] == Piece::None {
				board[(column - 1) as usize][i] = if red_turn {
					Piece::Red
				} else {
					Piece::Blue
				};
				chosen = ((column - 1) as u8, i as u8);
				pieces += 1;
				warning = "";
				let wait = time::Duration::from_millis(20);
				ghost_board[(column - 1) as usize][0_usize] = if red_turn {
					Piece::Red
				} else {
					Piece::Blue
				};
				'animation: loop {
					if !debug {
						print!("{esc}c", esc = 27 as char);
					};
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
			};
		};
		let winning = check_board(&board, chosen);
		println!("{:?}", winning);
		if !winning.is_empty() {
			if !debug {
				print!("{esc}c", esc = 27 as char);
			};
			println!();
			println!(">> {} won the game!", if red_turn {"Red".bright_red()} else {"Blue".bright_blue()}.bold());
			println!("\n {}", "1  2  3  4  5  6  7".white().underline());
			print_winning_board(&ghost_board, &winning);
			println!();

			break 'outer;
		}

		if pieces == 42 {
			if !debug {
				print!("{esc}c", esc = 27 as char);
			};
			println!();
			println!(">> [NOBODY] won the game!");
			println!("\n {}", "1  2  3  4  5  6  7".white().underline());
			print_winning_board(&ghost_board, &winning);
			println!();

			break 'outer;
		}

		red_turn = !red_turn;
	}
}

fn print_board(board: &[[Piece; 6]; 7], (x, y): (u8, u8) ) {
	for i in 0..board[0].len() {
		for (j, column) in board.iter().enumerate() {
			// print!("{} ", column[i]);
			if x == (j as u8) && y == (i as u8) {//&& (column[i] != Piece::None || y != 0)   {
				print!("{0}{1}{0}", " ".on_yellow(), piece_to_string(column[i as usize]).on_yellow());
			} else {
				print!(" {} ", piece_to_string(column[i as usize]));
			}
		}
		println!();
	}
}

fn print_winning_board(board: &[[Piece; 6]; 7], pieces: &Vec<(u8, u8)>) {
	for i in 0..board[0].len() {
		for (j, column) in board.iter().enumerate() {
			let mut highlighted = false;
			for (x, y) in pieces {
				if x == &(j as u8) && y == &(i as u8) {
					highlighted = true;
				}
			}
			if highlighted {//&& (column[i] != Piece::None || y != 0)   {
				print!("{0}{1}{0}", " ".on_yellow(), piece_to_string(column[i as usize]).on_yellow());
			} else {
				print!(" {} ", piece_to_string(column[i as usize]));
			}
		}
		println!();
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

fn check_board(board: &[[Piece; 6]; 7], (x, y): (u8, u8)) -> Vec<(u8, u8)> {
	let mut pieces: Vec<(u8, u8)> = Vec::new();
	let team = board[x as usize][y as usize];
	let mut iter: i8 = 0;

	let mut horizontal: u8 = 0;
	let mut horizontal_pieces: Vec<(u8, u8)> = Vec::new();
	loop {
		if x as i8 + iter > board.len() as i8 - 1 {
			horizontal += iter as u8;
			break;
		}
		if board[(x as i8 + iter) as usize][y as usize] != team {
			horizontal += iter as u8;
			break;
		}
		horizontal_pieces.push((x + iter as u8, y));
		iter += 1;
	}
	iter = 0;
	loop {
		if x as i8 + iter < 0 {
			horizontal += iter.unsigned_abs();
			break;
		}
		if board[(x as i8 + iter) as usize][y as usize] != team {
			horizontal += iter.unsigned_abs();
			break;
		}
		horizontal_pieces.push((x - iter.unsigned_abs(), y));
		iter -= 1;
	}
	iter = 0;
	println!("{}", horizontal);

	let mut vertical: u8 = 1;
	let mut vertical_pieces: Vec<(u8, u8)> = Vec::new();
	loop {
		if y as i8 + iter > board[x as usize].len() as i8 - 1 {
			vertical += iter as u8;
			break;
		}
		if board[x as usize][(y as i8 + iter) as usize] != team {
			vertical += iter as u8;
			break;
		}
		vertical_pieces.push((x, y + iter as u8));
		iter += 1;
	}
	iter = 0;
	println!("{}", vertical);

	let mut diagonal_down: u8 = 0;
	let mut diagonal_down_pieces: Vec<(u8, u8)> = Vec::new();
	loop {
		if x as i8 + iter > board.len() as i8 - 1 || y as i8 + iter > board[x as usize].len() as i8 - 1 {
			diagonal_down += iter as u8;
			break;
		}
		if board[(x as i8 + iter) as usize][(y as i8 + iter) as usize] != team {
			diagonal_down += iter as u8;
			break;
		}
		diagonal_down_pieces.push((x + iter as u8, y + iter as u8));
		iter += 1;
	}
	iter = 0;
	loop {
		if x as i8 + iter < 0 || y as i8 + iter < 0 {
			diagonal_down += iter.unsigned_abs();
			break;
		}
		if board[(x as i8 + iter) as usize][(y as i8 + iter) as usize] != team {
			diagonal_down += iter.unsigned_abs();
			break;
		}
		diagonal_down_pieces.push((x - iter.unsigned_abs(), y - iter.unsigned_abs()));
		iter -= 1;
	}
	iter = 0;
	println!("{}", diagonal_down);

	let mut diagonal_up: u8 = 0;
	let mut diagonal_up_pieces: Vec<(u8, u8)> = Vec::new();
	loop {
		if x as i8 + iter > board.len() as i8 - 1 || y as i8 - iter < 0 {
			diagonal_up += iter as u8;
			break;
		}
		if board[(x as i8 + iter) as usize][(y as i8 - iter) as usize] != team {
			diagonal_up += iter as u8;
			break;
		}
		diagonal_up_pieces.push((x + iter as u8, y - iter.unsigned_abs()));
		iter += 1;
	}
	iter = 0;
	loop {
		if x as i8 + iter < 0 || y as i8 - iter > board[x as usize].len() as i8 - 1 {
			diagonal_up += iter.unsigned_abs();
			break;
		}
		if board[(x as i8 + iter) as usize][(y as i8 - iter) as usize] != team {
			diagonal_up += iter.unsigned_abs();
			break;
		}
		diagonal_up_pieces.push((x - iter.unsigned_abs(), y + iter.unsigned_abs()));
		iter -= 1;
	}
	println!("{}", diagonal_up);

	if horizontal > 4 {	
		pieces.append(&mut horizontal_pieces);
	}
	if vertical > 4 {	
		pieces.append(&mut vertical_pieces);
	}
	if diagonal_down > 4 {	
		pieces.append(&mut diagonal_down_pieces);
	}
	if diagonal_up > 4 {	
		pieces.append(&mut diagonal_up_pieces);
	}


	pieces
}