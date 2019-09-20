// use std::net::SocketAddr;
// use std::str::FromStr;
// use std::time::Duration;
// use std::{io, process, thread};

// use actix::io::{FramedWrite, WriteHandler};
// use actix::{
//     Actor, Arbiter, AsyncContext, Context, Handler, Message, Running, StreamHandler, System,
// };
// use futures::future::ok;
// use futures::Future;
// use tokio_codec::FramedRead;
// use tokio_io::io::WriteHalf;
// use tokio_io::AsyncRead;
// use tokio_tcp::TcpStream;

// use crate::codec::{ClientChatCodec, RpcRequestC, RpcResponseC, UrlGetterMsg};

// fn main() -> std::io::Result<()> {
//     println!("Running chat client");

//     System::run(|| {
//         let addr = SocketAddr::from_str("127.0.0.1:17017").unwrap();
//         Arbiter::spawn(
//             TcpStream::connect(&addr)
//                 .and_then(|stream| {
//                     let addr = ChatClient::create(|ctx| {
//                         let (r, w) = stream.split();
//                         ctx.add_stream(FramedRead::new(r, ClientChatCodec));
//                         ChatClient {
//                             framed: FramedWrite::new(w, ClientChatCodec, ctx),
//                         }
//                     });

//                     thread::spawn(move || loop {
//                         let mut cmd = String::new();
//                         if io::stdin().read_line(&mut cmd).is_err() {
//                             println!("error");
//                             return;
//                         }

//                         addr.do_send(ClientCommand(cmd));
//                     });

//                     ok(())
//                 })
//                 .map_err(|e| {
//                     println!("Can not connect to server: {}", e);
//                     process::exit(1)
//                 }),
//         );
//     })
// }

// struct ChatClient {
//     framed: FramedWrite<WriteHalf<TcpStream>, ClientChatCodec>,
// }

// #[derive(Message)]
// struct ClientCommand(String);

// impl Actor for ChatClient {
//     type Context = Context<Self>;

//     fn started(&mut self, ctx: &mut Context<Self>) {
//         self.hb(ctx)
//     }

//     fn stopping(&mut self, _: &mut Context<Self>) -> Running {
//         println!("Disconnected");

//         System::current().stop();

//         Running::Stop
//     }
// }

// impl ChatClient {
//     fn hb(&self, ctx: &mut Context<Self>) {
//         ctx.run_later(Duration::new(1, 0), |act, ctx| {
//             act.framed.write(RpcRequestC::Ping);
//             act.hb(ctx);
//         });
//     }
// }

// impl WriteHandler<io::Error> for ChatClient {}

// impl Handler<ClientCommand> for ChatClient {
//     type Result = ();

//     fn handle(&mut self, msg: ClientCommand, _: &mut Context<Self>) {
//         let m = msg.0.trim();

//         if m.starts_with('/') {
//             let v: Vec<&str> = m.splitn(2, ' ').collect();
//             match v[0] {
//                 "/get" => {
//                     self.framed.write(RpcRequestC::Get(UrlGetterMsg {
//                         limit: 10,
//                         anon: None,
//                         work: true,
//                         hours: None,
//                     }));
//                 }
//                 "/check" => {
//                     self.framed.write(RpcRequestC::Check(UrlGetterMsg {
//                         limit: 10,
//                         anon: None,
//                         work: true,
//                         hours: None,
//                     }));
//                 }
//                 _ => println!("!!! unknown command"),
//             }
//         }
//     }
// }

// impl StreamHandler<RpcResponseC, io::Error> for ChatClient {
//     fn handle(&mut self, msg: RpcResponseC, _: &mut Context<Self>) {
//         match msg {
//             RpcResponseC::Proxy(ref msg) => {
//                 println!("message: {:?}", msg);
//             }
//             _ => (),
//         }
//     }
// }
