use spin::mutex::SpinMutex;
use volatile::Volatile;

pub(crate) static APIC_TIMER: SpinMutex<TimerManager> = SpinMutex::new(TimerManager::new());

pub(crate) trait APicTimer {
    fn lvt_timer(&mut self) -> Volatile<&mut u32>;
    fn initial_count(&mut self) -> Volatile<&mut u32>;
    fn current_count(&mut self) -> Volatile<&mut u32>;
    fn divide_config(&mut self) -> Volatile<&mut u32>;
}

pub(crate) struct TimerManager {
    time: u64,
}

impl TimerManager {
    pub const fn new() -> Self {
        unsafe {
            Self {
                time: 0
            }
        }
    }

    pub fn init(&mut self) {
        self.divide_config().write(0b1011); // divide 1:1
        self.lvt_timer().write((0b010 << 16) | 0x41);
        self.initial_count().write(0x1000000);
    }

    pub fn tick(&mut self) {
        self.time += 1
    }
}

impl APicTimer for TimerManager {
    fn lvt_timer(&mut self) -> Volatile<&mut u32> {
        unsafe { Volatile::new((0xfee00320 as *mut u32).as_mut().unwrap()) }
    }

    fn initial_count(&mut self) -> Volatile<&mut u32> {
        unsafe { Volatile::new((0xfee00380 as *mut u32).as_mut().unwrap()) }
    }

    fn current_count(&mut self) -> Volatile<&mut u32> {
        unsafe { Volatile::new((0xfee00390 as *mut u32).as_mut().unwrap()) }
    }

    fn divide_config(&mut self) -> Volatile<&mut u32> {
        unsafe { Volatile::new((0xfee003e0 as *mut u32).as_mut().unwrap()) }
    }
}