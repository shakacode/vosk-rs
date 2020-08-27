use vosk::VoskModel;

use audrey::read::Reader;
use audrey::sample::interpolate::{Converter, Linear, Sinc};
use audrey::sample::signal::{from_iter, Signal};

use std::fs::File;

const SAMPLE_RATE: u32 = 16000; 

pub fn main() {
    let audio_file_path = std::env::args().nth(1)
        .expect("Please specify an audio file to run STT on");

    let mut reader = audrey::open(audio_file_path).unwrap();
	let desc = reader.description();
	assert_eq!(1, desc.channel_count(),
        "The channel count is required to be one, at least for now");

    let model = VoskModel::new("./models/en-small");
    let mut sess = model.create_session(Default::default());
        
    let mut buff: Vec<i16> = Vec::with_capacity(1600);
    let mut samples_reader = reader.samples();

    loop {
        buff.clear();
        
        while let Some(s) = samples_reader.next() {
            buff.push(s.unwrap());
            if buff.len() >= 16000 {
                break;
            }
        }

        if buff.is_empty() {
            break;
        }

        println!("feed {}", buff.len());

        if model.feed(&mut sess, buff.as_slice()) {
            println!("{:?}", model.get_result(&mut sess));
        } else {
            // println!("{:?}", model.get_partial_result(&mut sess));
        }
    }

    println!("{:?}", model.get_final_result(sess));

    // let audio_buf :Vec<_> = if desc.sample_rate() == SAMPLE_RATE {
    //     .map(|s| s.unwrap()).collect()
    // } else {
    //     // We need to interpolate to the target sample rate
    //     let interpolator = Linear::new([0i16], [0]);
    //     let conv = Converter::from_hz_to_hz(
    //         from_iter(reader.samples::<i16>().map(|s| [s.unwrap()])),
    //         interpolator,
    //         desc.sample_rate() as f64,
    //         SAMPLE_RATE as f64);

    //     conv.until_exhausted().map(|v| v[0]).collect()
    // };


    

    // audio_buf

    
    // FILE *wavin;
    // char buf[3200];
    // int nread, final;

    // VoskModel *model = vosk_model_new("model");
    // VoskRecognizer *recognizer = vosk_recognizer_new(model, 16000.0);

    // wavin = fopen("test.wav", "rb");
    // fseek(wavin, 44, SEEK_SET);
    // while (!feof(wavin)) {
    //      nread = fread(buf, 1, sizeof(buf), wavin);
    //      final = vosk_recognizer_accept_waveform(recognizer, buf, nread);
    //      if (final) {
    //          printf("%s\n", vosk_recognizer_result(recognizer));
    //      } else {
    //          printf("%s\n", vosk_recognizer_partial_result(recognizer));
    //      }
    // }
    // printf("%s\n", vosk_recognizer_final_result(recognizer));

    // vosk_recognizer_free(recognizer);
    // vosk_model_free(model);
    // return 0;

}