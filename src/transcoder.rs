use notify::DebouncedEvent;
use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use walkdir::WalkDir;
use std::process::Command;
use crate::settings::Settings;

struct Job {
    path: std::path::PathBuf,
    config: Arc<Settings>,
}

impl Job {
    fn new(path: std::path::PathBuf, config: Arc<Settings>) -> Self {
        Job { path, config }
    }

    pub fn transcode(&self) {
        log::debug!("Executing job {:?}", &self.path);
        if let Ok(mut c) = Command::new(&self.config.media.ffmpeg_executable).arg("2").spawn() {
            log::debug!("Job command exited with {:?}", c.wait());
        }
    }
}

struct Worker {
    id: usize,
    thread: std::thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = std::thread::spawn(move || loop {
            let job = receiver
                .lock()
                .expect("Worker failed acquiring queue lock.")
                .recv()
                .expect("Sender has shut down");
            log::debug!("Worker {} received job {:?}", id, job.path);
            job.transcode();
            log::debug!("Worker {} finished job {:?}", id, job.path);
        });
        Worker { id, thread }
    }
}

pub struct Transcoder {
    config: Arc<Settings>,
    workers: Vec<Worker>,
    sender: Sender<Job>,
}

impl Transcoder {
    pub fn new(config: Arc<Settings>) -> Self {
        let (sender, receiver) = channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(4);

        for id in 0..2 {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        let tc = Self {
            config,
            sender,
            workers,
        };

        tc.transcode();

        tc
    }

    pub fn transcode(&self) {
        self.scan();
        // This blocks the thread the trandcoder is created in
        self.handle_fs_events();
    }

    fn handle_fs_events(&self) {
        let (tx, rx) = channel();

        let mut w = watcher(tx, Duration::from_secs(2)).expect("Failed setting up system watcher");

        let config = Arc::clone(&self.config);

        w.watch(config.photos.originals_dir(), RecursiveMode::Recursive)
            .expect("Can't watch originals dir for events");

        let sender = Sender::clone(&self.sender);

        let handle = std::thread::spawn(move || loop {
            match rx.recv() {
                Ok(event) => {
                    if let DebouncedEvent::Create(v) = event {
                        log::debug!("Sending transcoding job: {:?}", v);
                        let _ = sender.send(Job::new(v, Arc::clone(&config)));
                    }
                }
                Err(_) => {
                    log::info!("Stopping watcher");
                    break;
                }
            }
        });

        let _ = handle.join();
    }

    fn scan(&self) {
        let mut transcoded = vec![];

        for entry in WalkDir::new(self.config.photos.originals_dir()) {
            if let Ok(v) = entry {
                let ft = v.file_type();
                if ft.is_dir() || ft.is_symlink() {
                    continue;
                }

                if is_video(v.file_name()) {
                    let filename = v.file_name().to_owned();
                    transcoded.push(filename);
                }
            }
        }

        for entry in WalkDir::new(&self.config.media.videos_path) {
            if let Ok(v) = entry {
                let ft = v.file_type();
                if ft.is_dir() || ft.is_symlink() {
                    continue;
                }

                if is_video(v.file_name()) {
                    let filename = v.file_name().to_owned();
                    if !transcoded.contains(&filename) {
                        let path = v.path().to_owned();
                        log::debug!("Sending transcoding job: {:?}", &path);
                        let _ = self.sender.send(Job::new(path, Arc::clone(&self.config)));
                    }
                }
            }
        }
    }
}

fn is_video(filename: &std::ffi::OsStr) -> bool {
    if let Some(v) = filename.to_str() {
        return v.ends_with(".txt") || v.ends_with(".text");
    }

    false
}
