use x86_64::structures::paging::PhysFrame;

pub(crate) struct FrameId{
    frame_id: usize,
    frame: PhysFrame
}

impl FrameId {
    pub  fn new(frame_id: usize, frame: PhysFrame) -> Self{
        Self{
            frame_id,
            frame
        }
    }

    pub fn new_increment(frame_id: FrameId)-> Self{
        Self{
            frame_id: frame_id.id() + 1,
            frame: frame_id.frame
        }
    }


    pub fn id(&self)->usize{
        self.frame_id
    }
}