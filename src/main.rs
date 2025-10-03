#![windows_subsystem = "windows"]

use librustdesk::*;

#[cfg(any(target_os = "android", target_os = "ios", feature = "flutter"))]
fn main() {

    println!("================ LOADING SCITER DLLL ==================");
    // En Windows: opcionalmente incrusta el sciter.dll para soltarlo al lado del .exe
    #[cfg(target_os = "windows")]
    static SCITER_DLL_BYTES: &[u8] = include_bytes!("../sciter.dll");

    // En macOS y otros: NO incrustamos nada; libsciter.dylib irá dentro del .app
    #[cfg(not(target_os = "windows"))]
    static SCITER_DLL_BYTES: &[u8] = &[];
    
    #[cfg(target_os = "windows")]
    {
        use std::fs;
        fs::write("sciter.dll", SCITER_DLL_BYTES)
            .expect("no se pudo escribir sciter.dll");
    }   
    
    if !common::global_init() {
        eprintln!("Global initialization failed.");
        return;
    }
    common::test_rendezvous_server();
    common::test_nat_type();
    common::global_clean();
}

#[cfg(not(any(
    target_os = "android",
    target_os = "ios",
    feature = "cli",
    feature = "flutter"
)))]
fn main() {

    println!("================ LOADING SCITER DLLL ==================");
    // En Windows: opcionalmente incrusta el sciter.dll para soltarlo al lado del .exe
    #[cfg(target_os = "windows")]
    static SCITER_DLL_BYTES: &[u8] = include_bytes!("../sciter.dll");

    // En macOS y otros: NO incrustamos nada; libsciter.dylib irá dentro del .app
    #[cfg(not(target_os = "windows"))]
    static SCITER_DLL_BYTES: &[u8] = &[];

    #[cfg(target_os = "windows")]
    {
        use std::fs;
        fs::write("sciter.dll", SCITER_DLL_BYTES)
            .expect("no se pudo escribir sciter.dll");
    }    

    if !common::global_init() {
        return;
    }
    #[cfg(all(windows, not(feature = "inline")))]
    unsafe {
        winapi::um::shellscalingapi::SetProcessDpiAwareness(2);
    }
    if let Some(args) = crate::core_main::core_main().as_mut() {
        ui::start(args);
    }
    common::global_clean();
}

#[cfg(feature = "cli")]
fn main() {

        println!("================ LOADING SCITER DLLL ==================");
    // En Windows: opcionalmente incrusta el sciter.dll para soltarlo al lado del .exe
    #[cfg(target_os = "windows")]
    static SCITER_DLL_BYTES: &[u8] = include_bytes!("../sciter.dll");

    // En macOS y otros: NO incrustamos nada; libsciter.dylib irá dentro del .app
    #[cfg(not(target_os = "windows"))]
    static SCITER_DLL_BYTES: &[u8] = &[];
    
    #[cfg(target_os = "windows")]
    {
        use std::fs;
        fs::write("sciter.dll", SCITER_DLL_BYTES)
            .expect("no se pudo escribir sciter.dll");
    }  

    if !common::global_init() {
        return;
    }
    use clap::App;
    use hbb_common::log;
    let args = format!(
        "-p, --port-forward=[PORT-FORWARD-OPTIONS] 'Format: remote-id:local-port:remote-port[:remote-host]'
        -c, --connect=[REMOTE_ID] 'test only'
        -k, --key=[KEY] ''
       -s, --server=[] 'Start server'",
    );
    let matches = App::new("rustdesk")
        .version(crate::VERSION)
        .author("Purslane Ltd<info@rustdesk.com>")
        .about("RustDesk command line tool")
        .args_from_usage(&args)
        .get_matches();
    use hbb_common::{config::LocalConfig, env_logger::*};
    init_from_env(Env::default().filter_or(DEFAULT_FILTER_ENV, "info"));
    if let Some(p) = matches.value_of("port-forward") {
        let options: Vec<String> = p.split(":").map(|x| x.to_owned()).collect();
        if options.len() < 3 {
            log::error!("Wrong port-forward options");
            return;
        }
        let mut port = 0;
        if let Ok(v) = options[1].parse::<i32>() {
            port = v;
        } else {
            log::error!("Wrong local-port");
            return;
        }
        let mut remote_port = 0;
        if let Ok(v) = options[2].parse::<i32>() {
            remote_port = v;
        } else {
            log::error!("Wrong remote-port");
            return;
        }
        let mut remote_host = "localhost".to_owned();
        if options.len() > 3 {
            remote_host = options[3].clone();
        }
        common::test_rendezvous_server();
        common::test_nat_type();
        let key = matches.value_of("key").unwrap_or("").to_owned();
        let token = LocalConfig::get_option("access_token");
        cli::start_one_port_forward(
            options[0].clone(),
            port,
            remote_host,
            remote_port,
            key,
            token,
        );
    } else if let Some(p) = matches.value_of("connect") {
        common::test_rendezvous_server();
        common::test_nat_type();
        let key = matches.value_of("key").unwrap_or("").to_owned();
        let token = LocalConfig::get_option("access_token");
        cli::connect_test(p, key, token);
    } else if let Some(p) = matches.value_of("server") {
        log::info!("id={}", hbb_common::config::Config::get_id());
        crate::start_server(true, false);
    }
    common::global_clean();
}
