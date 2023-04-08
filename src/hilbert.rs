use std::mem;

#[derive(Copy, Clone)]
pub struct Coords
{
    pub x: u32,
    pub y: u32,
}
pub fn xy2index(dimension: u32, coords: Coords) -> u32
{
    let mut rx: u32;
    let mut ry: u32;
    let mut order_index = dimension / 2;
    let mut index = 0;
    while order_index > 0
    {
        rx = ((coords.x & order_index) > 0) as u32;
        ry = ((coords.y & order_index) > 0) as u32;
        index += order_index * order_index * ((3 * rx) ^ ry);
        rotate_quadrant(coords, rx, ry, dimension);
        order_index /= 2;
    }
    index
}
pub fn index2xy(dimension: u32, index: u32) -> Coords
{
    let mut point = Coords { x: 0, y: 0 };
    let mut rx: u32;
    let mut ry: u32;

    let mut order_index = 1u32;
    let mut quadrant = index;
    let rows = dimension;
    if index >= rows.pow(2)
    {
        panic!("Index {index} out of bounds!");
    }
    while order_index < rows
    {
        rx = 1 & (quadrant / 2);
        let dummy = quadrant ^ rx;
        ry = 1 & dummy;
        rotate_quadrant(point, rx, ry, order_index);
        point.x += order_index * rx;
        point.y += order_index * ry;
        quadrant /= 4;
        order_index *= 2;
    }

    point
}
fn rotate_quadrant(mut point: Coords, rx: u32, ry: u32, dimension: u32)
{
    if ry == 0
    {
        if rx == 1
        {
            point.x = dimension - 1 - point.x;
            point.y = dimension - 1 - point.y;
        }
        mem::swap(&mut point.x, &mut point.y);
    }
}
