// (x1, y1), (x2, y2)
type Point = (i32, i32);
type Area = (Point, Point);

fn probe(area: &Area, mut x_vel: i32, mut y_vel: i32) -> bool {
    let ((x1, y1), (x2, y2)) = area;

    let mut x = 0;
    let mut y = 0;

    //     // sum of arithmetic progression: (2 * a1 + (n-1)*d) / 2 * n
    //     // where d = -1, n = y_vel + 1 for x, x_vel for y
    //     let mut x = (2 * x_vel - y_vel) / 2 * (y_vel + 1);
    //     let mut y = (2 * y_vel - x_vel) / 2 * (x_vel + 1);

    //     if x > *x2 || y < *y1 {
    //         // The probe appears to pass through the target area
    //         return false;
    //     }

    while x <= *x2 && y >= *y1 {
        if x >= *x1 && y <= *y2 {
            return true;
        }
        x += x_vel;
        y += y_vel;
        if x_vel != 0 {
            x_vel -= 1
        }
        y_vel -= 1
    }
    return false;
}

fn count_successful_probes(area: &Area) -> u64 {
    let ((_x1, y1), (x2, _y2)) = area;
    (*y1..=y1.abs())
        .flat_map(|y_vel| (1..=*x2).map(move |x_vel| probe(area, x_vel, y_vel)))
        .filter(|&x| x)
        .count() as u64
}

pub fn part_a(input: Option<Area>) -> u64 {
    let ((_x1, y1), (_x2, y2)) = input.unwrap_or(((25, -260), (67, -200)));
    (y1.min(y2) * (y1.min(y2) + 1) / 2) as u64
}

pub fn part_b(input: Option<Area>) -> u64 {
    count_successful_probes(&input.unwrap_or(((25, -260), (67, -200))))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_a_works() {
        let area = ((20, -10), (30, -5));
        let result = super::part_a(Some(area));
        assert_eq!(result, 45);
    }

    #[test]
    fn test_part_a() {
        assert_eq!(super::part_a(None), 33670);
    }

    #[test]
    fn test_part_b_works() {
        let area = ((20, -10), (30, -5));
        let result = super::part_b(Some(area));
        assert_eq!(result, 112);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(super::part_b(None), 4903);
    }
}
