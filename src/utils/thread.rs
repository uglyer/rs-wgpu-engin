use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

pub fn parallel<F>(start: usize, stop: usize, fnc: F)
where
    F: Fn(usize) + Send + Sync + 'static,
{
    let stop_signal = Arc::new(AtomicBool::new(false));
    parallel_with_signal(stop_signal, start, stop, fnc)
}

pub fn parallel_with_signal<F>(stop_signal: Arc<AtomicBool>, start: usize, stop: usize, fnc: F)
where
    F: Fn(usize) + Send + Sync + 'static,
{
    let count = stop - start;
    if count < 1 {
        return;
    }

    let fnc = Arc::new(fnc);
    let c = Arc::new(Mutex::new((start..stop).collect::<Vec<_>>()));
    let procs = num_cpus::get() * 2;
    let num_threads = procs.min(count);

    let mut handles = Vec::new();

    for _ in 0..num_threads {
        let c = Arc::clone(&c);
        let fnc = Arc::clone(&fnc);
        let stop_signal = Arc::clone(&stop_signal);

        let handle = thread::spawn(move || {
            while !stop_signal.load(Ordering::Relaxed) {
                if let Some(i) = c.lock().unwrap().pop() {
                    fnc(i);
                } else {
                    break; // No more items to process
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_parallel() {
    let mut hash_set: HashSet<usize> = HashSet::new();
    for i in 0..100000 {
        hash_set.insert(i);
    }
    let shared_set = Arc::new(Mutex::new(hash_set));
    let shared_set_clone = Arc::clone(&shared_set);
    parallel(0,100000,move|i|{
        let mut set = shared_set_clone.lock().unwrap();
        set.remove(&i);
    });
    println!("Final set: {:?}", *shared_set.lock().unwrap());
    assert_eq!(shared_set.lock().unwrap().len(),0);
}
