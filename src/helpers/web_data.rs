use serde::{Deserialize};

#[derive(Deserialize)]
pub struct VersionData {
    pub builds: Vec<u16>,
}

#[derive(Deserialize)]
pub struct BuildData {
    pub downloads: DownloadsData,
}

#[derive(Deserialize)]
pub struct DownloadsData {
    pub application: Application
}

#[derive(Deserialize)]
pub struct Application {
    pub name: String,
}