pub struct Matrix {
    pub rows: u32,
	pub columns: u32,
	pub modulus: i32,
	matrix: Vec<Vec<i32>>
}

impl Matrix{

    pub fn init_null_matrix(my_rows: u32, my_columns: u32, my_modulus: i32) -> Matrix{
       Matrix{rows: my_rows, columns: my_columns, modulus: my_modulus, matrix: vec![vec![0; my_columns as usize]; my_rows as usize]}
    }
	
	pub fn init_matrix(my_rows: u32, my_columns: u32, my_modulus: i32, my_matrix: Vec<Vec<i32>>) -> Matrix{
       Matrix{rows: my_rows, columns: my_columns, modulus: my_modulus, matrix: my_matrix}
    }

    pub fn init_null_square_matrix(my_size: u32, my_modulus: i32) -> Matrix{
       Matrix{rows: my_size, columns: my_size, modulus: my_modulus, matrix: vec![vec![0; my_size as usize]; my_size as usize]}
    }
	
	pub fn get_entry(&self, x: usize, y:usize) -> i32{
	    match self.matrix.get(x) {
		    Some(row) => {
			    match row.get(y) {
				    Some(column) => {return *column}
					None => {println!("The column doesn't exist"); 100},
				}
			}
			None => {println!("The row doesn't exist"); 100},
		}
	}
	
	pub fn set_entry(&mut self, x: usize, y:usize, new_value: i32){
	    match self.matrix.get(x) {
		    Some(row) => {
			    match row.get(y) {
				    Some(_column) => {self.matrix[x][y] = new_value}
					None => println!("The column doesn't exist"),
				}
			}
			None => println!("The row doesn't exist"),
		}
	}
	
	pub fn row_swap(&mut self, row1: usize, row2: usize){
	    match self.matrix.get(row1) {
		    Some(_row) => {
			    match self.matrix.get(row2) {
		            Some(_other_row) => {
					    for entry in 0..self.columns{
						    let temp = self.matrix[row1][entry as usize];
				            self.matrix[row1][entry as usize] = self.matrix[row2][entry as usize];
							self.matrix[row2][entry as usize] = temp;
				        }
			        }
			        None => println!("This row doesn't exist"),
		        }
			}
			None => println!("This row doesn't exist"),
		}
	}
	
	pub fn row_mult(&mut self, row: usize, mult_value: i32){
	    match self.matrix.get(row) {
		    Some(_row) => {
			    for entry in 0..self.columns{
				    self.matrix[row][entry as usize] = true_mod(self.matrix[row][entry as usize]*mult_value,self.modulus);
				}
			}
			None => println!("This row doesn't exist"),
		}
	}
	
	pub fn add_mult_of_row(&mut self, base_row: usize, aux_row: usize, mult_value: i32){
	    match self.matrix.get(base_row) {
		    Some(_row) => {
			    match self.matrix.get(aux_row) {
		            Some(_other_row) => {
					    for entry in 0..self.columns{
				            self.matrix[base_row][entry as usize] = true_mod(self.matrix[base_row][entry as usize] + self.matrix[aux_row][entry as usize]*mult_value,self.modulus);
				        }
			        }
			        None => println!("This row doesn't exist"),
		        }
			}
			None => println!("This row doesn't exist"),
		}
	}
	
	pub fn print_me(&self){
		for row in self.matrix.iter(){
		    for entry in row.iter(){
			    print!("{} ", entry);
			}
			print!("\n");
		}
	}
	
