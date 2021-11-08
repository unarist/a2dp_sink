
use std::{error::Error, io::stdin};

use windows::{Devices::Enumeration::DeviceInformation, Foundation::TypedEventHandler, Media::Audio::AudioPlaybackConnection};

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let selector = AudioPlaybackConnection::GetDeviceSelector()?;
    let device_watcher = DeviceInformation::CreateWatcherAqsFilter(selector)?;
    
    device_watcher.Added(TypedEventHandler::new(|_sender, device_info: &Option<DeviceInformation>| {
        println!("Added: {0}", device_info.as_ref().unwrap().Name()?);
        Ok(())
    })).unwrap();
    device_watcher.Start()?;
    println!("Waiting for connection. Press enter to exit.");

    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(())
}

