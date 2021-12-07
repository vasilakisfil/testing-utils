use std::net::{IpAddr, Ipv4Addr};

pub trait Randomize {
    fn random() -> Self
    where
        Self: Sized;

    fn rand_list0(range: u32) -> Vec<Self>
    where
        Self: Sized,
    {
        (0..=rand_num_from(1..=range))
            .map(|_| Self::random())
            .collect::<Vec<Self>>()
    }

    fn rand_list1(range: u32) -> Vec<Self>
    where
        Self: Sized,
    {
        (1..=rand_num_from(1..=range))
            .map(|_| Self::random())
            .collect::<Vec<Self>>()
    }

    fn list(size: u16) -> Vec<Self>
    where
        Self: Sized,
    {
        (1..=size).map(|_| Self::random()).collect::<Vec<Self>>()
    }
}

pub fn opt<T: Clone>(t: T) -> Option<T> {
    sample(&[Some(t), None])
}

impl crate::Randomize for bool {
    fn random() -> Self {
        use rand::Rng;

        let mut rng = rand::thread_rng();

        rng.gen_range(0..1000) > 500
    }
}

pub fn sample<T: Clone, const N: usize>(array: &[T; N]) -> T {
    use rand::Rng;

    let mut rng = rand::thread_rng();

    array[rng.gen_range(0..array.len())].clone()
}

pub fn sample_vec<T: Clone>(vector: &[T]) -> T {
    use rand::Rng;

    let mut rng = rand::thread_rng();

    vector.get(rng.gen_range(0..vector.len())).cloned().unwrap()
}

pub fn rand_str_of(size: usize) -> String {
    use rand::{distributions::Alphanumeric, Rng};

    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}

impl Randomize for String {
    fn random() -> Self {
        use rand::{distributions::Alphanumeric, Rng};

        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect()
    }
}

pub fn rand_num_from<T, R>(range: R) -> T
where
    T: rand::distributions::uniform::SampleUniform,
    R: rand::distributions::uniform::SampleRange<T>,
{
    use rand::Rng;

    let mut rng = rand::thread_rng();

    rng.gen_range(range)
}

impl Randomize for IpAddr {
    fn random() -> Self {
        IpAddr::V4(Randomize::random())
    }
}

impl Randomize for Ipv4Addr {
    fn random() -> Self {
        let ip_part = || rand_num_from(1..255);

        Ipv4Addr::new(ip_part(), ip_part(), ip_part(), ip_part())
    }
}
