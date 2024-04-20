use criterion::{black_box, criterion_group, criterion_main, Criterion};

use std::{cell::RefCell, rc::Rc};

use anyhow::Result;

pub struct Application {
    pub render: bool,
    pub test: bool,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Message {
    StartRender,
    StopRender,
    ToggleTest,
    CheckTest,
    CheckRender,
}

pub enum Response {
    Render,
    Test(bool),
    None,
}

type Master = lowbulls::logic::LowBullMaster<Message, Rc<RefCell<Application>>, Response>;

fn handle_render_start(application: &mut Rc<RefCell<Application>>, _: Response) -> Result<()> {
    application.borrow_mut().render = true;
    Ok(())
}

fn handle_render_stop(application: &mut Rc<RefCell<Application>>, _: Response) -> Result<()> {
    application.borrow_mut().render = false;
    Ok(())
}

fn toggle_test(application: &mut Rc<RefCell<Application>>, _: Response) -> Result<()> {
    let old = application.borrow().test;
    application.borrow_mut().test = !old;
    Ok(())
}

fn check_test(application: &mut Rc<RefCell<Application>>, response: Response) -> Result<()> {
    if let Response::Test(value) = response {
        assert!(application.borrow().test == value);

        Ok(())
    } else {
        Err(anyhow::anyhow!("Invalid response"))
    }
}

fn check_render(application: &mut Rc<RefCell<Application>>, response: Response) -> Result<()> {
    if let Response::Test(value) = response {
        assert!(application.borrow().render == value);

        Ok(())
    } else {
        Err(anyhow::anyhow!("Invalid response"))
    }
}

fn ui_frame(master: &mut Master, frame: u32) {
    if frame % 2 == 0 {
        master
            .run_logic(Message::CheckRender, Response::Test(false))
            .unwrap();

        master
            .run_logic(Message::StartRender, Response::Render)
            .unwrap();
    } else {
        master
            .run_logic(Message::CheckRender, Response::Test(true))
            .unwrap();

        master
            .run_logic(Message::StopRender, Response::Render)
            .unwrap();
    }

    if frame % 6 == 0 {
        master
            .run_logic(Message::ToggleTest, Response::Test(true))
            .unwrap();
    }

    if frame % 3 == 0 {
        master
            .run_logic(Message::ToggleTest, Response::None)
            .unwrap();
    }
}

fn test_master(master: &mut Master) -> u64 {
    for frame in 0..10 {
        ui_frame(master, frame as u32);
    }

    0
}

fn test_single(master: &mut Master) -> u64 {
    master
        .run_logic(Message::StartRender, Response::None)
        .unwrap();

    0
}

fn criterion_benchmark(c: &mut Criterion) {
    let application = Rc::new(RefCell::new(Application {
        render: false,
        test: false,
    }));

    let mut master =
        lowbulls::logic::LowBullMaster::<Message, Rc<RefCell<Application>>, Response>::new(
            application.clone(),
        );

    master.register_logic(Message::StartRender, Box::new(handle_render_start));
    master.register_logic(Message::StopRender, Box::new(handle_render_stop));
    master.register_logic(Message::ToggleTest, Box::new(toggle_test));
    master.register_logic(Message::CheckTest, Box::new(check_test));
    master.register_logic(Message::CheckRender, Box::new(check_render));

    c.bench_function("Test", |b| b.iter(|| test_master(black_box(&mut master))));

    c.bench_function("Single Action Call", |b| {
        b.iter(|| test_single(black_box(&mut master)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
