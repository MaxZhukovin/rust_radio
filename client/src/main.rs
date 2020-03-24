use rodio::{Decoder,Sink};
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader,Cursor, Read};

fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let device = rodio::default_output_device().unwrap();
    let  mut sink = Sink::new(&device);


    for stream in listener.incoming(){
        let stream = stream.unwrap();
    
        readStreamAndPlay(stream, &mut sink);
    }
}


fn readStreamAndPlay(mut stream: TcpStream, sink: &mut Sink){
    let mut streamBuffer = Vec::new();
    stream.read(&mut streamBuffer);

    let mut streamBuffer = Cursor::new(streamBuffer);
    
    sink.append(Decoder::new(streamBuffer).unwrap());
    sink.sleep_until_end();
}
