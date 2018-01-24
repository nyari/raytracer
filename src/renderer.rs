use std::sync::{Arc, Mutex, mpsc};
use std::sync::mpsc::{Sender, Receiver};
use std::{time, thread};

use rtrace::core::{Color, View, PortionableViewIterator, RayCaster, Ray};
use rtrace::defs::{Point2Int};


pub trait RendererOutput {
    fn set_output(&mut self, coord: Point2Int, color: Color) -> bool;
}


pub struct SingleThreadedRenderer<WorldType, OutputType> {
    world: WorldType,
    view: View,
    output: OutputType,
}


impl<WorldType: RayCaster,
     OutputType: RendererOutput> 
    SingleThreadedRenderer<WorldType, OutputType> {

    pub fn new(world: WorldType, view: View, output: OutputType) -> Self {
        Self {  world: world,
                view: view,
                output: output}
    }

    pub fn execute(&mut self) {
        for (ray, coord) in PortionableViewIterator::new(&self.view) {
            match self.world.cast_ray(&ray) {
                Some(color) => {
                    self.output.set_output(coord, color);
                },
                None => ()
            }
        }
    }

    pub fn get_renderer_output(&self) -> &OutputType {
        &self.output
    }
}

enum ControlMessage {
    CastRay(Ray, Point2Int),
    Exit,
}

enum WorkerMessage {
    Ready,
    Result(Option<Color>, Point2Int),
}

struct ParallelWorker<WorldType> {
    world: Arc<WorldType>,
    control_tx: Option<Sender<ControlMessage>>,
    worker_rx: Option<Receiver<WorkerMessage>>,
    join_handle: Option<thread::JoinHandle<()>>,
}

impl<WorldType: 'static + RayCaster + Sync + Send> ParallelWorker<WorldType> {
    pub fn new(world: Arc<WorldType>) -> Self {
        Self {  world: world,
                control_tx: None,
                worker_rx: None,
                join_handle: None}
    }

    pub fn spawn(&mut self) {
        let (control_tx, control_rx): (Sender<ControlMessage>, Receiver<ControlMessage>) = mpsc::channel();
        let (worker_tx, worker_rx): (Sender<WorkerMessage>, Receiver<WorkerMessage>) = mpsc::channel();
        self.control_tx = Some(control_tx);
        self.worker_rx = Some(worker_rx);
        
        let world = self.world.clone();

        self.join_handle = Some(thread::spawn(move || {
            worker_tx.send(WorkerMessage::Ready);
            loop {
                match control_rx.recv() {
                    Ok(message) => {
                        match message {
                            ControlMessage::Exit => {
                                break;
                            },
                            ControlMessage::CastRay(ray, coord) => {
                                let cast_result = world.cast_ray(&ray);
                                if worker_tx.send(WorkerMessage::Result(cast_result, coord)).is_err() {
                                    break;
                                }
                                if worker_tx.send(WorkerMessage::Ready).is_err() {
                                    break;
                                }
                            }
                        }

                    },
                    Err(_) => break,
                }
            }
        }));
    }

    pub fn receive_sync(&self) -> Option<WorkerMessage> {
        let receiver = self.worker_rx.as_ref().expect("ParallelWorker not initalized");
        match receiver.recv() {
            Ok(message) => Some(message),
            Err(_) => None,
        }
    }

    pub fn receive_async(&self) -> Option<WorkerMessage> {
        let receiver = self.worker_rx.as_ref().expect("ParallelWorker not initalized");
        match receiver.try_recv() {
            Ok(message) => Some(message),
            Err(_) => None,
        }
    }

    pub fn send(&self, message: ControlMessage) {
        let sender = self.control_tx.as_ref().expect("ParalellWorker not initialized");
        sender.send(message);
    }

    pub fn join(&mut self) {
        let handle = self.join_handle.take().expect("ParallelWorker not initialized");
        handle.join();
    }
}


pub struct ParalellRenderer<WorldType, OutputType> {
    thread_count: u32,
    world: Arc<WorldType>,
    view: View,
    output: OutputType,
}


impl<WorldType: 'static + RayCaster + Sync + Send,
     OutputType: RendererOutput> 
    ParalellRenderer<WorldType, OutputType> {

    pub fn new(thread_count: u32, world: WorldType, view: View, output: OutputType) -> Self {
        Self {  thread_count: thread_count,
                world: Arc::new(world),
                view: view,
                output: output}
    }

    pub fn execute(&mut self) {
        let mut workers: Vec<ParallelWorker<WorldType>> = Vec::new();
        for worker_filler in 1..(self.thread_count-1) {
            workers.push(ParallelWorker::new(self.world.clone()));
        }
        for worker in workers.iter_mut() {
            worker.spawn();
        }

        for (ray, coord) in PortionableViewIterator::new(&self.view) {
'selector:  loop {
                for worker in workers.iter() {
                    match worker.receive_async() {
                        Some(message) => {
                            match message {
                                WorkerMessage::Ready => {
                                    worker.send(ControlMessage::CastRay(ray, coord));
                                    break 'selector;
                                }
                                WorkerMessage::Result(color_option, coord) => {
                                    match color_option {
                                        Some(color) => { self.output.set_output(coord, color); },
                                        None => (),
                                    }
                                }
                            }
                        },
                        None => (),
                    }
                }
            }
        }

        for worker in workers.iter() {
            worker.send(ControlMessage::Exit);
            let mut done = false;
            while !done {
                match worker.receive_async() {
                    Some(message) => {
                        match message {
                            WorkerMessage::Result(color_option, coord) => {
                                match color_option {
                                    Some(color) => {self.output.set_output(coord, color); },
                                    None => (),
                                }
                            },
                            _ => (),
                        }
                    }
                    None => {
                        done = true;
                    }
                }
            }
        }

        for worker in workers.iter_mut() {
            worker.join();
        }
    }

    pub fn get_renderer_output(&self) -> &OutputType {
        &self.output
    }
}
