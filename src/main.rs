
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
}

impl VM {
	pub fn new(prog: Vec<InstructionSet>) -> VM {
		VM {
			ip : 0,
			program: prog,
		}
	}

	fn fetch(&mut self) -> InstructionSet {
		let instr = self.program[self.ip];
		self.ip += 1;
		instr
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
	let x = vm.fetch(); // PSH
	let y = vm.fetch(); // VAL(5)
}
