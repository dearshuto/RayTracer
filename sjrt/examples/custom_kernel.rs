use std::ops::{Add, Div, Mul};

#[derive(Debug, Default, Clone, Copy)]
struct MyVector([f32; 3]);

impl sjrt::IConstract<f32> for MyVector {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z])
    }
}

impl sjrt::INormalized for MyVector {
    fn normalized(&self) -> Self {
        let x = self.0[0];
        let y = self.0[1];
        let z = self.0[2];
        let length = (x * x + y * y + z * z).sqrt();
        Self([x / length, y / length, z / length])
    }
}

impl sjrt::IInnerProduct<f32> for MyVector {
    fn dot(&self, other: &Self) -> f32 {
        self.0
            .iter()
            .zip(other.0.iter())
            .map(|(lhs, rhs)| *lhs * *rhs)
            .sum()
    }
}

impl Mul<f32> for MyVector {
    type Output = MyVector;

    fn mul(self, rhs: f32) -> Self::Output {
        let x = self.0[0];
        let y = self.0[1];
        let z = self.0[2];
        MyVector([x * rhs, y * rhs, z * rhs])
    }
}

impl Add<MyVector> for MyVector {
    type Output = MyVector;

    fn add(self, rhs: MyVector) -> Self::Output {
        let lhs_x = self.0[0];
        let lhs_y = self.0[1];
        let lhs_z = self.0[2];

        let rhs_x = rhs.0[0];
        let rhs_y = rhs.0[1];
        let rhs_z = rhs.0[2];

        MyVector([lhs_x + rhs_x, lhs_y + rhs_y, lhs_z + rhs_z])
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct MyColor([f32; 3]);

impl Into<sjrt::Color> for MyColor {
    fn into(self) -> sjrt::Color {
        todo!()
    }
}

impl num::Zero for MyColor {
    fn zero() -> Self {
        Self::default()
    }

    fn is_zero(&self) -> bool {
        self.0.iter().all(|v| *v == 0.0)
    }
}

impl Add<MyColor> for MyColor {
    type Output = MyColor;

    fn add(self, rhs: MyColor) -> Self::Output {
        todo!()
    }
}

impl Div<f32> for MyColor {
    type Output = MyColor;

    fn div(self, rhs: f32) -> Self::Output {
        todo!()
    }
}

impl sjrt::IComponentMul for MyColor {
    fn multiply(&self, rhs: &Self) -> Self {
        todo!()
    }
}

struct MyHitParams;
impl sjrt::IHitParams<MyVector, MyColor> for MyHitParams {
    fn normal(&self) -> MyVector {
        todo!()
    }

    fn position(&self) -> MyVector {
        todo!()
    }

    fn emission(&self) -> MyColor {
        todo!()
    }

    fn albedo(&self) -> MyColor {
        todo!()
    }
}

#[derive(Clone, Copy)]
struct MyKernel;

impl sjrt::IKernel for MyKernel {
    type RondomEngine = sjrt::util::RandomEngine;
    type Point = MyVector;
    type Color = MyColor;
    type HitParams = MyHitParams;

    fn random_engine(&self) -> Self::RondomEngine {
        todo!()
    }

    fn new_point(&self, x: f32, y: f32, z: f32) -> Self::Point {
        todo!()
    }
}

struct MyScene(sjrt::util::RapierScene);
impl sjrt::ISceneStructure<MyHitParams, MyVector> for MyScene {
    fn cast(&self, from: &MyVector, to: &MyVector) -> Option<MyHitParams> {
        todo!()
    }
}

fn main() {
    let mut buffer = sjrt::util::ImageBuffer::new(640, 480);
    let pipeline = sjrt::PathTracerEx::new(MyKernel {});
    let scene = MyScene(sjrt::util::RapierScene::new());
    sjrt::Executor::default().execute(&mut buffer, [].into_iter(), scene, pipeline);
}
