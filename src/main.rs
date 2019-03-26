//Author: Everett Sullivan.
//Date Created: 3/18/2019
//Date last modified: 3/19/2019

use std::io;
use std::io::Write; //bring flush into scope
use std::io::Read;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str;

use matrix;
use matrix::Matrix;

const QUIT: i32 = 1;
const HELP: i32 = 2;
const ENCRYPTFILE: i32 = 3;
const ENCRYPTTEXT: i32 = 4;

fn byte_to_num(byte: u8) -> u32 {
	match byte as u32{
		10 => 0,
        13 => 1,
		i => i-30,
        _ => 1000,
	}
}

fn num_to_byte(num: u32) -> u8 {
	match num{
		0 => 10 as u8,
        1 => 13 as u8,
		i => (i+30) as u8,
        _ => 0 as u8,
	}
}

fn print_help() {
    println!("Usefull program info will be printed here.");
	println!("Press enter to continue");
	io::stdin().read_line(&mut String::new()).unwrap();
}

fn encrypt_file() {
    println!("Encrypting a file will happen here.");
	println!("Press enter to continue");
	io::stdin().read_line(&mut String::new()).unwrap();
	
	let mut file_to_encrypt = String::new();
	let mut input = String::new();
	let mut matrix_size: u32 = 0;
	
	print!("Enter file name to be encrypted:   ");
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut input).expect("Failed to read line");

	file_to_encrypt = match input.trim().parse() {
		Ok(file) => file,
		Err(_) => {println!("Invalid input 1"); "blah".to_string()},
	};
	
	input = String::new(); //result input, or old data will linger.
	
	print!("Enter size of encrypt matrix:   ");
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut input).expect("Failed to read line");

	matrix_size = match input.trim().parse() {
		Ok(num) => num,
		Err(e) => {println!("Invalid input 2 {}", e); 3},
	};
	
	let mut temp = vec![vec![0; matrix_size as usize]; matrix_size as usize];
	
	for i in 0..matrix_size as usize {
	    for j in 0..matrix_size as usize {
		    input = String::new();
		    print!("Enter matrix entries:   ");
			io::stdout().flush().unwrap();
			io::stdin().read_line(&mut input).expect("Failed to read line");

			temp[i][j] = match input.trim().parse() {
				Ok(num) => num,
				Err(_) => {println!("Invalid input 3"); 0},
			};
		}
	}
	
	let encryption_matrix = Matrix::init_matrix(matrix_size as u32,matrix_size as u32,95,temp);
	encryption_matrix.print_me();
	
	////////////////////Read file
	
	let file = File::open(file_to_encrypt).unwrap();
	let mut buf_reader = BufReader::new(file);
	let mut contents = String::new();
	buf_reader.read_to_string(&mut contents).unwrap();
	let mut output: Vec<u8> = Vec::new();
	
	let mut current_chars = 0;
	let mut encrypt_vector = Matrix::init_matrix(matrix_size as u32,1,95,vec![vec![0]; matrix_size as usize]);
	
	for c in contents.bytes() {
	    if (c >= 32) & (c <= 126) {
		    encrypt_vector.set_entry(current_chars, 0, (c-32) as i32);
			current_chars = current_chars + 1;
		}
		
		if current_chars == matrix_size as usize {
		    encrypt_vector = Matrix::matrix_mult(&encryption_matrix,&encrypt_vector);
			for i in 0..matrix_size {
			    output.push((encrypt_vector.get_entry(i as usize,0)+32) as u8);
			}
			current_chars = 0;
		}
	}
	
	if current_chars != 0 {
	    for i in current_chars..matrix_size as usize {
		    encrypt_vector.set_entry(i, 0, 0);
		}
		
		encrypt_vector = Matrix::matrix_mult(&encryption_matrix,&encrypt_vector);
			for i in 0..matrix_size {
			    output.push((encrypt_vector.get_entry(i as usize,0)+32) as u8);
			}
		current_chars = 0;
	}
	
	let new_output = str::from_utf8(&output).unwrap();
	
	let mut f = File::create("encrypted.txt").expect("Unable to create file");
    f.write_all(new_output.as_bytes()).expect("Unable to write data");
	
	Matrix::matrix_invert(&encryption_matrix).print_me();
	
	
}

fn encrypt_text() {
    println!("Encrypting text will happen here.");
	println!("Press enter to continue");
	io::stdin().read_line(&mut String::new()).unwrap();
}

fn main() {
	
	let matrix5 = Matrix::init_matrix(3,3,10,vec![vec![1,3,0],vec![0,3,0],vec![2,6,1]]);
	matrix5.print_me();
	println!("{}", matrix5.det());
	println!("Matrix is invertable: {}", matrix5.is_invertable());
	let matrix6 = Matrix::matrix_invert(&matrix5);
	matrix6.print_me();
	Matrix::matrix_mult(&matrix5,&matrix6).print_me();
	
	let mut state = 0;
	let mut valid = false;
	
	while state != QUIT{
	    println!("Welcome to Everett's Hill Cipher program!");
	    println!("1) Quit");
	    println!("2) Print help");
	    println!("3) Encrypt / decrypt file");
	    println!("4) Encrypt / decrypt text");
		
		while !valid {
		
		    let mut input = String::new(); //need this inside the loop to not get repeated invalid input. why?
		
			print!("Enter your choice:   ");
		    io::stdout().flush().unwrap();
		    io::stdin().read_line(&mut input).expect("Failed to read line");
		
		    state = match input.trim().parse() {
		        Ok(num) => num,
		        Err(_) => {println!("Invalid input");continue},
		    };
			
			valid = true;
			match state{
			    QUIT => {state = QUIT;},
                HELP => print_help(),
                ENCRYPTFILE => encrypt_file(),
                ENCRYPTTEXT => encrypt_text(),
                _ => {println!("Options must be 1, 2,3 or 4.");valid = false;continue},
			}
		
		}
		valid = false;
	}
	
	let file = File::open("test.txt").unwrap();
	let mut buf_reader = BufReader::new(file);
	let mut contents = String::new();
	buf_reader.read_to_string(&mut contents).unwrap();
	
	for c in contents.bytes() {
	    println!("{}", c);
	}
	
	//for line in buf_reader.lines() {
	//    let line = line.expect("Unable to read line");
	//	for c in line.chars() {
	//	    println!("{}", c);
	//	}
	//}
	//let mut contents = String::new();
	//buf_reader.read_to_string(&mut contents).unwrap();
	//println!("{}", contents);
}
