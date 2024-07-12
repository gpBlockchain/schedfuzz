use ckb_types_patch::core::Cycle;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dirs = [
        "fuzz/corpus/fuzz_target_1",
        "fuzz/corpus/fuzz_target_2"
    ];
    for dir in dirs {
        println!("{}", dir);
        let mut failed_corpus_v0 = 0;
        let mut success_corpus_v0 = 0;

        let mut failed_corpus_v1 = 0;
        let mut success_corpus_v1 = 0;

        for directory in std::fs::read_dir(dir)? {
            let path = directory?.path();
            // println!("{}", path.display());
            let data = std::fs::read(path.clone())?;

            let r_patch = schedfuzz::patch::run(&data, 0).map_err(|e| format!("{:?}", e));
            let r_sched = schedfuzz::sched::run(&data, 0).map_err(|e| format!("{:?}", e));
            assert_eq!(r_patch, r_sched);
            match r_patch {
                Ok(_) => {
                    success_corpus_v0 = success_corpus_v0 + 1;
                }
                Err(_) => {
                    failed_corpus_v0 = failed_corpus_v0 + 1;
                }
            }

            let r_patch = schedfuzz::patch::run(&data, 2).map_err(|e| format!("{:?}", e));
            let r_sched = schedfuzz::sched::run(&data, 2).map_err(|e| format!("{:?}", e));
            assert_eq!(r_patch, r_sched);
            match r_patch {
                Ok(_) => {
                    success_corpus_v1 = success_corpus_v1 + 1;
                }
                Err(_) => {
                    failed_corpus_v1 = failed_corpus_v1 + 1;
                }
            }
        }
        println!("version 0  succ:{},failed:{}", success_corpus_v0, failed_corpus_v0);
        println!("version 1  succ:{},failed:{}", success_corpus_v1, failed_corpus_v1);
    };

    Ok(())
}
