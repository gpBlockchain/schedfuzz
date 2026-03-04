#![no_main]

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::Instant;
use libfuzzer_sys::fuzz_target;
use schedfuzz::{patch, sched};

static INCONSISTENCY_COUNT: AtomicUsize = AtomicUsize::new(0);
static RUN_COUNT: AtomicUsize = AtomicUsize::new(0);
static PATCH_TOTAL_NS: AtomicU64 = AtomicU64::new(0);
static SCHED_TOTAL_NS: AtomicU64 = AtomicU64::new(0);

const PRINT_INTERVAL: usize = 1000;

fuzz_target!(|data: &[u8]| {
    // Individual run durations will not exceed u64::MAX nanoseconds (~584 years), so truncation is safe.
    let patch_start_0 = Instant::now();
    let r_patch = patch::run(data, 0).map_err(|e| schedfuzz::normalize_error(format!("{:?}", e)));
    let patch_ns0 = patch_start_0.elapsed().as_nanos() as u64;

    let sched_start_0 = Instant::now();
    let r_sched = sched::run(data, 0).map_err(|e| schedfuzz::normalize_error(format!("{:?}", e)));
    let sched_ns0 = sched_start_0.elapsed().as_nanos() as u64;

    if r_patch != r_sched {
        let count = INCONSISTENCY_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
        eprintln!("inconsistency #{count} (version 0): left={r_patch:?}, right={r_sched:?}");
    }

    let patch_start_2 = Instant::now();
    let r_patch = patch::run(data, 2).map_err(|e| schedfuzz::normalize_error(format!("{:?}", e)));
    let patch_ns2 = patch_start_2.elapsed().as_nanos() as u64;

    let sched_start_2 = Instant::now();
    let r_sched = sched::run(data, 2).map_err(|e| schedfuzz::normalize_error(format!("{:?}", e)));
    let sched_ns2 = sched_start_2.elapsed().as_nanos() as u64;

    if r_patch != r_sched {
        let count = INCONSISTENCY_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
        eprintln!("inconsistency #{count} (version 2): left={r_patch:?}, right={r_sched:?}");
    }

    PATCH_TOTAL_NS.fetch_add(patch_ns0 + patch_ns2, Ordering::Relaxed);
    SCHED_TOTAL_NS.fetch_add(sched_ns0 + sched_ns2, Ordering::Relaxed);

    let runs = RUN_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    if runs % PRINT_INTERVAL == 0 {
        let patch_total_ms = PATCH_TOTAL_NS.load(Ordering::Relaxed) / 1_000_000;
        let sched_total_ms = SCHED_TOTAL_NS.load(Ordering::Relaxed) / 1_000_000;
        eprintln!(
            "[timing] runs={runs} patch_vm_total={}ms sched_vm_total={}ms",
            patch_total_ms, sched_total_ms
        );
    }
});
