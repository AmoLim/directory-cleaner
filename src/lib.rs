use std::fs;
use std::time::{SystemTime, Duration};
use std::path::Path;
use std::error::Error;

pub struct DeleteConfig {
    pub dir : String,   // the target directory
    pub cutoff_time : SystemTime    // delete the files created before cutoff_time.
}

impl DeleteConfig {
    pub fn build(
        mut args : impl Iterator<Item = String>
    ) -> Result<DeleteConfig, &'static str> {
        args.next();

        let dir = match args.next() {
            Some(dir) => dir,
            None => return Err("Didn't get a directory string")
        };

        let threshold = match args.next() {
            Some(threshold) => threshold,
            None => return Err("Didn't get a threshold argument")
        };  // get the threshold argument, measured in sec.

        let threshold : u64 = match threshold.parse() {
            Ok(threshold) => threshold,
            Err(_) => return Err("invalid threshold argument")
        };  // parse into u64.

        let cutoff_time = SystemTime::now() - Duration::from_secs(threshold);

        Ok(DeleteConfig{ dir, cutoff_time})
    }

}

pub fn run(config : DeleteConfig) -> Result<(), Box<dyn Error>>{
    let dir = Path::new(config.dir.as_str());

    if !dir.exists() {
        return Err("The directory does not exists".into());
    }
    
    delete_files(dir, config.cutoff_time)?;

    Ok(())
}

fn delete_files(dir : &Path, time : SystemTime) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(dir)? {
        let file = entry?;
        let created_time = file.metadata()?.created()?;    // get the created time of file.
        if created_time < time {
            fs::remove_file(file.path())?;
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::time;
    use std::thread;

    #[test]
    fn test_delete_old_files() {
        // Create a temp directory for this test.
        let dir = "./temp_for_test";

        fs::create_dir_all(dir).expect("can not create the directory");

        let dir = Path::new(dir);
        
        // Create some test files in the temporary folder.
        let file1_path = dir.join("file1.txt");
        let file2_path = dir.join("file2.txt");
        fs::File::create(&file1_path).expect("can not create a temp file for test");
        fs::File::create(&file2_path).expect("can not create a temp file for test");

        // Set threshold to 1 sec and wait for 2 sec
        let threshold = time::Duration::from_secs(1);
        thread::sleep(threshold + time::Duration::from_secs(1));

        let result = delete_files(dir, SystemTime::now() - threshold);

        assert!(result.is_ok());

        assert!(!file1_path.exists());
        assert!(!file2_path.exists());
    }
}