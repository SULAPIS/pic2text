#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use opencv::core::Point;
use opencv::core::Point2i;
use opencv::core::Scalar;
use opencv::core::Size2i;
use opencv::core::Vector;
use opencv::core::BORDER_CONSTANT;
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::imgproc::get_structuring_element;
use opencv::imgproc::morphology_default_border_value;
use opencv::imgproc::INTER_LINEAR;
use opencv::imgproc::LINE_8;
use opencv::prelude::*;

#[tauri::command]
fn cv_demo(p: &str) -> Vec<i32> {
    let path = "../pic/font.jpg";
    println!("{}", p);
    let mut img = imgcodecs::imread(&format!("../pic/{}", p), imgcodecs::IMREAD_COLOR).unwrap();
    // let mut img = imgcodecs::imread(path, imgcodecs::IMREAD_COLOR).unwrap();

    let mut font = img.clone();
    let mut rect_pic = img.clone();
    let mut cut_pic = img.clone();

    let mut img_gray = Mat::default();
    imgproc::cvt_color(&img, &mut img_gray, imgproc::COLOR_BGR2GRAY, 0).unwrap();
    img = img_gray.clone();

    imgcodecs::imwrite("../res/0.jpg", &img_gray, &Vector::new());
    imgproc::adaptive_threshold(&img, &mut img_gray, 255.0, 0, 1, 5, 10.0).unwrap();
    img = img_gray.clone();
    imgcodecs::imwrite("../res/1.jpg", &img_gray, &Vector::new());
    imgproc::erode(
        &img,
        &mut img_gray,
        &get_structuring_element(0, Size2i::new(3, 3), Point2i::new(-1, -1)).unwrap(),
        Point2i::new(-1, -1),
        1,
        BORDER_CONSTANT,
        morphology_default_border_value().unwrap(),
    );
    img = img_gray.clone();
    imgcodecs::imwrite("../res/2.jpg", &img_gray, &Vector::new());
    imgproc::dilate(
        &img,
        &mut img_gray,
        &get_structuring_element(0, Size2i::new(19, 19), Point2i::new(-1, -1)).unwrap(),
        Point2i::new(-1, -1),
        1,
        BORDER_CONSTANT,
        morphology_default_border_value().unwrap(),
    );
    imgcodecs::imwrite("../res/3.jpg", &img_gray, &Vector::new());

    img = img_gray.clone();
    let mut contours = opencv::types::VectorOfVectorOfPoint::new();
    let mut hierarchy = opencv::types::VectorOfVec4i::new();
    imgproc::find_contours(
        &img_gray,
        &mut contours,
        imgproc::RETR_EXTERNAL,
        imgproc::CHAIN_APPROX_SIMPLE,
        Point::default(),
    )
    .unwrap();

    imgproc::draw_contours(
        &mut font,
        &contours,
        -1,
        Scalar::new(0., 0., 255., 0.),
        5,
        LINE_8,
        &opencv::core::no_array(),
        100,
        Point::default(),
    );
    imgcodecs::imwrite("../res/4.jpg", &font, &Vector::new());

    let rect: Vec<opencv::core::Rect> = contours
        .iter()
        .map(|x| imgproc::bounding_rect(&x).unwrap())
        .collect();

    for x in rect.iter() {
        imgproc::rectangle(
            &mut rect_pic,
            *x,
            opencv::core::VecN([255., 0., 0., 0.]),
            5,
            LINE_8,
            0,
        );
    }
    imgcodecs::imwrite("../res/5.jpg", &rect_pic, &Vector::new());

    let mut indexs: Vec<i32> = Vec::new();
    for (i, r) in rect.iter().enumerate() {
        let cut = Mat::rowscols(
            &cut_pic,
            &opencv::core::Range::new(r.y, r.y + r.height).unwrap(),
            &opencv::core::Range::new(r.x, r.x + r.width).unwrap(),
        )
        .unwrap();
        let mut out = Mat::default();
        imgproc::resize(&cut, &mut out, Size2i::new(70, 70), 0.0, 0.0, INTER_LINEAR);
        // imgcodecs::imwrite(&format!("../pic/font{}.jpg", i), &out, &Vector::new());

        let mut max_index = -1;
        let mut max_res = 0.0;
        for i in 0..77 {
            let mut c =
                imgcodecs::imread(&format!("../match/font{}.jpg", i), imgcodecs::IMREAD_COLOR)
                    .unwrap();
            let zero = Mat::zeros(
                out.rows() - c.rows() + 1,
                out.cols() - c.cols() + 1,
                opencv::core::CV_32FC1,
            )
            .unwrap();
            let mut result = zero.to_mat().unwrap();

            let temp = imgproc::match_template(
                &out,
                &c,
                &mut result,
                imgproc::TM_CCOEFF_NORMED,
                &Mat::default(),
            );
            let res = result.data_typed::<f32>().unwrap()[0];
            if res > 0.6 && res > max_res {
                max_res = res;
                max_index = i;
            }
        }
        indexs.push(max_index);
        println!("{}: {}", max_index, max_res);
    }
    indexs
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![cv_demo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
