#![no_main]

use std::sync::atomic::{AtomicUsize, Ordering};
use libfuzzer_sys::fuzz_target;
use schedfuzz::{patch, sched};

static INCONSISTENCY_COUNT: AtomicUsize = AtomicUsize::new(0);

fuzz_target!(|data: &[u8]| {
    // Fuzzed code goes here
    let r_patch = patch::run(data, 0).map_err(|e| schedfuzz::normalize_error(format!("{:?}", e)));
    let r_sched = sched::run(data, 0).map_err(|e| schedfuzz::normalize_error(format!("{:?}", e)));
    if r_patch != r_sched {
        let count = INCONSISTENCY_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
        eprintln!("inconsistency #{count} (version 0): left={r_patch:?}, right={r_sched:?}");
    }

    let r_patch = patch::run(data, 2).map_err(|e| schedfuzz::normalize_error(format!("{:?}", e)));
    let r_sched = sched::run(data, 2).map_err(|e| schedfuzz::normalize_error(format!("{:?}", e)));
    if r_patch != r_sched {
        let count = INCONSISTENCY_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
        eprintln!("inconsistency #{count} (version 2): left={r_patch:?}, right={r_sched:?}");
    }
});
