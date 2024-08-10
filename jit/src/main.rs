use libc::{
    mmap, mprotect, munmap, MAP_ANON, MAP_JIT, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE,
};
use std::{collections::HashMap, mem, time::Instant};

const PAGE_SIZE: usize = 4096;

enum Instruction {
    Add(i32, i32),
}

struct Interpreter {
    code: Vec<Instruction>,
    execution_count: HashMap<usize, u32>,
    jit_cache: HashMap<usize, (*mut u8, usize)>,
}
impl Interpreter {
    fn new(code: Vec<Instruction>) -> Self {
        Interpreter {
            code,
            execution_count: HashMap::new(),
            jit_cache: HashMap::new(),
        }
    }

    fn execute(&mut self, pc: usize, a: i32, b: i32) -> i32 {
        *self.execution_count.entry(pc).or_insert(0) += 1;

        if let Some(&(jit_fn, _)) = self.jit_cache.get(&pc) {
            println!("Executing JIT compiled code at pc {}", pc);
            unsafe {
                let func: fn(i32, i32) -> i32 = mem::transmute(jit_fn);
                return func(a, b);
            }
        }

        let result = match &self.code[pc] {
            Instruction::Add(x, y) => a + b + x + y,
        };

        if self.execution_count[&pc] > 10 {
            println!("JIT compiling instruction at pc {}", pc);
            let (jit_fn, size) = self.jit_compile(pc);
            self.jit_cache.insert(pc, (jit_fn, size));
        }

        result
    }

    fn jit_compile(&self, pc: usize) -> (*mut u8, usize) {
        let (machine_code, code_len) = match &self.code[pc] {
            Instruction::Add(x, y) => {
                let mut code = vec![
                    0x20, 0x00, 0x00, 0x8B, // ADD X0, X0, X1
                    0x00, 0x04, 0x00, 0x91, // ADD X0, X0, #1 (placeholder for x)
                    0x00, 0x08, 0x00, 0x91, // ADD X0, X0, #2 (placeholder for y)
                    0xC0, 0x03, 0x5F, 0xD6, // RET
                ];

                // Encode immediate values (12-bit, shifted left by 10)
                let imm_x = ((*x as u32) & 0xFFF) << 10;
                let imm_y = ((*y as u32) & 0xFFF) << 10;
                code[4..8].copy_from_slice(&(0x91000000 | imm_x).to_le_bytes());
                code[8..12].copy_from_slice(&(0x91000000 | imm_y).to_le_bytes());

                (code, 16)
            }
        };

        let alloc_size = PAGE_SIZE;

        println!("Machine code: {:02X?}", machine_code);

        unsafe {
            let ptr = mmap(
                std::ptr::null_mut(),
                alloc_size,
                PROT_WRITE | PROT_READ,
                MAP_PRIVATE | MAP_ANON | MAP_JIT,
                -1,
                0,
            ) as *mut u8;

            if ptr == libc::MAP_FAILED as *mut u8 {
                panic!("mmap failed: {}", std::io::Error::last_os_error());
            }

            std::ptr::copy_nonoverlapping(machine_code.as_ptr(), ptr, code_len);

            // Now make the memory executable
            if mprotect(ptr as *mut libc::c_void, alloc_size, PROT_READ | PROT_EXEC) != 0 {
                panic!("mprotect failed: {}", std::io::Error::last_os_error());
            }

            println!("Memory protection changed successfully");
            (ptr, alloc_size)
        }
    }
}

impl Drop for Interpreter {
    fn drop(&mut self) {
        for &(ptr, size) in self.jit_cache.values() {
            unsafe {
                munmap(ptr as *mut _, size);
            }
        }
    }
}

fn main() {
    let code = vec![Instruction::Add(1, 2)];
    let mut interpreter = Interpreter::new(code);

    for i in 0..20 {
        println!("Iteration {}", i);

        let start = Instant::now();
        let result = interpreter.execute(0, i, 2);
        let duration = start.elapsed();

        println!("Result: {}, Time: {:?}", result, duration);
    }
}
