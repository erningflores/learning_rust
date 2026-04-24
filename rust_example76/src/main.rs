use std::fs::File;
use std::path::PathBuf;

struct TempFile{
    file: File,
    path: PathBuf,
}

impl TempFile{
    fn new(path: PathBuf) -> std::io::Result<Self>{
        let file = File::create(&path)?;

        Ok(Self{file, path})
    }
}

impl Drop for TempFile{
    fn drop(&mut self){
        if let Err(e) = std::fs::remove_file(&self.path){
            eprintln!("Failed to remove temporary file: {}", e);
        }
        println!("> Dropped temporary file: {:?}", self.path);
    }
}

fn main() -> std::io::Result<()> {
    println!("Drop trait example!\n");

    {
        let _temp = TempFile::new("test.txt".into())?;
        println!("Temporary file created");
    }
    println!("End of scope - file should be cleaned up");

    let _temp2 = TempFile::new("another_test.txt".into())?;
    drop(_temp2);
    println!("Manually drop file");

    Ok(())
}
