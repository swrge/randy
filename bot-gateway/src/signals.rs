// https://unix.stackexchange.com/questions/251195/difference-between-less-violent-kill-signal-hup-1-int-2-and-term-15
use randy_gateway::{CloseFrame, MessageSender};
use std::sync::atomic::Ordering;
use tokio::signal::unix::{signal, SignalKind};

fn handle_signal(sender: &MessageSender, signal_name: &str) {
    println!("Received {}, shutting down...", signal_name);
    super::SHUTDOWN.store(true, Ordering::Relaxed);
    if sender.is_closed() {
        println!("Sender is closed");
    } else if let Err(why) = sender.close(CloseFrame::RESUME) {
        eprintln!("Failed to close sender: {}", why);
    }
}

/// Signal handler for graceful shutdown.
pub async fn on_signal(sender: MessageSender) {
    if sender.is_closed() {
        println!("Sender is already closed!");
    }

    let mut sigint = signal(SignalKind::interrupt()).expect("Failed to register SIGINT handler");
    let mut sighup = signal(SignalKind::hangup()).expect("Failed to register SIGHUP handler");
    let mut sigterm = signal(SignalKind::terminate()).expect("Failed to register SIGTERM handler");

    tokio::select! {
        _ = sigint.recv() => {
            println!("Received SIGINT, shutting down...");
            handle_signal(&sender, "SIGINT");
        },
        _ = sighup.recv() => {
            println!("Received SIGHUP, shutting down...");
            handle_signal(&sender, "SIGHUP");
        },
        _ = sigterm.recv() => {
            println!("Received SIGTERM, shutting down...");
            handle_signal(&sender, "SIGTERM");
        }
    }
}
