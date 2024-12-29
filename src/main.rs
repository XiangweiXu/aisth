use aisth::lm::math::SquareMatrix;

fn main() {
    let m: SquareMatrix<4> = SquareMatrix::<4>::zero();

    println!("{}", m[0][0]);
}
