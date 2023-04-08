#![feature(hasher_prefixfree_extras)]
#![feature(is_some_with)]
#![feature(int_log)]
use std::{fs::File, io::Read, ops::AddAssign};

use argh::FromArgs;
use bitvec::{prelude::Msb0, view::BitView};

use hash_visualizer::{
    hash::fnv1a,
    hilbert::{index2xy, Coords},
};

use image::{Rgb, RgbImage};
#[derive(FromArgs)]
/// hash visualizer
struct Args
{
    /// use hilbert curve algorithm instead of linear mapping
    #[argh(switch, short = 'h')]
    hilbert: bool,
    /// use a file instead of a string as the input
    #[argh(option, short = 'f')]
    file:    Option<String>,
    ///  unused if file flag is specified, string to use as input instead of a file, defaults to "no input given" if left blank
    #[argh(positional, default = "String::from(\"no input given\")")]
    input:   String,
}
fn main()
{
    let args: Args = argh::from_env();
    let data = match args.file.is_some()
    {
        true =>
        {
            let mut file = File::open(args.file.unwrap()).unwrap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
            buffer
        }
        false => args.input.into_bytes(),
    };

    let hash = fnv1a(&data);

    println!(
        "string entropy: {} hashed entropy: {}",
        shannon_entropy(&data),
        shannon_entropy(&hash)
    );

    let bits = hash.view_bits::<Msb0>();
    let size = (bits.len() as f64).sqrt();

    let size = match args.hilbert
    {
        true => size.ceil(),
        _ => size.floor(),
    } as u32;
    let dimension = match args.hilbert
    {
        true => 2u32.pow((size as f32 + 1.).log2().ceil() as u32),
        _ => size,
    };

    println!("dimension: {} bitcount: {}", dimension, bits.len(),);

    let mut img = RgbImage::new(dimension, dimension);
    let mut coords = (0u32, 0u32);
    let mut counter = 0;
    for (i, bit) in bits.iter().take((dimension.pow(2)) as _).enumerate()
    {
        if !args.hilbert
        {
            if coords == (dimension, dimension)
            {
                println!("coords: {:?}, dimension: {:?}", coords, dimension);
                break;
            }
            if coords.0 == dimension
            {
                coords.0 = 0;
                coords.1 = coords.1 + 1;
            }
        }

        let pos = match args.hilbert
        {
            true => index2xy(dimension, i as _),
            _ => Coords {
                x: coords.0,
                y: coords.1,
            },
        };

        if *bit == true
        {
            img.put_pixel(pos.x, pos.y, Rgb([255, 255, 255]));
        }
        else
        {
            img.put_pixel(pos.x, pos.y, Rgb([0, 0, 0]));
        }
        coords.0 = coords.0 + 1;
        counter.add_assign(1);
    }
    println!("written: {} bits of {}", counter, bits.len());
    let _ = img.save("output.png").unwrap();
}

pub fn shannon_entropy(data: &[u8]) -> f32
{
    let bytes = data.as_ref();
    let mut entropy = 0.0;
    let mut counts = [0; 256];

    for &b in bytes
    {
        counts[b as usize] += 1;
    }

    for &count in &counts
    {
        if count == 0
        {
            continue;
        }

        let p: f32 = (count as f32) / (bytes.len() as f32);
        entropy -= p * p.log(256.0);
    }

    entropy
}
