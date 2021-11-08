
use std::{cell::RefCell, error::Error, io::stdin, rc::Rc};

use windows::{Devices::Enumeration::{DeviceInformation, DeviceInformationUpdate}, Foundation::TypedEventHandler, Media::Audio::{AudioPlaybackConnection, AudioPlaybackConnectionOpenResultStatus}, runtime::HSTRING};

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let selector = AudioPlaybackConnection::GetDeviceSelector()?;
    let device_watcher = DeviceInformation::CreateWatcherAqsFilter(selector)?;
    let connection: Rc<RefCell<Option<AudioPlaybackConnection>>> = Rc::new(RefCell::new(None));
    let connection2 = connection.clone();
    
    device_watcher.Added(TypedEventHandler::new(  move |_sender, args: &Option<DeviceInformation>| {
        // ここで ? してもイベントハンドラのResultに反映されるだけ
        let device_info = args.as_ref().unwrap();
        println!("Added: {} ({})", device_info.Id().unwrap(), device_info.Name().unwrap());
        let new_connection = connect(device_info.Id().unwrap()).unwrap();
        connection.replace(Some(new_connection));
        Ok(())
    })).unwrap();
    device_watcher.Removed(TypedEventHandler::new( move |_sender, args: &Option<DeviceInformationUpdate>| {
        // ここで ? してもイベントハンドラのResultに反映されるだけ
        let update = args.as_ref().unwrap();
        println!("Removed: {0}", update.Id().unwrap());
        if let Some(con) = connection2.borrow().as_ref() {
            if con.DeviceId().unwrap() == update.Id().unwrap() {
                connection2.replace(None);
                println!("[AudioPlaybackConnection] Dispose")
            }
        }
        Ok(())
    })).unwrap();
    device_watcher.Start()?;
    println!("Waiting for connection. Press enter to exit.");

    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(())
}

fn format_status(status: AudioPlaybackConnectionOpenResultStatus) -> String {
    match status {
        AudioPlaybackConnectionOpenResultStatus::Success => String::from("Success"),
        AudioPlaybackConnectionOpenResultStatus::DeniedBySystem => String::from("DeniedBySystem"),
        AudioPlaybackConnectionOpenResultStatus::RequestTimedOut => String::from("RequestTimedOut"),
        AudioPlaybackConnectionOpenResultStatus::UnknownFailure => String::from("UnknownFailure"),
        x => format!("{:?}", x)
    }
}

fn connect(device_id: HSTRING) -> Result<AudioPlaybackConnection, Box<dyn Error>> {
    let connection = AudioPlaybackConnection::TryCreateFromId(device_id)?;
    connection.StateChanged(TypedEventHandler::new(|sender: &Option<AudioPlaybackConnection>, _| {
        // ここで ? してもイベントハンドラのResultに反映されるだけ
        let connection = sender.as_ref().unwrap();
        println!("[AudioPlaybackConnection] OnStateChanged: {:?}", connection.State().unwrap());
        Ok(())
    }))?;
    connection.Start()?;
    let result = connection.Open()?;
    println!("[AudioPlaybackConnection] Open: {}", format_status(result.Status()?));
    // TODO: non-Success handling
    Ok(connection)
}

