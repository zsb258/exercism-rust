const TOP: isize = -1;
const BOT: isize = 1;
const LT: isize = -1;
const RT: isize = 1;

/// 8 adjacent directions, starting from top and going clockwise
const DIRECTIONS: [(isize, isize); 8] = [
    (TOP, 0),
    (TOP, RT),
    (0, RT),
    (BOT, RT),
    (BOT, 0),
    (BOT, LT),
    (0, LT),
    (TOP, LT),
];

const MINE: u8 = b'*' as u8;

fn get_adj_indices<'a>(
    r: &'a usize,
    c: &'a usize,
    max_r: &'a usize,
    max_c: &'a usize,
) -> impl Iterator<Item = (usize, usize)> + 'a {
    DIRECTIONS
        .to_owned()
        .to_vec()
        .into_iter()
        .filter_map(|(dr, dc)| {
            let x = *r as isize + dr;
            let y = *c as isize + dc;
            if x >= 0 && (x as usize) < *max_r && y >= 0 && (y as usize) < *max_c {
                Some((x as usize, y as usize))
            } else {
                None
            }
        })
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(i, &row)| {
            row.as_bytes()
                .iter()
                .enumerate()
                .map(|(j, &c)| {
                    if c == MINE {
                        MINE as char
                    } else {
                        match get_adj_indices(&i, &j, &minefield.len(), &row.len())
                            .filter(|(x, y)| minefield[*x].as_bytes()[*y] == MINE)
                            .count()
                        {
                            0 => ' ',
                            n => (b'0' + n as u8) as char,
                        }
                    }
                })
                .collect::<String>()
        })
        .collect()
}
