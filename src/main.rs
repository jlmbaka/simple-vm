//Allow Instruction set to be referred to without namespacing
use InstructionSet::{PSH, ADD, POP, SET, HLT, VAL};

#[derive(Copy, Clone)]
enum InstructionSet {
	PSH,
	ADD,
	POP,
	SET,
	HLT,
	VAL{value: isize},
}

struct VM {
	ip: usize,
	program: Vec<InstructionSet>,
	running: bool,
	sp: isize,
	stack: [InstructionSet; 256],
}

impl VM {
	pub fn new(prog: Vec<InstructionSet>) -> VM {
		VM {
			ip : 0,
			program: prog,
			running: true,			
			sp: -1,
			stack:[VAL{value:0}; 256],	
		}
	}

	fn fetch(&mut self) -> InstructionSet {
		let instr = self.program[self.ip];
		instr
	}

	fn eval(&mut self, instr: InstructionSet) {
		match instr {
			HLT => self.running = false,
			PSH => {
				self.sp += 1;
				self.ip += 1; // move to operand location
				self.stack[self.sp as usize] = self.program[self.ip];
				//[DEBUG]
				println!("PSH {0}", match self.program[self.ip] {
					VAL{value} => value, 
					_ => 0,});
			},
			POP => {
				let val_popped = match self.stack[self.sp as usize] {
					VAL{value} => value,
					_ => 0,
				};
				self.sp -= 1;
				println!("POP {0}", val_popped);
			},
			ADD => {
				// first we pop the stack and store it as a
				let a = match self.stack[self.sp as usize] {
					VAL{value} => value,
					_ => 0, 
				};
				self.sp -= 1;

				// we then pop the top of the stack and store it as b
				let b = match self.stack[self.sp as usize] {
					VAL{value} => value,
					_ => 0,
				};
				self.sp -= 1;

				//[DEBUG]
				println!("ADD {0}; {1}", a, b);

				// we then add the result and push it to the stack
				let result = b + a;
				self.sp += 1; // increment stack pointer **before**
				self.stack[self.sp as usize] = VAL{value: result}; // set the value to the top of the stack
			},
			_ => println!("[ERROR]: Unkown instruction"),
		}
	}
}

fn main() {
	let program = vec![
		PSH, VAL{value: 5},
		PSH, VAL{value: 6},
		ADD,
		POP,
		HLT,
	];

	let mut vm = VM::new(program);
	loop {
		let instr = vm.fetch();
		vm.eval(instr);
		vm.ip += 1;
		if !vm.running {
			break;
		}
	}
}
