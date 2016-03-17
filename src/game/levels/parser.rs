
use ::game::levels::{Level, OpCode};

pub struct LevelParser {
    level_count: u32,
    current_opcode: usize,

    last_opcode_time: u32,  // Last time we processed an opcode
    opcode_wait: u32,        // How long to wait until we process another opcode

    current: OpCode
}

impl LevelParser {
    pub fn new(level_count: u32) -> LevelParser {
        LevelParser {
            level_count: level_count,
            current_opcode: 0,

            last_opcode_time: 0,
            opcode_wait: 0,

            current: OpCode::None
        }
    }

    pub fn process(&mut self, current_level: &mut Box<Level>, current_time: u32) -> OpCode {
        // Should we process the level?
        if current_time - self.last_opcode_time >= self.opcode_wait {
            let result = current_level.get(self.current_opcode);
            self.current = result.clone();
            self.current_opcode += 1;
            self.last_opcode_time = current_time;
            result
        } else {
            OpCode::None
        }
    }

    pub fn wait_for(&mut self, seconds: u32) {
        self.opcode_wait = seconds * 1000;
    }

    pub fn reset(&mut self) {
        self.current_opcode = 0;
        self.opcode_wait = 0;
    }

    pub fn current_opcode(&self) -> OpCode {
       self.current.clone()
    }
}
