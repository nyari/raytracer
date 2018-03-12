use std::sync::{Arc, mpsc};
use std::sync::mpsc::{Sender, Receiver, RecvError, TryRecvError, SendError};
use std::{thread};

use rtrace::core::{Color, View, ViewIterator, RayCaster, Ray};
use rtrace::defs::{Point2Int};


pub trait RendererOutput {
    fn set_output(&mut self, coord: Point2Int, color: Color) -> bool;
}

#[allow(dead_code)]
pub struct SingleThreadedRenderer<WorldType, OutputType> {
    world: WorldType,
    view: View,
    output: OutputType,
}


#[allow(dead_code)]
impl<WorldType: RayCaster,
     OutputType: RendererOutput> 
    SingleThreadedRenderer<WorldType, OutputType> {

    pub fn new(world: WorldType, view: View, output: OutputType) -> Self {
        Self {  world: world,
                view: view,
                output: output}
    }

    pub fn execute(&mut self) {
        for (ray, coord) in ViewIterator::new(&self.view) {
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
    join_handle: Option<thread::JoinHandle<()>>
}

#[allow(dead_code)]
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
        
        let world = Arc::clone(&self.world);

        self.join_handle = Some(thread::spawn(move || {
            worker_tx.send(WorkerMessage::Ready).expect("Initial ready message in worker unhandled");
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

    pub fn receive_sync(&self) -> Result<WorkerMessage, RecvError> {
        let receiver = self.worker_rx.as_ref().expect("ParallelWorker not initalized");
        receiver.recv()
    }

    pub fn receive_async(&self) -> Result<Option<WorkerMessage>, ()> {
        let receiver = self.worker_rx.as_ref().expect("ParallelWorker not initalized");
        match receiver.try_recv() {
            Ok(message) => Ok(Some(message)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(_) => Err(()),
        }
    }

    pub fn send(&self, message: ControlMessage) -> Result<(), SendError<ControlMessage>> {
        let sender = self.control_tx.as_ref().expect("ParallelWorker not initialized");
        sender.send(message)
    }

    pub fn join(&mut self) -> Result<(), ()>{
        let handle = self.join_handle.take().expect("ParallelWorker not initialized");
        match handle.join() {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }
    }
}


enum ParallelRenderedInternalError {
    FailedWorker(usize),
    FailedWorkerWithControlMessage(usize, ControlMessage),
    EndOfViewIteration
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

    fn process_iteration(workers: &Vec<ParallelWorker<WorldType>>, view_iterator: &mut ViewIterator, output: &mut OutputType) -> Result<(), ParallelRenderedInternalError> {
        for (worker_index, worker) in workers.iter().enumerate() {
            let worker_receive_result = worker.receive_async();
            
            if let Err(()) = worker_receive_result {
                return Err(ParallelRenderedInternalError::FailedWorker(worker_index))
            }

            if let Some(message) = worker_receive_result.unwrap() {
                match message {
                    WorkerMessage::Ready => {
                        match view_iterator.next() {
                            Some((ray, coord)) => {
                                println!("Sent ray to: {}, {}", coord.x, coord.y);
                                if let Err(SendError(message)) = worker.send(ControlMessage::CastRay(ray, coord)) {
                                    return Err(ParallelRenderedInternalError::FailedWorkerWithControlMessage(worker_index, message))
                                }
                            },
                            None => {
                                return Err(ParallelRenderedInternalError::EndOfViewIteration)
                            }
                        }
                    }

                    WorkerMessage::Result(color_option, coord) => {
                        match color_option {
                            Some(color) => { println!("Recevied result for: {}, {}", coord.x, coord.y); output.set_output(coord, color); },
                            None => (),
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn replace_worker(workers: &mut Vec<ParallelWorker<WorldType>>, index: usize, world: &Arc<WorldType>) {
        workers.swap_remove(index);
        let mut new_worker = ParallelWorker::new(Arc::clone(world));
        new_worker.spawn();
        workers.push(new_worker);
    }

    pub fn execute(&mut self) {
        let mut workers: Vec<ParallelWorker<WorldType>> = Vec::new();
        for _ in 1..(self.thread_count) {
            let mut new_worker = ParallelWorker::new(Arc::clone(&self.world));
            new_worker.spawn();
            workers.push(new_worker);
        }

        {
            let mut view_iterator = ViewIterator::new(&self.view);
            loop {
                match Self::process_iteration(&workers, &mut view_iterator, &mut self.output) {
                    Err(ParallelRenderedInternalError::FailedWorker(worker_index)) => {
                        Self::replace_worker(&mut workers, worker_index, &self.world);
                    },

                    Err(ParallelRenderedInternalError::FailedWorkerWithControlMessage(worker_index, message)) => {
                        Self::replace_worker(&mut workers, worker_index, &self.world);
                        if let ControlMessage::CastRay(ray, coord) = message {
                            match self.world.cast_ray(&ray) {
                                Some(color) => {
                                    self.output.set_output(coord, color);
                                }
                                None => (),
                            }
                        }
                    }

                    Err(ParallelRenderedInternalError::EndOfViewIteration) => {
                        break;
                    },

                    _ => (),
                }
            }
        }

        for worker in workers.iter() {
            worker.send(ControlMessage::Exit).is_ok();
        }
        for worker in workers.iter_mut() {
            worker.join().is_ok();
        }
        for worker in workers.iter() {
            let mut done = false;
            while !done {
                match worker.receive_sync() {
                    Ok(message) => {
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
                    Err(_) => {
                        done = true;
                    }
                }
            }
        }
    }

    pub fn get_renderer_output(&self) -> &OutputType {
        &self.output
    }
}
