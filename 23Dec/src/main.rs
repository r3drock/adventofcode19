use nic::intcode;
use std::thread;

fn create_nics(count: usize, program: &Vec<isize>) -> Vec<intcode::NetworkAmplifier> {
    let mut nics = Vec::new();
    for _ in 0..count {
        nics.push(intcode::NetworkAmplifier::new(program.clone()));
    }
    nics
}

fn main() {
    const COUNT : usize = 50;
    let program = intcode::read_data("program");
    let mut nics = create_nics(COUNT, &program);
    let mut handles = vec![];
    let mut txs_vec  = vec![];
    for _ in 0..nics.len() {
        let mut txs = vec![];
        for nic in nics.iter() {
            txs.push(nic.get_input_transmitter());
        }
        txs_vec.push(txs);
    }
    for i in (0..nics.len()).rev() {
        nics[i].push_input((i as isize, -1));
    }

    for i in (0..nics.len()).rev() {
        let txs = txs_vec.pop().unwrap();

        let mut nic = nics.pop().unwrap();

        let thread_name = format!("{}", i);
        let handle =
            thread::Builder::new().name(thread_name).spawn(move || {
            //thread::sleep(Duration::from_secs(1));
            let destination_address = match nic.run_program_until_output(false) {
                None => return,
                Some(i) => i as usize,
            };
            let x = nic.run_program_until_output(false).unwrap();
            let y = nic.run_program_until_output(false).unwrap();
            if destination_address == 255 {
                println!("FINISHED {}", y);
                return;
            } else {
                println!("{} => {}: {} {}", i, destination_address, x, y);
                txs[destination_address].send((x,y)).unwrap();
            }
        });
        handles.push(handle);
    }
    for handle in handles  {
        let handle = match handle {
            Err(e) => panic!("{}", e),
            Ok(a) => a,
        };
        handle.join().unwrap();
    }
    println!("END");
}
