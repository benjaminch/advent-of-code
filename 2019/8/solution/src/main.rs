use std::io::{self, Error, Read, Write};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let input_pixels: Vec<i32> = input
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    // Part 1
    let image: Image = build_image(&input_pixels.clone(), 25, 6);
    let (layer_having_fewest_0, _count_0): (usize, i32) =
        find_layer_having_fewest(0, &image).unwrap();
    let count_1_in_that_layer: i32 = count_in_layer(1, layer_having_fewest_0, &image);
    let count_2_in_that_layer: i32 = count_in_layer(2, layer_having_fewest_0, &image); // can be done via substraction since input seems to contain only 0, 1 and 2.
    writeln!(
        io::stdout(),
        "Layer with more 0: {} (1 count * 2 count in that layer =  {})",
        layer_having_fewest_0,
        count_1_in_that_layer * count_2_in_that_layer
    )?;

    // Part 2

    return Ok(());
}

fn count_in_layer(input: i32, layer: usize, image: &Image) -> i32 {
    let mut input_count: i32 = 0;
    let pixels_in_layer: usize = image.width * image.height;
    let start = layer * pixels_in_layer;
    let stop = start + pixels_in_layer;

    for p in start..stop {
        if image.pixels[p] == input {
            input_count += 1;
        }
    }

    return input_count;
}

fn find_layer_having_fewest(input: i32, image: &Image) -> Option<(usize, i32)> {
    let mut min_layer: Option<(usize, i32)> = None;
    let mut current_layer_input_count: i32;

    for current_layer in 0..image.layers_count {
        current_layer_input_count = count_in_layer(input, current_layer, image);
        if min_layer.is_none() || current_layer_input_count < min_layer.unwrap().1 {
            min_layer = Some((current_layer, current_layer_input_count));
        }
    }

    return min_layer;
}

fn build_image(input_pixels: &Vec<i32>, width: usize, height: usize) -> Image {
    return Image {
        pixels: input_pixels.clone(),
        width: width,
        height: height,
        layers_count: input_pixels.len() / (width * height),
    };
}

#[derive(Debug)]
struct Image {
    pixels: Vec<i32>,
    layers_count: usize,
    width: usize,
    height: usize,
}
