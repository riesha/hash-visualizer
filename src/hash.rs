pub enum MagicNumbers
{
    Fnv1aPrime = 16777619,
    Fnv1aBias  = 2166136261,
}
// these are of course not really how you wanna do this but its just fine for what i wanted to do...
pub fn fnv1a(data: &[u8]) -> Vec<u8>
{
    let mut res = Vec::new();
    let mut out = MagicNumbers::Fnv1aBias as u32;
    for byte in data
    {
        out ^= *byte as u32;
        out = out.wrapping_mul(MagicNumbers::Fnv1aPrime as u32);
        res.extend_from_slice(&out.to_le_bytes());
    }
    res
}

pub fn fnv1(data: &[u8]) -> Vec<u8>
{
    let mut res = Vec::new();
    let mut out = MagicNumbers::Fnv1aBias as u32;
    for byte in data
    {
        out = out.wrapping_mul(MagicNumbers::Fnv1aPrime as u32);
        out ^= *byte as u32;
        res.extend_from_slice(&out.to_le_bytes());
    }
    res
}
