pub mod logic;

pub struct Application {
    pub render: bool,
    pub test: bool,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Message {
    StartRender,
    StopRender,
    HandleTest,
}

pub enum Response {
    Render,
    Test(bool),
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;
    use anyhow::Result;

    type Master = logic::LowBullMaster<Message, Rc<RefCell<Application>>, Response>;

    fn handle_render(application: &mut Rc<RefCell<Application>>, _: Response) -> Result<()> {
        application.borrow_mut().render = true;
        Ok(())
    }

    fn handle_test(application: &mut Rc<RefCell<Application>>, _: Response) -> Result<()> {
        application.borrow_mut().test = true;
        Ok(())
    }

    fn ui_frame(master: &mut Master, frame: u32) {
        if frame % 2 == 0 {
            master
                .run_logic(Message::StartRender, Response::Render)
                .unwrap();
        } else {
            master
                .run_logic(Message::StopRender, Response::Render)
                .unwrap();
        }
    }

    #[test]
    fn test_master() {
        let application = Rc::new(RefCell::new(Application {
            render: false,
            test: false,
        }));

        let mut master = logic::LowBullMaster::<Message, Rc<RefCell<Application>>, Response>::new(
            application.clone(),
        );

        master.register_logic(Message::StartRender, Box::new(handle_render));
        master.register_logic(Message::HandleTest, Box::new(handle_test));

        assert!(!application.borrow().render);
        assert!(!application.borrow().test);

        for frame in 0..10 {
            ui_frame(&mut master, frame as u32);
            // assert!(application.borrow().render);
            // assert!(application.borrow().test);
        }
    }
}