	pub fn get_minor(&self, x: usize, y:usize) -> Result<Matrix, &'static str>{
	    match self.matrix.get(x) {
		    Some(row) => {
			    match row.get(y) {
				    Some(_column) => {
					    let mut minor_matrix = Matrix{rows: self.rows-1, columns: self.columns-1, modulus: self.modulus, matrix: vec![vec![0; (self.rows-1) as usize]; (self.columns-1) as usize]};
						for row in 0..(self.rows-1){ //vectors are index by 0 so we must adjust
							for column in 0..(self.columns-1){ //a..b is [a,b)
							    let mut my_x = row;
								let mut my_y = column;
								if row >= (x as u32) {
									my_x = my_x + 1;
								}
								if column >= (y as u32) {
									my_y = my_y + 1;
								}
								minor_matrix.set_entry(row as usize,column as usize, self.matrix[my_x as usize][my_y as usize]);
							}
						}
						Ok(minor_matrix)
					}
					None => return Err("The column doesn't exist, balh"),
				}
			}
			None => return Err("The row doesn't exist"),
		}
	}
	
	pub fn det(&self) -> i32{
	    let mut result = 0;
	    if self.rows == 2{
		    result = self.matrix[0][0]*self.matrix[1][1] - self.matrix[0][1]*self.matrix[1][0];
		} else if self.rows == 1 {
		    result = self.matrix[0][0];
		} else {
		    for i in 0..self.rows {
			    let minor = self.get_minor(0,i as usize).unwrap();
			    if(i % 2) == 0{
				    result = result + self.matrix[0][i as usize]*minor.det();
				} else{
			        result = result - self.matrix[0][i as usize]*minor.det();
				}
			}
		}
		true_mod(result,self.modulus)
	}
	
	pub fn is_invertable(&self) -> bool {
	    GCD(self.modulus as u32,self.det() as u32) == 1
	}
	
	pub fn matrix_mult(left_matrix: &Matrix,right_matrix: &Matrix) -> Matrix{
	    let my_rows = left_matrix.rows;
		let my_cols = right_matrix.columns;
	    let mut result_matrix = Matrix{rows: my_rows, columns: my_cols, modulus: left_matrix.modulus, matrix: vec![vec![0; my_cols as usize]; my_rows as usize]};
		for row in 0..my_rows{ //vectors are index by 0 so we must adjust
	        for column in 0..my_cols{ //a..b is [a,b)
	            let mut temp = 0;
				for number in 0..left_matrix.columns{
	               temp = temp + left_matrix.get_entry(row as usize,number as usize)*right_matrix.get_entry(number as usize,column as usize);
	            }
				result_matrix.set_entry(row as usize,column as usize,true_mod(temp, result_matrix.modulus));
	        }
	    }
		return result_matrix; //state thing to return
	}
	
	pub fn matrix_invert(target_matrix: &Matrix) -> Matrix{
	    let my_size = target_matrix.rows; //set up matrix for Guassian elimination.
		let my_mod = target_matrix.modulus;
	    let mut guass_matrix = Matrix{rows: my_size, columns: 2*my_size, modulus: target_matrix.modulus, matrix: vec![vec![0; (my_size*2) as usize]; my_size as usize]};
		for row in 0..my_size{
	        for column in 0..my_size{
	            guass_matrix.matrix[row as usize][column as usize] = target_matrix.matrix[row as usize][column as usize];
	        }
			guass_matrix.matrix[row as usize][(my_size + row) as usize] = 1;
	    }
		for column in 0..my_size{ //preform Guassian elimination.
			//if our current line has a zero in the given spot switch it with a row that does not
			let current_entry = guass_matrix.matrix[column as usize][column as usize];
			if (current_entry == 0) | (GCD(current_entry as u32,my_mod as u32) != 1){ //only number relatively prime to the modulus can be inverted.
			    let mut found = false;
				let mut i = column;
			    while (i < my_size) & (found == false){
				    i = i + 1;
				    if (guass_matrix.matrix[i as usize][column as usize] != 0) & (GCD(guass_matrix.matrix[i as usize][column as usize] as u32,my_mod as u32) == 1){
					    found = true;
					}
				}
				if found == false{
				    println!("Can not be inverted");
				    return guass_matrix;
				}
			}
			let inverse_element = modulo_inverse(current_entry,my_mod);
			guass_matrix.row_mult(column as usize,inverse_element);
			for row in 0..my_size{
			    //we don't need to change the current row are rows that already have a zero in the current column
				if (row != column) & (guass_matrix.matrix[row as usize][column as usize] != 0){
			        guass_matrix.add_mult_of_row(row as usize,column as usize,-guass_matrix.matrix[row as usize][column as usize]);
				}
			}
		}
		//Extract the inverse from the right side of the guassian elimination matrix.
		let mut result_matrix = Matrix{rows: my_size, columns: my_size, modulus: target_matrix.modulus, matrix: vec![vec![0; my_size as usize]; my_size as usize]};
		for row in 0..my_size{
	        for column in 0..my_size{
	            result_matrix.matrix[row as usize][column as usize] = guass_matrix.matrix[row as usize][(my_size + column) as usize];
	        }
	    }
		return result_matrix;
	}
}

//true_mod
//Purpose:
//    Given an intger num, computes the mathematical result num mod n
//Pre-conditions:
//    the modulus is a positive integer greater than one.
pub fn true_mod(num: i32, modulus: i32) -> i32{
	let mut my_num = num % modulus;
	if my_num < 0{
	    my_num = my_num + modulus;
	}
	my_num
}

//modulo_inverse
//Purpose:
//    Given an intger i in the group of integers mod n for some n
//    returns the inverse of i in mod n if it exists.
//Pre-conditions:
//    the modulus is a positive integer greater than one.
//    the number (num) to be inverted is not zero.
pub fn modulo_inverse(num: i32, modulus: i32) -> i32{
    if (modulus < 2) | (num % modulus == 0){
	    return 0;
	}
    let mut first_num = modulus;
	let mut second_num = num % modulus;
	let mut remainder;
	let mut quotient;
	let mut first_inverse = 0;
	let mut second_inverse = 1;
	remainder = first_num % second_num;
	quotient = first_num / second_num;
	while remainder != 0{
	    first_num = second_num;
        second_num = remainder;
		let temp = first_inverse - quotient*second_inverse;
		first_inverse = second_inverse;
        second_inverse = temp;
		remainder = first_num % second_num;
		quotient = first_num / second_num;
	}
	true_mod(second_inverse,modulus)
}

//GCD
//Purpose:
//    Given two positive numbers returns their gcd.
//Pre-conditions:
//    Both numbers x and y are positive.
pub fn GCD(x: u32, y: u32) -> u32{
    let mut first_num;
	let mut second_num;
	let mut remainder;
	if x > y{
	    first_num = x;
	    second_num = y;
	} else {
	    first_num = y;
	    second_num = x;
	}
	remainder = first_num % second_num;
	while remainder != 0{
	    first_num = second_num;
        second_num = remainder;
		remainder = first_num % second_num;
	}
	second_num
}