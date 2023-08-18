/* ********************************************************************************************************** */
/*                                                                                                            */
/*                                                     :::::::::  ::::::::   ::::::::   :::    ::: :::::::::: */
/* peer.rs                                            :+:    :+: :+:    :+: :+:    :+: :+:    :+: :+:         */
/*                                                   +:+    +:+ +:+    +:+ +:+        +:+    +:+ +:+          */
/* By: se-yukun <yukun@doche.io>                    +#+    +:+ +#+    +:+ +#+        +#++:++#++ +#++:++#      */
/*                                                 +#+    +#+ +#+    +#+ +#+        +#+    +#+ +#+            */
/* Created: 2023/08/18 02:58:51 by se-yukun       #+#    #+# #+#    #+# #+#    #+# #+#    #+# #+#             */
/* Updated: 2023/08/18 02:58:54 by se-yukun      #########  ########   ########  ###    ### ##########.io.    */
/*                                                                                                            */
/* ********************************************************************************************************** */

use std::net::SocketAddr;
use std::process::Command;
use std::sync::Arc;
use std::{env, process, thread};

use std::net::UdpSocket;

use tun_tap::{Iface, Mode};

fn cmd(cmd: &str, args: &[&str]) {
    let ecode = Command::new(cmd)
        .args(args)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    assert!(ecode.success(), "Failed to execte {}", cmd);
}

pub async fn server() {
    // Read Local & Remote IP from args
    let loc_address = env::args()
        .nth(2)
        .unwrap()
        .parse::<SocketAddr>()
        .unwrap_or_else(|err| {
            eprintln!("Unable to recognize listen ip: {}", err);
            process::exit(1);
        });

    // Create socket
    let socket = UdpSocket::bind(&loc_address).unwrap();
    let socket = Arc::new(socket);

    // Create interface
    let name = &env::args().nth(3).expect("Unable to read Interface name");
    let iface = Iface::new(&name, Mode::Tap).unwrap_or_else(|err| {
        eprintln!("Failed to configure the interface name: {}", err);
        process::exit(1);
    });
    let iface = Arc::new(iface);

    // Configure the „local“ (kernel) endpoint.
    let ip = &env::args()
        .nth(4)
        .expect("Unable to recognize remote interface IP");
    cmd("ip", &["addr", "add", "dev", iface.name(), &ip]);
    cmd("ip", &["link", "set", "up", "dev", iface.name()]);

    let iface = Arc::new(iface);
    let iface_writer = Arc::clone(&iface);
    let iface_reader = Arc::clone(&iface);
    let socket_keep = socket.clone();
    let socket_send = socket.clone();
    let socket_recv = socket.clone();

    {
        let mut buf = vec![0; 1500];
        let (_, addr) = socket.recv_from(&mut buf).unwrap();

        let keeper = thread::spawn(move || {
            println!("k loaded");
            loop {
                match socket_keep.send_to(&vec![], addr) {
                    Ok(_) => {}
                    Err(_) => {
                        break;
                    }
                };
            }
        });
        let writer = thread::spawn(move || {
            println!("w loaded");
            loop {
                if keeper.is_finished() {
                    break;
                }
                let mut buf = vec![0; 1500];
                let len = socket_recv.recv(&mut buf).unwrap();
                println!("recv: {:?}", len);
                iface_writer.send(&buf[..len]).unwrap();
            }
        });
        let _ = thread::spawn(move || {
            println!("r loaded");
            loop {
                let mut buf = vec![0; 1504];
                let len = iface_reader.recv(&mut buf).unwrap();
                println!("if recv");
                if len > 0 {
                    match socket_send.send_to(&buf[..len], &addr) {
                        Ok(_) => {}
                        Err(_) => {
                            break;
                        }
                    };
                    println!("send: {:?}", len);
                }
            }
        });
        writer.join().unwrap();
    }
}
