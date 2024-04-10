pub mod actions {
    use sysinfo::{Pid, PidExt, Process, ProcessExt, System, SystemExt};

    pub fn clean() {
        println!("\nCleaning...");
        std::thread::sleep(std::time::Duration::from_secs(3));
        println!("Clean Complete. Exiting...\n");
        std::process::exit(0);
    }
    pub fn watch() {
        println!("\nWatching...");

        // Run in a seperate thread and then detach it

        loop {
            println!("\nWatch interval...");
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
    }

    pub fn purge() {
        // Run through old stuff and PURGE
    }

    pub fn kill() {
        println!("\nStopping Sentry...\n");

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

        println!("All Sentry instances stopped. Exiting...\n");
        std::process::exit(0);
    }
}
