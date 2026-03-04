fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dirs = [
        "fuzz/corpus/fuzz_tx_consistency",
        "fuzz/corpus/fuzz_tx_consistency_only_valid_data1",
        "fuzz/corpus/crash"
    ];
    let mut total_inconsistencies = 0;
    for dir in dirs {
        println!("{}", dir);
        let mut failed_corpus_v0 = 0;
        let mut success_corpus_v0 = 0;
        let mut inconsistencies_v0 = 0;

        let mut failed_corpus_v1 = 0;
        let mut success_corpus_v1 = 0;
        let mut inconsistencies_v1 = 0;

        for directory in std::fs::read_dir(dir)? {
            let path = directory?.path();
            let data = std::fs::read(path.clone())?;

            let r_patch = schedfuzz::patch::run(&data, 0).map_err(|e| schedfuzz::normalize_error(format!("{:?}", e)));
            let r_sched = schedfuzz::sched::run(&data, 0).map_err(|e| schedfuzz::normalize_error(format!("{:?}", e)));
            if r_patch != r_sched {
                inconsistencies_v0 += 1;
                eprintln!("inconsistency (version 0) file: {}\n  left:  {:?}\n  right: {:?}", path.display(), r_patch, r_sched);
            }
            match r_patch {
                Ok(_) => {
                    success_corpus_v0 = success_corpus_v0 + 1;
                }
                Err(_) => {
                    failed_corpus_v0 = failed_corpus_v0 + 1;
                }
            }

            let r_patch = schedfuzz::patch::run(&data, 2).map_err(|e| schedfuzz::normalize_error(format!("{:?}", e)));
            let r_sched = schedfuzz::sched::run(&data, 2).map_err(|e| schedfuzz::normalize_error(format!("{:?}", e)));
            if r_patch != r_sched {
                inconsistencies_v1 += 1;
                eprintln!("inconsistency (version 2) file: {}\n  left:  {:?}\n  right: {:?}", path.display(), r_patch, r_sched);
            }
            match r_patch {
                Ok(_) => {
                    success_corpus_v1 = success_corpus_v1 + 1;
                }
                Err(_) => {
                    failed_corpus_v1 = failed_corpus_v1 + 1;
                }
            }
        }
        println!("version data   succ:{}, failed:{}, inconsistencies:{}", success_corpus_v0, failed_corpus_v0, inconsistencies_v0);
        println!("version data1  succ:{}, failed:{}, inconsistencies:{}", success_corpus_v1, failed_corpus_v1, inconsistencies_v1);
        total_inconsistencies += inconsistencies_v0 + inconsistencies_v1;
    };

    println!("\ntotal inconsistencies: {}", total_inconsistencies);
    Ok(())
}
