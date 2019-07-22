# Workshop 5

For this workshop, we will implement some logic around points in space.

We'll start with 2D points where coordinates are of type `i32`. Then we'll see
if we can make `Point2D` more generic.

## 1. Point comparison

As we build out types representing points in space, it will likely be useful to
run the supplied tests.

- Uncomment the first test `point_equality`
- Fix the compiler errors by deriving some traits on `Point2D`
  - Note that for the purposes of these exercises, `Point::default` is the origin

## 2. Point operations

What use is `Point2D` without the ability to do some operations with them?

- Implement the traits `Add` and `Sub` for `Point2D`
- Make sure that the following tests pass
  - `point_addition`
  - `point_subtraction`

## 3. Point construction

Frequently writing `Point2D::new` must be getting tiring. Let's impl a trait on
point that allows us to turn `(i32, i32)` into `Point2D`.

- Support constructing points with the syntax `(1, 2).into()`
  - Should also work with `Point2D::from((1, 2))`

## 4. Point ordering

We want to make sure that if we have a `Vec<Point2D>`, then we can sort it
based on the distance each point is from the origin.

- Impl `Point2D::dist_sq` which computes the square of the distance between 2
  `Point2D` instances.
- Make sure all tests are passing.

## 5. [ext] Generic Point2D

The aim of this extension is to update the `Point2D` type you've just created
so that it is generic over it's coordinates. This means you should be able to
have `Point2D<T>` where the coordinates are of type `T`. For example

- `Point<i32>`
- `Point<f32>`

### Notes on ordering your generic Point2D

For simplicity and brevity of this workshop, we suggest not implementing `Ord`
for your generic implementation of `Point2D`. This is because `f32` doesn't
implement the `Ord` trait, only `PartialOrd`, and sorting a `Vec<T>` requires
that `T` implements `Ord`.

If you feel up to it though, don't let us stop you! We'll be more than happy to
help out as you go.
