use tokio_metrics::RuntimeMonitor;

// use winit::event_loop::EventLoopBuilder;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoopBuilder;
#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;
#[cfg(target_os = "android")]
use winit::platform::android::EventLoopBuilderExtAndroid;
use winit::platform::run_return::EventLoopExtRunReturn;

#[cfg(target_os = "android")]
#[no_mangle]
async fn android_main(app: AndroidApp) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let handle = tokio::runtime::Handle::current();
    let runtime_monitor = RuntimeMonitor::new(&handle);

    // print runtime metrics every 500ms
    let frequency = std::time::Duration::from_millis(500);
    tokio::spawn(async move {
        for metrics in runtime_monitor.intervals() {
            println!("Metrics = {:?}", metrics);
            tokio::time::sleep(frequency).await;
        }
    });

    // run some tasks
    tokio::spawn(do_work());

    let mut event_loop = EventLoopBuilder::new().with_android_app(app).build();

    let mut quit = false;

    while !quit {
        event_loop.run_return(|event, _, control_flow| {
            control_flow.set_wait();

            if let Event::WindowEvent { event, .. } = &event {
                // Print only Window events to reduce noise
                println!("{event:?}");
            }

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    quit = true;
                }
                Event::MainEventsCleared => {
                    control_flow.set_exit();
                }
                _ => (),
            }
        });
    }

    Ok(())
}

async fn do_work() {
    for _ in 0..25 {
        tokio::task::yield_now().await;
    }
}
