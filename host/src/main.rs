use methods::METHOD_ELF;
use risc0_zkvm::{ExecutorEnv, ExecutorImpl};

fn main() {
    let env = ExecutorEnv::builder().build().unwrap();
    let mut exec = ExecutorImpl::from_elf(env, METHOD_ELF).unwrap();
    let session = exec.run().unwrap();
    println!("total cycles: {}", session.total_cycles);
}
