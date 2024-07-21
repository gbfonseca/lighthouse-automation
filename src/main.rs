use std::{thread::sleep, time::Duration};

use enigo::{Enigo, Key, Keyboard, Settings};
use fantoccini::{ClientBuilder, Locator};

// let's set up the sequence of steps we want the browser to take
#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let c = ClientBuilder::native()
        .connect("http://localhost:9515")
        .await
        .expect("failed to connect to WebDriver");

    // first, go to the Wikipedia page for Foobar
    c.goto("chrome://inspect").await?;
    let url = c.current_url().await?;

    sleep(Duration::from_millis(2000));

    // click "Foo (disambiguation)"
    c.find(Locator::XPath("//*[@id=\"emulator-5554:webview_devtools_remote_6189\"]/div[2]/div[1]/div/div/div[3]/span[1]")).await?.click().await?;

    let windows = c.windows().await.unwrap();
    let new_window = windows.clone().get(1).unwrap().clone();

    c.close_window().await;
    c.switch_to_window(new_window.to_owned()).await;

    c.maximize_window().await;

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

    c.find(Locator::LinkText("Analyze page load"))
        .await?
        .click()
        .await?;

    sleep(Duration::from_millis(115000));

    // sleep(Duration::from_millis(20000));

    // // click "Foo Lake"
    // c.find(Locator::LinkText("Foo Lake")).await?.click().await?;

    // let url = c.current_url().await?;
    // // assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foo_Lake");

    c.close().await
}
