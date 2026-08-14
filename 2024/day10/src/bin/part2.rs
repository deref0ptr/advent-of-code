use smallvec::SmallVec;

fn main() -> anyhow::Result<()> {
    day10::run::<SmallVec<[_; 4]>>()
}
