use crate::{convert::convert, Result};
use std::{path::PathBuf, sync::mpsc::Receiver};

pub fn run() -> Result<()> {
    let start = std::time::Instant::now();
    let path = args_or_input()?;
    dbg!(&path);

    let files = if path.is_dir() {
        dbg!("is dir");
        filter_for_compat(path)?
    } else {
        vec![path]
    };

    dbg!(&files);

    let mut tasks = vec![];

    // let (timer_tx, timer_rx) = std::sync::mpsc::channel();
    // tasks.push(toggle_timer(timer_rx));
    // timer_tx.send(TimerLife::Start).unwrap();

    let (tx, rx) = std::sync::mpsc::channel();

    for file in files {
        let tx = tx.clone();
        tasks.push(std::thread::spawn(move || {
            // let mut file = file.clone();
            // let file = file.deref_mut();
            convert(file.to_path_buf()).unwrap();
            tx.send(()).unwrap();
            // std::thread::sleep(std::time::Duration::from_millis(50));
        }));
    }

    tasks.push(std::thread::spawn(move || {
        while rx.recv().is_ok() {
            println!("Thread finished");
        }
        drop(rx);
    }));
    drop(tx);

    for task in tasks {
        task.join().unwrap();
    }

    let end = start.elapsed();
    println!("Time elapsed: {:?}", end);

    Ok(())
}

pub fn filter_for_compat(dir_with_files: PathBuf) -> Result<Vec<PathBuf>> {
    let mut files = vec![];
    for entry in std::fs::read_dir(dir_with_files)? {
        let entry = entry?;
        let path = entry.path();
        dbg!(&path);
        match path.extension().and_then(|s| s.to_str()) {
            Some("xlsx") | Some("xlsm") | Some("xlsb") | Some("xls") => files.push(path),
            _ => (),
        }
    }
    Ok(files)
}

pub fn args_or_input() -> Result<PathBuf> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        Ok(PathBuf::from(args[1].clone()))
    } else {
        println!("Please provide the path to the excel file: ");
        // print!("Please provide the path to the excel file: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        Ok(PathBuf::from(input.trim()).canonicalize()?)
    }
}

// struct Timer {
//     start: std::time::Instant,
//     store_time: Option<std::time::Duration>,
// }
//
// impl Timer {
//     pub fn start() -> Self {
//         Self {
//             start: std::time::Instant::now(),
//             store_time: None,
//         }
//     }
//
//     pub fn stop(&mut self) {
//         let el = self.start.elapsed();
//         self.store_time = Some(el);
//     }
// }
//
// pub enum TimerLife {
//     Start,
//     Stop,
// }
//
// // Not working as intended lol
// pub fn toggle_timer(rx_timer: Receiver<TimerLife>) -> std::thread::JoinHandle<()> {
//     let mut timer: Timer = Timer::start();
//
//     std::thread::spawn(move || loop {
//         match rx_timer.recv() {
//             Ok(TimerLife::Start) => {
//                 timer = Timer::start();
//                 println!("Timer started");
//             }
//             Ok(TimerLife::Stop) => {
//                 timer.stop();
//                 println!("Timer stopped: {:?}", timer.store_time.unwrap());
//                 break;
//             }
//             _ => (),
//         }
//     })
// }
