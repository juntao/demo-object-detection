use mediapipe_rs::postprocess::utils::draw_detection;
use mediapipe_rs::tasks::vision::ObjectDetectorBuilder;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model_data: &[u8] = include_bytes!("mobilenetv2_ssd_256_uint8.tflite");

    let args: Vec<String> = env::args().collect();
    let img_path: String = args[1];
    let output_path: String = args[2];

    let mut input_img = image::open(img_path)?;
    let detection_result = ObjectDetectorBuilder::new()
        .max_results(2) // set max result
        .build_from_buffer(model_data)? // create a object detector
        .detect(&input_img)?; // do inference and generate results

    // show formatted result message
    println!("{}", detection_result);

    // draw detection result to image
    draw_detection(&mut input_img, &detection_result);
    // save output image
    input_img.save(output_path)?;

    Ok(())
}
