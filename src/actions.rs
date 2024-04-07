pub mod actions {
    use sysinfo::{Pid, PidExt, Process, ProcessExt, System, SystemExt};

    pub fn clean() {
        println!(" ");
        println!("Cleaning...");
        std::thread::sleep(std::time::Duration::from_secs(3));
        println!("Clean Complete.");
        println!(" ");
        std::process::exit(0);
    }
    pub fn watch() {
        println!(" ");
        println!("Watching...");

        loop {
            println!("Watch interval...");
            println!(" ");
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
    }

    pub fn archive() {}

    pub fn kill() {
        println!(" ");
        println!("Stopping Sentry...");
        println!(" ");

        let mut system: System = System::new_all();
        system.refresh_all();

        let mut sentry_instances: Vec<(&Pid, &Process)> = Vec::new();
        let current_sentry: u32 = std::process::id();

        for (pid, process) in system.processes() {
            if process.name().eq_ignore_ascii_case("sentry_lite") {
                sentry_instances.push((pid, process));
            }
        }

        for (pid, process) in sentry_instances {
            if !pid.as_u32().eq(&current_sentry) {
                process.kill();
            }
        }

        println!("All Sentry instances stopped. Exiting...");
        std::process::exit(0);
    }
}
