#![feature(hasher_prefixfree_extras)]
use std::hash::Hasher;

use bitvec::{prelude::Msb0, view::BitView};
use fnv::FnvHasher;
use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::{draw_text_mut, text_size};
use integer_sqrt::IntegerSquareRoot;
use ring::digest::{Context, Digest, SHA256};
use rusttype::{Font, Scale};

const PAYLOAD: &str = r#"hi every1 im new!!!!!!! *holds up spork* my name is katy but u can call me t3h PeNgU1N oF d00m!!!!!!!! lol…as u can see im very random!!!! thats why i came here, 2 meet random ppl like me ^_^… im 13 years old (im mature 4 my age tho!!) i like 2 watch invader zim w/ my girlfreind (im bi if u dont like it deal w/it) its our favorite tv show!!! bcuz its SOOOO random!!!! shes random 2 of course but i want 2 meet more random ppl =) like they say the more the merrier!!!! lol…neways i hope 2 make alot of freinds here so give me lots of commentses!!!!
DOOOOOMMMM!!!!!!!!!!!!!!!! <--- me bein random again ^_^ hehe…toodles!!!!!

love and waffles,

t3h PeNgU1N oF d00m."#;
//  img.put_pixel(x, y, Rgb([255, 255, 255]));
//  img.put_pixel(x, y, Rgb([0, 0, 0]));
fn hash_sha256(str: &str) -> Digest
{
    let mut context = Context::new(&SHA256);

    context.update(str.as_bytes());
    context.finish()
}
enum MagicNumbers
{
    FNV1A_PRIME = 16777619,
    FNV1A_BIAS  = 2166136261,
}

#[inline(always)]
fn fnv1a(data: &[u8]) -> Vec<u8>
{
    let mut res = Vec::new();
    let mut out = MagicNumbers::FNV1A_BIAS as u32;
    for byte in data
    {
        out ^= *byte as u32;
        out = out.wrapping_mul(MagicNumbers::FNV1A_PRIME as u32);
        res.extend_from_slice(&out.to_le_bytes());
    }
    res
}
#[inline(always)]
fn fnv1(data: &[u8]) -> Vec<u8>
{
    let mut res = Vec::new();
    let mut out = MagicNumbers::FNV1A_BIAS as u32;
    for byte in data
    {
        out = out.wrapping_mul(MagicNumbers::FNV1A_PRIME as u32);
        out ^= *byte as u32;
        res.extend_from_slice(&out.to_le_bytes());
    }
    res
}
fn main()
{
    let hash = fnv1a(PAYLOAD.as_bytes());
    println!(
        "string entropy: {} hashed entropy: {}",
        shannon_entropy(PAYLOAD.as_bytes()),
        shannon_entropy(&hash)
    );
    let bits = hash.view_bits::<Msb0>();
    let dimension = (bits.len() as f64).sqrt() as u32;

    println!("dimension: {} bitcount: {}", dimension, bits.len());

    let mut img = RgbImage::new(dimension, dimension);
    let mut coords = (0u32, 0u32);

    for bit in bits
    {
        if coords == (dimension - 1, dimension - 1)
        {
            break;
        }
        if coords.0 > dimension - 1
        {
            coords.0 = 0;
            coords.1 = coords.1 + 1;
        }

        if *bit == true
        {
            img.put_pixel(coords.0, coords.1, Rgb([255, 255, 255]));
        }
        else
        {
            img.put_pixel(coords.0, coords.1, Rgb([0, 0, 0]));
        }
        coords.0 = coords.0 + 1;
    }

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
