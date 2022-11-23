mod battery;
mod utils;

const FILL_CHARS: [char; 9] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

fn main() {
    let gradient = utils::Gradient::new();
    loop {
        if let Ok(battery_infos) = battery::read_battery_infos() {
            for i in 0..battery_infos.len() {
                if i > 0 {
                    print!(" | ");
                }

                let info = &battery_infos[i];
                if battery_infos.len() > 1 {
                    print!("{} ", info.name);
                }

                let cap = info.capacity();
                let cap_perc = (cap * 100.0) as usize;
                let color = match info.status {
                    battery::BatteryStatus::Charging => utils::RGB {
                        r: 51,
                        g: 153,
                        b: 255,
                    },
                    battery::BatteryStatus::Discharging | battery::BatteryStatus::NotCharging => {
                        gradient.get(cap)
                    }
                    battery::BatteryStatus::Full => gradient.get(1.0),
                    _ => utils::RGB {
                        r: 102,
                        g: 102,
                        b: 102,
                    },
                };
                let fill_char = FILL_CHARS
                    [((cap * FILL_CHARS.len() as f64).floor() as usize).min(FILL_CHARS.len() - 1)];
                print!("<span color=\"{color}\"><span bgcolor=\"#333333\">{fill_char}</span> {cap_perc}%</span>");

                if let Ok(opt) = info.hours_to_status_change() {
                    if let Some(hours) = opt {
                        let h = hours.floor() as usize;
                        let m = (hours.fract() * 60.) as usize;
                        print!(" {h}:{m:02}");
                    }
                } else {
                    print!(" <span color=\"red\">?</span>")
                }
            }
            println!();
        } else {
            println!("<span color=\"red\">?</span>")
        }
        std::thread::sleep(std::time::Duration::from_secs(3));
    }
}
