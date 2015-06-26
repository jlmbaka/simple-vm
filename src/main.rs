
#[derive(Copy, Clone)]
enum InstructionSet {
	PSH,
	ADD,
	POP,
	SET,
	HLT,
	VAL(i32),
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
			stack:[InstructionSet::VAL(0); 256],	
		}
	}

	fn fetch(&mut self) -> InstructionSet {
		let instr = self.program[self.ip];
		instr
	}

	fn eval(&mut self, instr: InstructionSet) {
		use InstructionSet::*;
		match instr {
			HLT => self.running = false,
			PSH => {
				self.sp += 1;
				self.ip += 1; // move to operand location
				self.stack[self.sp as usize] = self.program[self.ip];
			},
			_ => {},
		}
	}
}

fn main() {
	let program = vec![
		InstructionSet::PSH, InstructionSet::VAL(5),
		InstructionSet::PSH, InstructionSet::VAL(6),
		InstructionSet::ADD,
		InstructionSet::POP,
		InstructionSet::HLT,
	];

	let mut vm = VM::new(program);
	loop {
		vm.eval(vm.fetch());
		vm.ip += 1;
		if !vm.running {
			break;
		}
	}
}
