use tokio::{task, time};

use custom_utils::logger::debug;
use custom_utils::logger::LevelFilter::Trace;
use rumqttc::{self, AsyncClient, MqttOptions, QoS, TlsConfiguration, Transport};
use std::error::Error;
use std::time::Duration;
use tokio::fs::read;

#[tokio::main(worker_threads = 1)]
async fn main() -> Result<(), Box<dyn Error>> {
    custom_utils::logger::logger_stdout(Trace);
    let mut mqttoptions = MqttOptions::new("test-1", "broker-cn.emqx.io", 8883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    if let Some(ca) = custom_utils::args::arg_value("--ca", "-c") {
        debug!("TlsConfiguration::SimpleNativeCa");
        let ca = read(ca).await?;
        let ca = rustls_pemfile::certs(&mut ca.as_slice())?.remove(0);
        let transport = Transport::Tls(TlsConfiguration::SimpleNativeCa { ca: ca });
        mqttoptions.set_transport(transport);
    } else {
        debug!("TlsConfiguration::Native");
        mqttoptions.set_transport(Transport::Tls(TlsConfiguration::Native));
    }

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    task::spawn(async move {
        requests(client).await;
        time::sleep(Duration::from_secs(3)).await;
    });
    loop {
        let event = eventloop.poll().await;
        println!("{:?}", event?);
    }
}

async fn requests(client: AsyncClient) {
    client
        .subscribe("hello/world", QoS::AtMostOnce)
        .await
        .unwrap();

    for i in 1..=3 {
        client
            .publish("hello/world", QoS::ExactlyOnce, false, vec![1; i])
            .await
            .unwrap();

        time::sleep(Duration::from_secs(1)).await;
    }
    time::sleep(Duration::from_secs(120)).await;
}