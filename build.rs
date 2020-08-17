use std::{env, fs};
use std::path::PathBuf;
use std::process::Command;


fn main() {
    println!("cargo:rerun-if-changed=cbits/vosk.h");
    println!("cargo:rerun-if-changed=build.rs");

    let bindings = bindgen::Builder::default()
        .generate_inline_functions(true)
        .derive_default(false)
        .header("cbits/vosk.h")
        .clang_arg("-I./resources/vosk-api/src/")
        .clang_arg("-I./resources/kaldi/src/")
        .clang_arg("-I./resources/openfst/src/include")
        .clang_arg("-std=c++14")
        .clang_arg("-x")
        .clang_arg("c++")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .opaque_type("std::.*")
        .opaque_type("kaldi::.*")
        .opaque_type("fst::.*")
        .opaque_type("KaldiRecognizer")
        .opaque_type("Model")
        .opaque_type("SpkModel")
        .whitelist_type("KaldiRecognizer")
        .whitelist_type("Model")
        .whitelist_type("SpkModel")
        .rustified_non_exhaustive_enum("*")
        .no_copy(".*")
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .warnings(false)
        .static_flag(true)
        .cpp(true)
        .include("resources/openfst/src/include")
        .file("resources/openfst/src/lib/compat.cc")
        .file("resources/openfst/src/lib/encode.cc")
        .file("resources/openfst/src/lib/fst-types.cc")
        .file("resources/openfst/src/lib/fst.cc")
        .file("resources/openfst/src/lib/mapped-file.cc")
        .file("resources/openfst/src/lib/properties.cc")
        .file("resources/openfst/src/lib/symbol-table-ops.cc")
        .file("resources/openfst/src/lib/symbol-table.cc")
        .file("resources/openfst/src/lib/util.cc")
        .file("resources/openfst/src/lib/weight.cc")
        .compile("libopenfst");

    cc::Build::new()
        .warnings(false)
        .static_flag(true)
        .cpp(true)
        .include("resources/vosk-api/src")
        .include("resources/kaldi/src/")
        .include("resources/openfst/src/include")
        .file("resources/vosk-api/src/kaldi_recognizer.cc")
        .file("resources/vosk-api/src/model.cc")
        .file("resources/vosk-api/src/spk_model.cc")
        .compile("libvosk");

    let out_dir = env::var("OUT_DIR").unwrap();
    let contents = fs::read_to_string("resources/kaldi/src/lat/kaldi-lattice.cc").expect("Something went wrong reading the file");

    let contents = contents.replace("printer.Print(&os, \"<unknown>\");", "printer.Print(os, \"<unknown>\");");

    let kaldi_lattice = format!("{}/kaldi-lattice.cc", out_dir);
    fs::write(&kaldi_lattice, contents).expect("Write file!");
    

    Command::new("sh")
        .arg("-c")
        .arg("resources/kaldi/src/base/get_version.sh")
        .status()
        .expect("Failed get_version.sh!");

    cc::Build::new()
        .warnings(false)
        .static_flag(true)
        .cpp(true)
        .define("HAVE_OPENBLAS", "true")
        
        .include("resources/openfst/src/include")
        .include("resources/kaldi/src")

        // base
        .file("resources/kaldi/src/base/io-funcs.cc")
        .file("resources/kaldi/src/base/kaldi-error.cc")
        .file("resources/kaldi/src/base/kaldi-math.cc")
        .file("resources/kaldi/src/base/kaldi-utils.cc")
        // .file("resources/kaldi/src/base/timer.cc")

        // matrix

        .file("resources/kaldi/src/matrix/kaldi-matrix.cc")
        .file("resources/kaldi/src/matrix/kaldi-vector.cc")
        .file("resources/kaldi/src/matrix/matrix-functions.cc")
        .file("resources/kaldi/src/matrix/optimization.cc")

        // cuda
        .file("resources/kaldi/src/cudamatrix/cu-matrix.cc")
        .file("resources/kaldi/src/cudamatrix/cu-allocator.cc")
        .file("resources/kaldi/src/cudamatrix/cu-common.cc")
        // .file("resources/kaldi/src/cudamatrix/cu-math.cc")
        
        // fstext
        .file("resources/kaldi/src/fstext/context-fst.cc")
        .file("resources/kaldi/src/fstext/grammar-context-fst.cc")
        .file("resources/kaldi/src/fstext/kaldi-fst-io.cc")
          .file("resources/kaldi/src/fstext/push-special.cc")

        // feat
        // .file("resources/kaldi/src/feat/feature-fbank.cc")
        // .file("resources/kaldi/src/feat/feature-functions.cc")
        .file("resources/kaldi/src/feat/feature-mfcc.cc")
        // .file("resources/kaldi/src/feat/feature-plp.cc")
        // .file("resources/kaldi/src/feat/feature-spectrogram.cc")
        // .file("resources/kaldi/src/feat/feature-window.cc")
        // .file("resources/kaldi/src/feat/mel-computations.cc")
        // .file("resources/kaldi/src/feat/online-feature.cc")
        // .file("resources/kaldi/src/feat/pitch-functions.cc")
        // .file("resources/kaldi/src/feat/resample.cc")
        // .file("resources/kaldi/src/feat/signal.cc")
        // .file("resources/kaldi/src/feat/wave-reader.cc")

        // lm
        // .file("resources/kaldi/src/lm/arpa-file-parser.cc")
        // .file("resources/kaldi/src/lm/arpa-lm-compiler.cc")
        .file("resources/kaldi/src/lm/const-arpa-lm.cc")
        // .file("resources/kaldi/src/lm/kaldi-rnnlm.cc")
        // .file("resources/kaldi/src/lm/mikolov-rnnlm-lib.cc")

        // rnnlm
        // .file("resources/kaldi/src/rnnlm/rnnlm-compute-state.cc")
        // .file("resources/kaldi/src/rnnlm/rnnlm-core-compute.cc")
        // .file("resources/kaldi/src/rnnlm/rnnlm-core-training.cc")
        // .file("resources/kaldi/src/rnnlm/rnnlm-embedding-training.cc")
        // .file("resources/kaldi/src/rnnlm/rnnlm-example-utils.cc")
        // .file("resources/kaldi/src/rnnlm/rnnlm-example.cc")
        // .file("resources/kaldi/src/rnnlm/rnnlm-lattice-rescoring.cc")
        // .file("resources/kaldi/src/rnnlm/rnnlm-training.cc")
        .file("resources/kaldi/src/rnnlm/rnnlm-utils.cc")
        // .file("resources/kaldi/src/rnnlm/sampler.cc")
        // .file("resources/kaldi/src/rnnlm/sampling-lm-estimate.cc")
        // .file("resources/kaldi/src/rnnlm/sampling-lm.cc")

        // decoder
        // .file("resources/kaldi/src/decoder/decodable-matrix.cc")
        // .file("resources/kaldi/src/decoder/decoder-wrappers.cc")
        // .file("resources/kaldi/src/decoder/faster-decoder.cc")
        // .file("resources/kaldi/src/decoder/grammar-fst.cc")
        .file("resources/kaldi/src/decoder/lattice-faster-decoder.cc")
        // .file("resources/kaldi/src/decoder/lattice-faster-online-decoder.cc")
        // .file("resources/kaldi/src/decoder/lattice-incremental-decoder.cc")
        // .file("resources/kaldi/src/decoder/lattice-incremental-online-decoder.cc")
        // .file("resources/kaldi/src/decoder/lattice-simple-decoder.cc")
        // .file("resources/kaldi/src/decoder/simple-decoder.cc")
        // .file("resources/kaldi/src/decoder/training-graph-compiler.cc")

        // nnet3
        .file("resources/kaldi/src/nnet3/am-nnet-simple.cc")
        // .file("resources/kaldi/src/nnet3/attention.cc")
        // .file("resources/kaldi/src/nnet3/convolution.cc")
        .file("resources/kaldi/src/nnet3/decodable-online-looped.cc")
        .file("resources/kaldi/src/nnet3/decodable-simple-looped.cc")
        // .file("resources/kaldi/src/nnet3/discriminative-supervision.cc")
        // .file("resources/kaldi/src/nnet3/discriminative-training.cc")
        // .file("resources/kaldi/src/nnet3/natural-gradient-online.cc")
        .file("resources/kaldi/src/nnet3/nnet-am-decodable-simple.cc")
        // .file("resources/kaldi/src/nnet3/nnet-analyze.cc")
        // .file("resources/kaldi/src/nnet3/nnet-attention-component.cc")
        // .file("resources/kaldi/src/nnet3/nnet-batch-compute.cc")
        // .file("resources/kaldi/src/nnet3/nnet-chain-diagnostics.cc")
        // .file("resources/kaldi/src/nnet3/nnet-chain-diagnostics2.cc")
        // .file("resources/kaldi/src/nnet3/nnet-chain-example.cc")
        // .file("resources/kaldi/src/nnet3/nnet-chain-training.cc")
        // .file("resources/kaldi/src/nnet3/nnet-chain-training2.cc")
        // .file("resources/kaldi/src/nnet3/nnet-combined-component.cc")
        // .file("resources/kaldi/src/nnet3/nnet-common.cc")
        // .file("resources/kaldi/src/nnet3/nnet-compile-looped.cc")
        // .file("resources/kaldi/src/nnet3/nnet-compile-utils.cc")
        // .file("resources/kaldi/src/nnet3/nnet-compile.cc")
        // .file("resources/kaldi/src/nnet3/nnet-component-itf.cc")
        // .file("resources/kaldi/src/nnet3/nnet-computation-graph.cc")
        .file("resources/kaldi/src/nnet3/nnet-computation.cc")
        .file("resources/kaldi/src/nnet3/nnet-compute.cc")
        // .file("resources/kaldi/src/nnet3/nnet-convolutional-component.cc")
        // .file("resources/kaldi/src/nnet3/nnet-descriptor.cc")
        // .file("resources/kaldi/src/nnet3/nnet-diagnostics.cc")
        // .file("resources/kaldi/src/nnet3/nnet-discriminative-diagnostics.cc")
        // .file("resources/kaldi/src/nnet3/nnet-discriminative-example.cc")
        // .file("resources/kaldi/src/nnet3/nnet-discriminative-training.cc")
        // .file("resources/kaldi/src/nnet3/nnet-example-utils.cc")
        // .file("resources/kaldi/src/nnet3/nnet-example.cc")
        // .file("resources/kaldi/src/nnet3/nnet-general-component.cc")
        // .file("resources/kaldi/src/nnet3/nnet-graph.cc")
        .file("resources/kaldi/src/nnet3/nnet-nnet.cc")
        // .file("resources/kaldi/src/nnet3/nnet-normalize-component.cc")
        // .file("resources/kaldi/src/nnet3/nnet-optimize-utils.cc")
        // .file("resources/kaldi/src/nnet3/nnet-optimize.cc")
        .file("resources/kaldi/src/nnet3/nnet-parse.cc")
        // .file("resources/kaldi/src/nnet3/nnet-simple-component.cc")
        // .file("resources/kaldi/src/nnet3/nnet-tdnn-component.cc")
        // .file("resources/kaldi/src/nnet3/nnet-training.cc")
        .file("resources/kaldi/src/nnet3/nnet-utils.cc")

        // lat
        // .file("resources/kaldi/src/lat/compose-lattice-pruned.cc")
        // .file("resources/kaldi/src/lat/confidence.cc")
        // .file("resources/kaldi/src/lat/determinize-lattice-pruned.cc")
        .file(&kaldi_lattice) // .file("resources/kaldi/src/lat/kaldi-lattice.cc")
        .file("resources/kaldi/src/lat/lattice-functions.cc")
        // .file("resources/kaldi/src/lat/minimize-lattice.cc")
        // .file("resources/kaldi/src/lat/phone-align-lattice.cc")
        // .file("resources/kaldi/src/lat/push-lattice.cc")
        .file("resources/kaldi/src/lat/sausages.cc")
        .file("resources/kaldi/src/lat/word-align-lattice-lexicon.cc")
        .file("resources/kaldi/src/lat/word-align-lattice.cc")

        // util
        // .file("resources/kaldi/src/util/kaldi-holder.cc")
        .file("resources/kaldi/src/util/kaldi-io.cc")
        // .file("resources/kaldi/src/util/kaldi-semaphore.cc")
        // .file("resources/kaldi/src/util/kaldi-table.cc")
        // .file("resources/kaldi/src/util/kaldi-thread.cc")
        .file("resources/kaldi/src/util/parse-options.cc")
        .file("resources/kaldi/src/util/simple-io-funcs.cc")
        // .file("resources/kaldi/src/util/simple-options.cc")
        .file("resources/kaldi/src/util/text-utils.cc")

        // online2
        .file("resources/kaldi/src/online2/online-endpoint.cc")
        .file("resources/kaldi/src/online2/online-feature-pipeline.cc")
        // .file("resources/kaldi/src/online2/online-gmm-decodable.cc")
        // .file("resources/kaldi/src/online2/online-gmm-decoding.cc")
        // .file("resources/kaldi/src/online2/online-ivector-feature.cc")
        // .file("resources/kaldi/src/online2/online-nnet2-decoding-threaded.cc")
        // .file("resources/kaldi/src/online2/online-nnet2-decoding.cc")
        // .file("resources/kaldi/src/online2/online-nnet2-feature-pipeline.cc")
        .file("resources/kaldi/src/online2/online-nnet3-decoding.cc")
        // .file("resources/kaldi/src/online2/online-nnet3-incremental-decoding.cc")
        // .file("resources/kaldi/src/online2/online-nnet3-wake-word-faster-decoder.cc")
        // .file("resources/kaldi/src/online2/online-speex-wrapper.cc")
        .file("resources/kaldi/src/online2/online-timing.cc")
        // .file("resources/kaldi/src/online2/onlinebin-util.cc")
        
        .compile("libkaldi");
}
