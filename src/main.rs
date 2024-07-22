use std::{process::exit, thread::sleep, time::Duration};

use enigo::{Enigo, Key, Keyboard, Settings};
use fantoccini::{wd::Capabilities, ClientBuilder, Locator};
use regex::Regex;

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let options = Capabilities::new();

    let c = ClientBuilder::native()
        .capabilities(options)
        .connect("http://localhost:9515")
        .await
        .expect("failed to connect to WebDriver");

    // first, go to the Wikipedia page for Foobar
    c.goto("chrome://inspect").await?;

    sleep(Duration::from_millis(2000));

    let id = get_id(&c.source().await.unwrap(), "emulator-5554");

    let x_path = format!("//*[@id=\"{}\"]/div[3]/div/div/div/div[3]/span[1]", id);
    c.find(Locator::XPath(&x_path)).await?.click().await?;

    let windows = c.windows().await.unwrap();
    let new_window = windows.clone().get(1).unwrap().clone();

    c.close_window().await.unwrap();
    c.switch_to_window(new_window.to_owned()).await.unwrap();

    c.maximize_window().await.unwrap();

    sleep(Duration::from_millis(2000));

    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    enigo.key(Key::Control, enigo::Direction::Press).unwrap();
    enigo.key(Key::Shift, enigo::Direction::Press).unwrap();
    enigo
        .key(Key::Unicode('p'), enigo::Direction::Press)
        .unwrap();
    enigo.key(Key::Control, enigo::Direction::Release).unwrap();
    enigo.key(Key::Shift, enigo::Direction::Release).unwrap();
    enigo
        .key(Key::Unicode('p'), enigo::Direction::Release)
        .unwrap();

    enigo.text("lighthouse").unwrap();

    enigo.key(Key::Return, enigo::Direction::Click).unwrap();

    Ok(())
}

fn get_id(html: &str, device_name: &str) -> String {
    let re = Regex::new("id=\"([^\"]+)\"").unwrap();
    let mut ids = Vec::new();

    for cap in re.captures_iter(html) {
        ids.push(cap[1].to_string());
    }

    let device_id = ids.iter().find(|id| id.starts_with(device_name));

    match device_id {
        Some(it) => it.to_owned(),
        None => {
            eprintln!("Id nao encontrado");
            exit(1)
        }
    }
}
