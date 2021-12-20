#[derive(Clone)]
enum Value {
    Literal(u8),
    Pair(Box<Value>, Box<Value>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(x) => write!(f, "{}", x),
            Self::Pair(l, r) => write!(f, "[{},{}]", l, r),
        }
    }
}

impl std::str::FromStr for Value {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.as_bytes()
            .into_iter()
            .fold(Vec::with_capacity(32), |mut stack, byte| {
                match byte {
                    // b'[' => stack.push(*byte),
                    b'0'..=b'9' => stack.push(Value::Literal(byte - b'0')),
                    b']' => {
                        let right = stack.pop().unwrap();
                        let left = stack.pop().unwrap();
                        stack.push(Value::Pair(Box::new(left), Box::new(right)));
                    }
                    _ => {}
                };
                stack
            })
            .pop()
            .unwrap())
    }
}

impl Value {
    pub fn magnitude(&self) -> u64 {
        match self {
            Self::Literal(v) => *v as u64,
            Self::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }

    pub fn add(self, other: Self) -> Self {
        Self::Pair(Box::new(self), Box::new(other)).reduce()
    }

    fn reduce(mut self) -> Self {
        loop {
            if let Some(f) = self.explode() {
                self = f;
            } else if let Some(f) = self.split() {
                self = f;
            } else {
                return self;
            }
        }
    }

    fn explode(&self) -> Option<Self> {
        if let Some((_, n, _)) = self.walk(0) {
            Some(n)
        } else {
            None
        }
    }

    fn walk(&self, depth: u8) -> Option<(Option<u8>, Self, Option<u8>)> {
        if let Self::Pair(a, b) = self {
            if depth > 3 {
                let (a, b) = (a.clone(), b.clone());
                if let (Self::Literal(a), Self::Literal(b)) = (*a, *b) {
                    Some((Some(a), Self::Literal(0), Some(b)))
                } else {
                    None
                }
            } else if let Some((l, new_a, r)) = a.walk(depth + 1) {
                if let Some(v) = r {
                    Some((
                        l,
                        Self::Pair(Box::new(new_a), Box::new(b.add_left(v))),
                        None,
                    ))
                } else {
                    Some((l, Self::Pair(Box::new(new_a), b.clone()), r))
                }
            } else if let Some((l, new_b, r)) = b.walk(depth + 1) {
                if let Some(v) = l {
                    Some((
                        None,
                        Self::Pair(Box::new(a.add_right(v)), Box::new(new_b)),
                        r,
                    ))
                } else {
                    Some((l, Self::Pair(a.clone(), Box::new(new_b)), r))
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn add_left(&self, value: u8) -> Self {
        match self {
            Self::Literal(v) => Self::Literal(v + value),
            Self::Pair(a, b) => Self::Pair(Box::new(a.add_left(value)), b.clone()),
        }
    }

    fn add_right(&self, value: u8) -> Self {
        match self {
            Self::Literal(v) => Self::Literal(v + value),
            Self::Pair(a, b) => Self::Pair(a.clone(), Box::new(b.add_right(value))),
        }
    }
    fn split(&self) -> Option<Self> {
        match self {
            Self::Literal(v) if *v > 9 => Some(Self::Pair(
                Box::new(Self::Literal(v / 2)),
                Box::new(Self::Literal(v - v / 2)),
            )),
            Self::Pair(left, right) => {
                if let Some(a) = left.split() {
                    Some(Self::Pair(Box::new(a), right.clone()))
                } else if let Some(b) = right.split() {
                    Some(Self::Pair(left.clone(), Box::new(b)))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

pub fn part_a(input: Option<&str>) -> u64 {
    input
        .unwrap_or(include_str!("../input.txt"))
        .lines()
        .map(|line| line.parse::<Value>().unwrap())
        .reduce(|acc, x| acc.add(x))
        .unwrap()
        .magnitude()
}

pub fn part_b(input: Option<&[u8]>) -> u64 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::Value;

    #[test]
    fn it_works() {
        let head = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
            .parse::<Value>()
            .unwrap();

        match &head {
            Value::Pair(l1, _) => match l1.as_ref() {
                Value::Pair(l2, _) => match l2.as_ref() {
                    Value::Literal(v) => assert_eq!(*v, 3),
                    _ => panic!(),
                },
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    #[test]
    fn test_magnitude() {
        let magnitude = "[[1,2],[[3,4],5]]".parse::<Value>().unwrap().magnitude();
        assert_eq!(magnitude, 143);
        let magnitude = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            .parse::<Value>()
            .unwrap()
            .magnitude();
        assert_eq!(magnitude, 3488);
    }

    #[test]
    fn test_explode() {
        let head = "[[[[[9,8],1],2],3],4]".parse::<Value>().unwrap();
        assert_eq!(format!("{}", head.reduce()), "[[[[0,9],2],3],4]");
    }

    #[test]
    fn test_reduce() {
        let head = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"
            .parse::<Value>()
            .unwrap();

        assert_eq!(
            format!("{}", head.reduce()),
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
        );
    }

    #[test]
    fn test_part_a_works() {
        let result = super::part_a(Some(include_str!("../example.txt")));
        assert_eq!(result, 4140);
    }

    #[test]
    fn test_part_a() {
        let result = super::part_a(None);
        assert_eq!(result, 4140);
    }
}