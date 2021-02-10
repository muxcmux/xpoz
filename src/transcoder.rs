use crate::settings::Settings;
use notify::DebouncedEvent;
use notify::{watcher, RecursiveMode, Watcher};
use std::process::Command;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use walkdir::WalkDir;

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

        let mp4 = self.mp4();

        let mut tmp = std::env::temp_dir();
        tmp.push(&mp4);

        let mut cmd = Command::new(&self.config.media.ffmpeg.bin);
        cmd.arg("-i");
        cmd.arg(&self.path);

        if self.is_hdr() {
            cmd.args(&self.config.media.ffmpeg.hdr);
        } else {
            cmd.args(&self.config.media.ffmpeg.sdr);
        }

        cmd.arg(&tmp);

        if let Ok(mut child) = cmd.spawn() {
            let status = child.wait();
            log::debug!("Transcoding finished with {:?}", status);

            if status.is_ok() && status.unwrap().success() {
                let mut output = std::path::PathBuf::from(&self.config.media.videos_path);
                output.push(&mp4);
                let _ = std::fs::rename(&tmp, &output);
            }
        }
    }

    fn mp4(&self) -> String {
        let filename = self.path.file_name().unwrap().to_owned();
        let fnstring = filename.into_string().unwrap();
        let uuid = fnstring.split(".").next().unwrap();
        [uuid, "mp4"].join(".")
    }

    fn is_hdr(&self) -> bool {
        let mut probe = Command::new(&self.config.media.ffmpeg.probe);
        probe.args(&[
            "-show_entries",
            "stream=color_space",
            "-select_streams",
            "v",
            "-loglevel",
            "panic",
        ]);
        probe.arg(&self.path);

        if let Ok(out) = probe.output() {
            let out = String::from_utf8_lossy(&out.stdout);
            return out.lines().filter(|line| line.contains("bt2020")).count() > 0;
        }

        false
    }
}

struct Worker {
    _id: usize,
    _thread: std::thread::JoinHandle<()>,
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
        Worker {
            _id: id,
            _thread: thread,
        }
    }
}

pub struct Transcoder {
    config: Arc<Settings>,
    _workers: Vec<Worker>,
    sender: Sender<Job>,
}

impl Transcoder {
    pub fn new(config: Arc<Settings>) -> Self {
        assert!(
            config.media.workers < 25,
            "Can't spawn more than 24 ffmpeg workers"
        );

        let (sender, receiver) = channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(config.media.workers);

        for id in 0..config.media.workers {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        let tc = Self {
            config,
            sender,
            _workers: workers,
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

        for entry in WalkDir::new(&self.config.media.videos_path) {
            if let Ok(v) = entry {
                let ft = v.file_type();
                if ft.is_dir() || ft.is_symlink() {
                    continue;
                }

                if is_video(v.file_name()) {
                    let filename = v.file_name().to_owned();
                    let fnstring = filename.into_string().unwrap();
                    transcoded.push(fnstring.split(".").next().unwrap().to_owned());
                }
            }
        }

        for entry in WalkDir::new(&self.config.photos.originals_dir()) {
            if let Ok(v) = entry {
                let ft = v.file_type();
                if ft.is_dir() || ft.is_symlink() {
                    continue;
                }

                if is_video(v.file_name()) {
                    let filename = v.file_name().to_owned().into_string().unwrap();
                    if !transcoded.contains(&filename.split(".").next().unwrap().to_owned()) {
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
        return v.ends_with(".mp4")
            || v.ends_with(".MP4")
            || v.ends_with(".mov")
            || v.ends_with(".MOV");
    }

    false
}
