extern crate dirs;

pub fn intiate_crawl() {
    match dirs::home_dir() {
        Some(path) => info!("Home directory: {}", path.display()),
        None => error!("Could not find home directory"),
    }
    println!("Fix");
}
